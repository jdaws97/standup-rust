mod config;


use std::fs;
use config::Config;

pub struct Standup {
    pub category: String,
    pub sentence: String,
    pub days_ago: u32,
    pub config: Config,
}

impl Standup {
    pub fn check_path(path: &str) {
        let path_bool: bool = path.is_empty();
        if !path_bool {
            let paths = fs::read_dir(path);
            for item in paths {
                println!("{}", item);
            }
        }
    }
}