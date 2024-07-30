use std::{sync::Arc, time::Duration};

use character::Character;
use config::{CharacterConfig, Condition, Config, TaskName};

mod action_queue;
mod character;
mod conditions;
mod config;
mod error;
mod response;
mod tasks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();
    let token = std::env::var("ARTIFACTS_TOKEN")?;

    let client = reqwest::Client::new();

    let config_str = tokio::fs::read_to_string("config.toml").await?;
    let config: Config = toml::from_str(&config_str)?;

    for character_config in config.characters.into_iter() {
        let CharacterConfig { name, tasks } = character_config;

        let character = Arc::new(Character::new(client.clone(), name, token.clone()));

        let character_clone = character.clone();

        tokio::spawn(async move {
            let character = character_clone;

            loop {
                for task_config in &tasks {
                    if let Some(condition) = &task_config.condition {
                        match condition {
                            Condition::FullInventory => {
                                let is_full_inventory = match conditions::full_inventory(
                                    &character,
                                    task_config.name.clone(),
                                )
                                .await
                                {
                                    Ok(full) => full,
                                    Err(e) => {
                                        tracing::error!(error = ?e, "full inventory condition failed");
                                        continue;
                                    }
                                };

                                // skip the task if the inventory is not full
                                if !is_full_inventory {
                                    continue;
                                }
                            }
                        }
                    }

                    match task_config.name {
                        TaskName::MineCopper => tasks::mine_copper(&character).await.unwrap(),
                        TaskName::MineIron => tasks::mine_iron(&character).await.unwrap(),
                        TaskName::CopperIngots => tasks::copper_ingots(&character).await.unwrap(),
                        TaskName::KillChickens => tasks::kill_chickens(&character).await.unwrap(),
                        TaskName::DepositInventory => {
                            if let Err(e) = tasks::deposit_inventory(&character).await {
                                tracing::error!(error = ?e, "depositing inventory failed");
                                continue;
                            }
                        }
                        TaskName::KillYellowSlime => {
                            tasks::kill_yellow_slime(&character).await.unwrap()
                        }
                        TaskName::KillCows => tasks::kill_cows(&character).await.unwrap(),
                    }
                }
            }
        });

        tokio::spawn(async move {
            character
                .execution_loop()
                .await
                .expect("execution loop failed");
        });
    }

    // sleep forever
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
