#![crate_name = "rust_calendar"]
#![crate_type = "bin"]

mod calendar;
mod options;
use calendar::{Calendar, WeekStartingFrom};
use clap::Parser;

fn _print_help(){
    let usage = "A calendar command-line tool written in rust
Usage: rust-calendar [ARGUMENTS]
Arguments:
    now\t\t\tPrints the calendar for the current month
    debug_calendar\tDebug prints the calendar for the current month
    help\t\tShow this help
    year_month\t\tPrint a calendar for a month and year,Usage:rust-calendar year_month YEAR MONTH";
    println!("{}",usage);
}
fn main() {
    let cil = options::Cil::parse();
    //看看用户输入的都可以干什么
    match &cil.command{
        Some(options::Commands::Date { year, month }) => {
            let year = year.as_ref().unwrap().parse().expect("Error year");
            let month = month.as_ref().unwrap().parse().expect("Error month");
            Calendar::from_year_month(WeekStartingFrom::from_number(cil.the_first_day_of_the_week),year, month).print();
        },
        Some(options::Commands::Now)=>{
            Calendar::from_now(WeekStartingFrom::from_number(cil.the_first_day_of_the_week)).print();
        },
        None => {
            options::Cil::parse_from(vec!["rust-calendar","help"].iter());
        },
    }
}

