use reqwest::Request;
use tokio::sync::{mpsc, Mutex};

use crate::{
    error::Error,
    response::{APIResponse, ActionData},
};

pub struct ActionQueue {
    client: reqwest::Client,
    sender: mpsc::Sender<(String, Request)>,
    receiver: Mutex<mpsc::Receiver<(String, Request)>>,
    character_name: String,
}

impl ActionQueue {
    pub fn new(client: reqwest::Client, character_name: String) -> Self {
        let (tx, rx) = mpsc::channel(1);

        Self {
            client,
            sender: tx,
            receiver: Mutex::new(rx),
            character_name,
        }
    }

    pub async fn push(&self, description: impl Into<String>, task: Request) {
        self.sender
            .send((description.into(), task))
            .await
            .expect("mpsc send failed");
    }

    /// Pop and execute the next action in the queue, returning the remaining cooldown seconds
    pub async fn pop_execute(&self) -> Result<Option<i32>, Error> {
        let (description, request) = {
            let mut rx = self.receiver.lock().await;

            match rx.recv().await {
                Some(req) => req,
                None => {
                    return Ok(None);
                }
            }
        };

        tracing::info!("{} -> {}", self.character_name, description);

        let res = self
            .client
            .execute(request)
            .await?
            .json::<APIResponse<ActionData>>()
            .await?;

        match res {
            APIResponse {
                data: Some(data),
                error: None,
            } => Ok(Some(data.cooldown.remaining_seconds as i32)),
            APIResponse {
                data: None,
                error: Some(error),
            } => {
                match error.code {
                    490 => Ok(None), // character already at location
                    499 => Err(Error::Cooldown),
                    _ => Err(Error::APIError(error)),
                }
            }
            _ => Err(Error::InvalidAPIResponse),
        }
    }
}
