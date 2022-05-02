use clap:: {
    Args,
    Parser,
    Subcommand
};

use strum_macros::Display;

#[derive(Debug, Parser, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct StandupArgs{
    #[clap(subcommand)]
    pub entity: Entities,

}

#[derive(Debug, Subcommand, Clone)]
pub enum Entities {
    /// Add a sentence to your notes
    Add(AddCommand),

    /// Open your notes
    Open(OpenCommand),

    /// Open the config
    Config(ConfigCommand),

}

#[derive(Debug, Args, Clone)]
pub struct AddCommand {
    #[clap(subcommand)]
    /// Category for the note
    pub category: Category,

    /// Sentence to be added to your standup notes
    pub sentence: String,
    
}

#[derive(Debug, Args, Clone)]
pub struct OpenCommand {
    /// Day paramter used for opening a certain days notes
    pub daysago: i32,
}

#[derive(Display, Debug, Subcommand, Clone)]
pub enum Category {
    /// DONE category is for completed tasks
    #[strum()]
    Done,

    /// IN-PROGRESS is for tasks in the works
    #[strum()]
    InProgress,

    /// BLOCKER is a task which can't move forward
    #[strum()]
    Blocker,

    /// NOTES are general notes for the day
    #[strum()]
    Notes,

}

#[derive(Debug, Args, Clone)]
pub struct ConfigCommand {

    #[clap(subcommand)]
    pub change_config: ConfigCommandList,

}

#[derive(Display, Debug, Subcommand, Clone)]
pub enum ConfigCommandList {
    /// Open the config file
    #[strum()]
    Open,

    /// Config days
    Days(DaysCommand),

    /// Change path in the config
    Path(PathCommand),
}

#[derive(Debug, Args, Clone)]
pub struct DaysCommand {
    /// Adjust how many days to keep notes
    #[clap(default_value_t = 7)]
    pub days: i32,

}

#[derive(Debug, Args, Clone)]
pub struct PathCommand {
    /// Give the new path
    pub path: String,
}
