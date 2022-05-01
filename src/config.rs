use std::fs;

use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub days: u32,
    pub path: String,
}
pub fn configure() -> Config {
    
    let default_path: String = String::from("/tmp");
    let config: Config = Config { days: 7, path: default_path, };

    if let Some(proj_dirs) = ProjectDirs::from(
        "app",
        "skyola",
        "standup",
    ) {
        let config_dir = proj_dirs.config_dir();

        let config_file = fs::read_to_string(config_dir.join("standup.toml"));

        let new_config = match config_file  {
            Ok(file) => toml::from_str(&file).unwrap(),
            Err(_) => config,
        };
        return new_config
    };

    config
}


