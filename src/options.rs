use clap::*;
#[derive(Parser)]
#[command(name = "rust-calendar")]
#[command(author = "Mogmoug <mogmoug123@outlook.com>")]
#[command(version = "0.1.1")]
#[command(about = "A calendar command-line tool written in rust", long_about = None)]
pub struct Cil{
    #[command(subcommand)]
    pub command: Option<Commands>,
    /// The first day of the week.Sunday(0) or Monday(1).
    #[arg(short,long,default_value_t = 0)]
    pub the_first_day_of_the_week: i8,
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