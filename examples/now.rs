use rust_calendar::calendar::{Calendar, WeekStartingFrom};

fn main() {
    Calendar::from_now(WeekStartingFrom::StartingFromSunday).print();
}