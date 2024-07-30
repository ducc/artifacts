use crate::{config::TaskName::KillCows, error::Error, Character};

pub async fn kill_cows(character: &Character) -> Result<(), Error> {
    // Move to the cows
    character.move_to(KillCows, 0, 2).await?;

    // Attack the cows
    character.fight(KillCows).await?;

    Ok(())
}
