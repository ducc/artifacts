use std::time::Duration;

use reqwest::{Method, Request};
use serde_json::json;

use crate::{
    action_queue::ActionQueue,
    config::TaskName,
    error::Error,
    response::{self, APIResponse, InventorySlot},
};
pub struct Character {
    client: reqwest::Client,
    name: String,
    pub(super) queue: ActionQueue,
    token: String,
}

impl Character {
    pub fn new(client: reqwest::Client, name: String, token: String) -> Self {
        Self {
            queue: ActionQueue::new(client.clone(), name.clone()),
            client,
            name,
            token,
        }
    }

    pub async fn execution_loop(&self) -> Result<(), Error> {
        loop {
            let cooldown_remaining_seconds = match self.queue.pop_execute().await {
                Ok(Some(seconds)) => seconds,
                Ok(None) => {
                    continue;
                }
                Err(Error::Cooldown) => {
                    let name = &self.name;
                    tracing::warn!(character = %name, "on cooldown");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
                Err(e) => {
                    tracing::error!(error = ?e, "error executing action");
                    continue;
                }
            };

            tokio::time::sleep(Duration::from_secs(cooldown_remaining_seconds as u64)).await;
        }
    }

    fn build_request(
        &self,
        current_task: TaskName,
        method: Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<(String, Request), Error> {
        let mut req = self
            .client
            .request(
                method.clone(),
                format!("https://api.artifactsmmo.com/{}", path),
            )
            .header("Authorization", &self.token);

        let mut description = format!("{current_task:?} {method} {path}");

        if let Some(body) = body {
            req = req.json(&body);
            description = format!("{description} {}", serde_json::to_string(&body)?);
        }

        Ok((description, req.build()?))
    }

    async fn action(
        &self,
        current_task: TaskName,
        method: Method,
        action: &str,
        body: Option<serde_json::Value>,
    ) -> Result<(), Error> {
        let (description, req) = self.build_request(
            current_task,
            method,
            &format!("my/{}/action/{}", self.name, action),
            body,
        )?;

        self.queue.push(description, req).await;

        Ok(())
    }

    pub async fn move_to(&self, current_task: TaskName, x: i32, y: i32) -> Result<(), Error> {
        self.action(
            current_task,
            Method::POST,
            "move",
            Some(json!({
                "x": x,
                "y": y
            })),
        )
        .await
    }

    pub async fn fight(&self, current_task: TaskName) -> Result<(), Error> {
        self.action(current_task, Method::POST, "fight", None).await
    }

    pub async fn gathering(&self, current_task: TaskName) -> Result<(), Error> {
        self.action(current_task, Method::POST, "gathering", None)
            .await
    }

    pub async fn unequip(&self, current_task: TaskName, slot: &str) -> Result<(), Error> {
        self.action(
            current_task,
            Method::POST,
            "unequip",
            Some(json!({
                "slot": slot
            })),
        )
        .await
    }

    pub async fn crafting(&self, current_task: TaskName, code: &str) -> Result<(), Error> {
        self.action(
            current_task,
            Method::POST,
            "crafting",
            Some(json!({
                "code": code,
            })),
        )
        .await
    }

    pub async fn equip(&self, current_task: TaskName, code: &str, slot: &str) -> Result<(), Error> {
        self.action(
            current_task,
            Method::POST,
            "equip",
            Some(json!({
                "code": code,
                "slot": slot,
            })),
        )
        .await
    }

    pub async fn deposit(
        &self,
        current_task: TaskName,
        code: &str,
        quantity: i64,
    ) -> Result<(), Error> {
        self.action(
            current_task,
            Method::POST,
            "bank/deposit",
            Some(json!({
                "code": code,
                "quantity": quantity,
            })),
        )
        .await
    }

    pub async fn inventory(
        &self,
        current_task: TaskName,
    ) -> Result<(i64, Vec<InventorySlot>), Error> {
        let (_, req) = self.build_request(
            current_task,
            Method::GET,
            &format!("characters/{}", self.name),
            None,
        )?;
        let response: APIResponse<response::Character> =
            self.client.execute(req).await?.json().await?;

        let data = match response.data {
            Some(data) => data,
            None => return Err(Error::InvalidAPIResponse),
        };

        let mut slots: Vec<InventorySlot> = vec![];

        slots.push(InventorySlot {
            name: "inventory_slot1".into(),
            code: data.inventory_slot1,
            quantity: data.inventory_slot1_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot2".into(),
            code: data.inventory_slot2,
            quantity: data.inventory_slot2_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot3".into(),
            code: data.inventory_slot3,
            quantity: data.inventory_slot3_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot4".into(),
            code: data.inventory_slot4,
            quantity: data.inventory_slot4_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot5".into(),
            code: data.inventory_slot5,
            quantity: data.inventory_slot5_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot6".into(),
            code: data.inventory_slot6,
            quantity: data.inventory_slot6_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot7".into(),
            code: data.inventory_slot7,
            quantity: data.inventory_slot7_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot8".into(),
            code: data.inventory_slot8,
            quantity: data.inventory_slot8_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot9".into(),
            code: data.inventory_slot9,
            quantity: data.inventory_slot9_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot10".into(),
            code: data.inventory_slot10,
            quantity: data.inventory_slot10_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot11".into(),
            code: data.inventory_slot11,
            quantity: data.inventory_slot11_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot12".into(),
            code: data.inventory_slot12,
            quantity: data.inventory_slot12_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot13".into(),
            code: data.inventory_slot13,
            quantity: data.inventory_slot13_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot14".into(),
            code: data.inventory_slot14,
            quantity: data.inventory_slot14_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot15".into(),
            code: data.inventory_slot15,
            quantity: data.inventory_slot15_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot16".into(),
            code: data.inventory_slot16,
            quantity: data.inventory_slot16_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot17".into(),
            code: data.inventory_slot17,
            quantity: data.inventory_slot17_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot18".into(),
            code: data.inventory_slot18,
            quantity: data.inventory_slot18_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot19".into(),
            code: data.inventory_slot19,
            quantity: data.inventory_slot19_quantity,
        });

        slots.push(InventorySlot {
            name: "inventory_slot20".into(),
            code: data.inventory_slot20,
            quantity: data.inventory_slot20_quantity,
        });

        Ok((data.inventory_max_items, slots))
    }
}
