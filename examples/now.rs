use rust_calendar::calendar::Calendar;
fn main(){
    Calendar::from_now(rust_calendar::calendar::WeekStartingFrom::StartingFromSunday).print();
}