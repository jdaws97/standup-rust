mod arguments;
mod config;

use arguments::StandupArgs;
use clap::Parser;
use config::configure;

fn main() {
    let args = StandupArgs::parse();
    let config = configure();
    println!("{:?}", config.path);
    println!("{:?}", args.entity);
}
