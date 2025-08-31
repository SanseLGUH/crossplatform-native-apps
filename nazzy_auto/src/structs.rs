use serde::{Serialize, Deserialize};

pub const METADATA: &str = "https://raw.githubusercontent.com/SanseLGUH/my-cli-scripts/refs/heads/main/nz_mini_games_installator/mini_games/metadata.json";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MiniGame {
    pub path: String,
    pub description: String
}

pub type MiniGameCollection = std::collections::HashMap<String, MiniGame>;
