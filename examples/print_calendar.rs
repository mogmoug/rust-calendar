use rust_calendar::calendar::*;
fn main() {
    const STARTINGSUNDAY: rust_calendar::calendar::WeekStartingFrom = WeekStartingFrom::StartingFromSunday;
    for i in 1..12{
        Calendar::from_year_month(STARTINGSUNDAY, 2023, i).print();
    }
}
