use crate::{config::TaskName::DepositInventory, error::Error, Character};

pub async fn deposit_inventory(character: &Character) -> Result<(), Error> {
    character.move_to(DepositInventory, 4, 1).await?; // walk to the bank

    let (_, inventory) = character.inventory(DepositInventory).await?;

    for slot in inventory {
        if slot.quantity > 0 {
            tracing::debug!("Depositing {} {}", slot.quantity, slot.code);
            character
                .deposit(DepositInventory, &slot.code, slot.quantity)
                .await?;
        }
    }

    Ok(())
}
