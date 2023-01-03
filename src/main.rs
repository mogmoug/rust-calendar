#![crate_name = "rust_calendar"]
#![crate_type = "bin"]

mod calendar;
mod options;
use calendar::Calendar;
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
    match &cil.command{
        Some(options::Commands::Date { year, month }) => {
            Calendar::from_year_month((&year).as_ref().unwrap().parse().expect("Error year"),(&month).as_ref().unwrap().parse().expect("Error year"),cil.the_first_day_of_the_week);
        },
        Some(options::Commands::Now)=>{
            Calendar::get_print_calendar_now(cil.the_first_day_of_the_week);
        },
        Some(options::Commands::DebugInfo)=>{
            println!("{:?}",Calendar::get_calendar_now(cil.the_first_day_of_the_week));
        },
        None => {
            options::Cil::parse_from(vec!["rust-calendar","help"].iter());
        },
    }
}

