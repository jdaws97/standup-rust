mod arguments;
mod config;
mod standup;

use arguments::StandupArgs;
use clap::Parser;
use config::configure;
use standup::Standup;

fn main() {
    let args = StandupArgs::parse();
    let config = configure();
    let entity = &args.entity;
    // let standup_self = Standup{ 
    //     category: args.entity.Add.category,
    //     sentence: args.entity.sentence,

    // }
    // let dir = Standup::check_path(&config.path.to_string());
    println!("{:?}", config.path);
    println!("{:?}", args.entity);
    // println!("{:?}", dir);
}
