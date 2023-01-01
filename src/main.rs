use std::env;

use chrono::prelude::*;
#[derive(Debug)]
pub(crate) struct Calendar {
    first_day: u32,
    num_of_days: u32,
    days: [[u32; 7]; 6],
}
impl Calendar {
    fn new(first_day: u32, num_of_days: u32) -> Calendar {
        Calendar {
            first_day,
            num_of_days,
            days: [[0; 7]; 6],
        }
    }
    fn init(&mut self) {
        let mut curday: u32 = self.first_day;
        let mut num_of_weeks: usize = 0;
        for d in 1..=self.num_of_days {
            if curday >= 7 {
                num_of_weeks += 1;
                curday = 0;
                self.days[num_of_weeks][curday as usize] = d;
                curday += 1;
            } else {
                self.days[num_of_weeks][curday as usize] = d;
                curday += 1;
            }
        }
    }
    fn print_calendar(&self) {
        println!("Sun\tMon\tTue\tWed\tThu\tFri\tSat");
        for w in self.days {
            for d in w {
                print!("{}\t", d);
            }
            println!()
        }
    }
    /// 用于计算一年中的一个月有几天
    /// year 这个月所在的年份
    /// month 这个月是几月，一月是1
    fn num_day_of_month(year: i32, month: u32) -> u32{
        //判断是否是闰年
        let is_rn = ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0);
        return match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31, //1，3，5，7，8，10，12是大月
            4 | 6 | 9 | 11 => 30,//4，6,9,11是小月
            2 => match is_rn {
                true => 29,//2月闰年29天，平年28天
                false => 28,
            },
            _ => 30,
        };
    }
    fn get_print_calendar_now() -> Calendar {
        let date = Local::now();
        let first_day = date.clone().with_day(1);
        let mut c: Calendar = Calendar::new(
            first_day.unwrap().weekday().num_days_from_sunday(),
            Self::num_day_of_month(date.year(),date.month()),
        );
        c.init();
        c.print_calendar();
        c
    }
    fn get_calendar_now() -> Calendar {
        let date = Local::now();
        let first_day = date.clone().with_day(1);
        let mut c: Calendar = Calendar::new(
            first_day.unwrap().weekday().num_days_from_sunday(),
            Self::num_day_of_month(date.year(),date.month()),
        );
        c.init();
        c
    }

}
fn print_help(){
    let usage = "A calendar command-line tool written in rust
Usage: rust-calendar [ARGUMENTS]
Arguments:
    now\t\t\tPrints the calendar for the current month
    debug_calendar\tDebug prints the calendar for the current month
    help\t\tShow this help";
    println!("{}",usage);
}
fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len()==1{
        Calendar::get_print_calendar_now();
    }
    
    if args.len()==2{
        match args[1].as_str() {
            "now" => {
                Calendar::get_print_calendar_now();
            }
            "debug_calendar" => {
                println!("{:#?}",Calendar::get_calendar_now());
            }
            "help" =>{
                print_help();
            },
            _ => {
                eprintln!("unknown arguments");
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn can_print(){
        let mut c: Calendar = Calendar::new(6, 29);
        c.init();
        c.print_calendar();    
    }
    #[test]
    fn print_calendar_now(){
        Calendar::get_print_calendar_now();
    }
}