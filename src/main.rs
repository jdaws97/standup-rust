mod arguments;
mod config;
mod standup;

use arguments::StandupArgs;
use clap::Parser;
use config::configure;
use arguments::ConfigCommandList;
use standup::Standup;
use arguments::Entities;
use std::process::Command;
use std::path::Path;
use std::env::current_dir;


fn main() {
    let args = StandupArgs::parse();
    let config = configure();
    let mut category: String = String::new();
    let mut sentence: String = String::new();
    let mut days_ago: i32 = 0;
    let mut config_args: String = String::new();
    let mut config_days: i32 = 0;
    let mut path: String = String::new();
  
    match &args.entity {
        Entities::Add (add_command) => {
            category = add_command.category.to_string();
            sentence = add_command.sentence.to_string();
            println!("Category: {}", category);
        },
        Entities::Open (open_command) => {
            days_ago = open_command.daysago;
            println!("Days Ago: {}", days_ago);
        },
        Entities::Config (config_command) => {
            let config_list = config_command.change_config.clone();
            match &config_list {
                ConfigCommandList::Open => {
                    config_args = ConfigCommandList::Open.to_string();
                    println!("{}", ConfigCommandList::Open);
                }
                ConfigCommandList::Days (days_command) => {
                    config_days = days_command.days;
                    println!("{}", days_command.days);
                }
                ConfigCommandList::Path (path_command) => {
                    path = path_command.path.clone();
                    println!("{}", path_command.path)
                }
            }
            println!("{:?}", config_list);
        },
    }
    
    let mut standup = Standup { category: category, sentence: sentence, days_ago: days_ago, config: config };
    
    let standup_dir = Standup::check_path(&mut standup, Path::new(&path));

    let config = configure();

    Standup::create_config(&mut standup, &config.path);


    if !config_args.is_empty() {
        let config_dir = current_dir().unwrap();
        Standup::check_path(&mut standup, Path::new(&config_dir));
        Command::new("vim")
                    .arg("standup_config.json")
                    .status()
                    .expect("Command failed");
    }
    Standup::check_standup();
    println!("{:?}", standup_dir);
}
