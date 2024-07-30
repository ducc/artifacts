use crate::{config::TaskName::KillChickens, error::Error, Character};

pub async fn kill_chickens(character: &Character) -> Result<(), Error> {
    // Move to the chickens
    character.move_to(KillChickens, 0, 1).await?;

    // Attack the chickens
    character.fight(KillChickens).await?;

    Ok(())
}
