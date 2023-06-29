#![cfg(test)]
mod tests{
    #[test]
    fn test_build(){
        println!("OK");
    }
    #[test]
    fn test_print_calendar(){
        use crate::calendar::Calendar;
        Calendar::from_year_month(crate::calendar::WeekStartingFrom::StartingFromSunday,2023, 6).print();
    }
    #[test]
    fn print_calendar_now(){
        use crate::calendar::Calendar;
        Calendar::from_now(crate::calendar::WeekStartingFrom::StartingFromSunday).print();
    }
}