#![cfg(test)]
mod tests{
    #[test]
    fn test_build(){
        println!("OK");
    }
    #[test]
    fn test_print_calendar(){
        use crate::calendar::Calendar;
        let c = Calendar::get_calendar_now();
        c.print_calendar();
    }
    #[test]
    fn can_print(){
        use crate::calendar::Calendar;
        let mut c: Calendar = Calendar::new(6, 29);
        c.init();
        c.print_calendar();    
    }
    #[test]
    fn print_calendar_now(){
        use crate::calendar::Calendar;
        Calendar::get_print_calendar_now();
    }
}