use clap::*;
#[derive(Parser)]
#[command(name = "rust-calendar")]
#[command(author = "Mogmoug <mogmoug123@outlook.com>")]
#[command(version = "0.1.1")]
#[command(about = "A calendar command-line tool written in rust", long_about = None)]
pub struct Cil{
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Now get the calendar for this month.")]
    Now,
    #[command(about = "Usage: rust-calendar date [YEAR] [MONTH]\nGet the calendar for a month and year.")]
    Date{
        year: Option<String>,
        month: Option<String>
    },
    #[command(about = "Debug output information.")]
    DebugInfo
}