use crate::{config::TaskName::CopperIngots, error::Error, Character};

pub async fn copper_ingots(character: &Character) -> Result<(), Error> {
    // Move to forest forge
    character.move_to(CopperIngots, 1, 5).await?;

    // Craft copper ingot
    character.crafting(CopperIngots, "copper").await?;

    Ok(())
}
