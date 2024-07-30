use crate::{character::Character, config::TaskName, error::Error};

// True if the character has a full inventory
pub async fn full_inventory(character: &Character, current_task: TaskName) -> Result<bool, Error> {
    let (max_items, inventory) = character.inventory(current_task).await?;

    if inventory.iter().map(|slot| slot.quantity).sum::<i64>() < max_items {
        tracing::debug!("Inventory is not full");
        return Ok(false);
    }

    Ok(true)
}
