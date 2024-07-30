use crate::{config::TaskName::KillYellowSlime, error::Error, Character};

pub async fn kill_yellow_slime(character: &Character) -> Result<(), Error> {
    // Move to the slime
    character.move_to(KillYellowSlime, 4, -1).await?;

    // Attack the slime
    character.fight(KillYellowSlime).await?;

    Ok(())
}
