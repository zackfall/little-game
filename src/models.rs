use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct WorldData {
    world_id: usize,
    world_name: String,
    world_description: String,
    // TODO: WorldConfig struct
    world_config: String,
    // TODO: WorldMap struct
    world_map: String,
    created_at: DateTime<Utc>,
    last_login: DateTime<Utc>,
    playen: Player,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    name: String,
    strengths: Vec<String>,
    weaknesses: Vec<String>,
}
