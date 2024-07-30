use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub characters: Vec<CharacterConfig>,
}

#[derive(Deserialize)]
pub struct CharacterConfig {
    pub name: String,
    pub tasks: Vec<TaskConfig>,
}

#[derive(Deserialize)]
pub struct TaskConfig {
    pub name: TaskName,
    pub condition: Option<Condition>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum TaskName {
    CopperIngots,
    KillChickens,
    DepositInventory,
    KillYellowSlime,
    KillCows,
    MineCopper,
    MineIron,
}

#[derive(Deserialize, Debug)]
pub enum Condition {
    FullInventory,
}
