use std::{fs::File, sync::OnceLock};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub lines_count: u32,
    pub start_radius: f32,
    pub max_iterations: u32,
    pub force_factor: f32,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        let file = File::open("config.yaml").unwrap();
        serde_yaml::from_reader(file).unwrap()
    })
}
