use std::fs;
use std::fs::File;
use std::path::Path;

use directories::ProjectDirs;
use serde::Deserialize;
use serde::Serialize;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Default, Serialize, Clone)]
pub struct Config {
    pub days: u32,
    pub path: String,
}
pub fn configure() -> Config {
    
    let mut default_path: String = String::from("/tmp");
    let config: Config = Config { days: 7, path: default_path, };

    if let Some(proj_dirs) = ProjectDirs::from(
        "app",
        "skyola",
        "standup",
    ) {
        let config_dir = proj_dirs.config_dir();
        
        let config_file_path = config_dir.join("standup_config.json");

        let config_file = match File::open(Path::new(&config_file_path)) {
            Ok(config_file) => config_file,
            Err(_) => return config
        };

        let reader = BufReader::new(config_file);

        let config_content: Config = serde_json::from_reader(reader).unwrap();

        // let new_config = match config_file  {
        //     Ok(file) => config,
        //     Err(_) => config,
        // };
        return config_content
    };

    return config
}


