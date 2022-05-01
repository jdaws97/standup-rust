mod arguments;
mod config;
mod standup;

use arguments::StandupArgs;
use clap::Parser;
use config::configure;
use arguments::ConfigCommandList;
use standup::Standup;
use arguments::Entities;

fn main() {
    let args = StandupArgs::parse();
    let config = configure();
    let mut category: String = String::new();
    let mut days_ago: i32;
    let mut config_days: i32 = 0;
  
    match &args.entity {
        Entities::Add (add_command) => {
            category = add_command.category.to_string();
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
                    println!("{}", ConfigCommandList::Open);
                }
                ConfigCommandList::Days (days_command) => {
                    println!("{}", days_command.days)
                }
                ConfigCommandList::Path (path_command) => {
                    println!("{}", path_command.path)
                }
            }
            println!("{:?}", config_list);
        },
    }
    
    // let entities: &Entities = &args.entity;
    // println!("{:?}", entities);
    // let standup_self = Standup{ 
    //     category: args.entity.Add.category,
    //     sentence: args.entity.sentence,

    // }
    // let dir = Standup::check_path(&config.path.to_string());
    println!("{:?}", config.path);
    println!("{:?}", args.entity);
    // println!("{:?}", dir);
}
