use crate::{config::TaskName::MineIron, error::Error, Character};

pub async fn mine_iron(character: &Character) -> Result<(), Error> {
    // Move to the iron
    character.move_to(MineIron, 1, 7).await?;

    // Gather 6 iron
    for _ in 0..6 {
        character.gathering(MineIron).await?;
    }

    Ok(())
}
