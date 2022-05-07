extern crate serde_json;

use crate::config::Config;

use std::fs;
use std::env::set_current_dir;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use directories::ProjectDirs;
use std::time::SystemTime;
use chrono::prelude::*;

#[derive(Default)]
pub struct Standup {
    pub category: String,
    pub sentence: String,
    pub days_ago: i32,
    pub config: Config,
}

impl Standup {

    pub fn check_path(&mut self, path: &Path) -> bool {
        if Path::new(&path).exists() {
            match set_current_dir(Path::new(&path)) {
                Ok(_) => return true,
                Err(_) => return false,
            };
        };
        return false
    }

    pub fn create_config(&mut self, config_path: &str) {

        if let Some(proj_dirs) = ProjectDirs::from(
            "app",
            "skyola",
            "standup",
        ) {
            let config_dir = proj_dirs.config_dir();
            if !&config_dir.exists() {
                fs::create_dir_all(config_dir).unwrap();
            };
            self.check_path(&config_dir);
            if !Path::new(&config_dir.join("standup_config.json")).exists() {
                let file = match File::create("standup_config.json") {
                    Ok(file) => file,
                    Err(_) => panic!("Couldn't create config file"),
                };
                serde_json::to_writer(&file, &self.config).expect("Couldn't write to config file");

            }
        }
    }

    pub fn check_standup(&mut self) {
        let date_time = Local::now();
        let current_date = date_time.month().to_string() + "-" + &date_time.day().to_string() + "-" + &date_time.year().to_string();
        println!("{:?}", current_date);
        let config: Config = self.config.clone();
        self.check_path(Path::new(&config.path));

    }


    pub fn create_standup(&mut self) {
        

    }

}
