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
    let standup = Standup::check_path(&config.path.to_string());
    println!("{:?}", config.path);
    println!("{:?}", args.entity);
}
