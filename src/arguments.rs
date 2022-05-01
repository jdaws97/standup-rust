use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct StandupArgs{
    #[clap(subcommand)]
    pub entity: Entities,

}

#[derive(Debug, Subcommand)]
pub enum Entities {
    /// Add a sentence to your notes
    Add(AddCommand),

    /// Open your notes
    Open(OpenCommand),

    /// Open the config
    Config(ConfigCommand),

}

#[derive(Debug, Args)]
pub struct AddCommand {
    #[clap(subcommand)]
    /// Category for the note
    pub category: Category,

    /// Sentence to be added to your standup notes
    pub sentence: String,
    
}

#[derive(Debug, Args)]
pub struct OpenCommand {
    /// Day paramter used for opening a certain days notes
    pub daysago: i32,
}

#[derive(Debug, Subcommand)]
pub enum Category {
    /// DONE category is for completed tasks
    Done,

    /// IN-PROGRESS is for tasks in the works
    InProgress,

    /// BLOCKER is a task which can't move forward
    Blocker,

    /// NOTES are general notes for the day
    Notes,

}

#[derive(Debug, Args)]
pub struct ConfigCommand {

    #[clap(subcommand)]
    pub change_config: ConfigCommandList,

}

#[derive(Debug, Subcommand)]
pub enum ConfigCommandList {
    /// Open the config file
    Open,

    /// Config days
    Days(DaysCommand),

    /// Change path in the config
    Path(PathCommand),
}

#[derive(Debug, Args)]
pub struct DaysCommand {
    /// Adjust how many days to keep notes
    #[clap(default_value_t = 7)]
    pub days: u32,

}

#[derive(Debug, Args)]
pub struct PathCommand {
    /// Give the new path
    pub path: String,
}
