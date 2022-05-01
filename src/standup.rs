
use crate::config::Config;

use std::env::set_current_dir;
use std::path::Path;

pub struct Standup {
    pub category: String,
    pub sentence: String,
    pub days_ago: u32,
    pub config: Config,
}

impl Standup {
    pub fn check_path(&mut self, path: &str) -> bool {
        let path_bool: bool = path.is_empty();
        if !path_bool {
            if Path::new(path).exists() {
                match set_current_dir(Path::new(path)) {
                    Ok(_) => return true,
                    Err(_) => return false,
                };
            };
        };
        return false
    }
}