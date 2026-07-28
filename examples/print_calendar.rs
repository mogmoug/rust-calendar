use rust_calendar::calendar::{Calendar, WeekStartingFrom};

fn main() {
    const STARTING_SUNDAY: WeekStartingFrom = WeekStartingFrom::StartingFromSunday;
    for i in 1..=12 {
        Calendar::from_year_month(STARTING_SUNDAY, 2023, i).print();
    }
}
