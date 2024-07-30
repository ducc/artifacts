use crate::{config::TaskName::MineCopper, error::Error, Character};

pub async fn mine_copper(character: &Character) -> Result<(), Error> {
    // Move to the copper
    character.move_to(MineCopper, 2, 0).await?;

    // Gather 6 copper
    for _ in 0..6 {
        character.gathering(MineCopper).await?;
    }

    Ok(())
}
