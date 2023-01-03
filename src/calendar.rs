use chrono::prelude::*;
#[derive(Debug)]
pub(crate) struct Calendar {
    first_day: u32,
    num_of_days: u32,
    days: [[u32; 7]; 6],
}
#[allow(dead_code)]
impl Calendar {
    pub fn new(first_day: u32, num_of_days: u32) -> Calendar {
        Calendar {
            first_day,
            num_of_days,
            days: [[0; 7]; 6],
        }
    }
    pub fn init(&mut self) {
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
    pub fn print_calendar(&self) {
        println!("Sun\tMon\tTue\tWed\tThu\tFri\tSat");
        for w in self.days {
            for d in w {
                if d != 0{
                    print!("{}\t", d);
                }else {
                    print!(" \t");
                }
            }
            println!()
        }
    }
    /// 用于计算一年中的一个月有几天
    /// year 这个月所在的年份
    /// month 这个月是几月，一月是1
    pub fn num_day_of_month(year: i32, month: u32) -> u32{
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
    pub fn get_print_calendar_now() -> Calendar {
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
    pub fn get_calendar_now() -> Calendar {
        let date = Local::now();
        let first_day = date.clone().with_day(1);
        let mut c: Calendar = Calendar::new(
            first_day.unwrap().weekday().num_days_from_sunday(),
            Self::num_day_of_month(date.year(),date.month()),
        );
        c.init();
        c
    }
    pub fn from_year_month(year: i32,month: i32) -> Calendar{
        let mut date = Local::now();
        date = match date.with_year(year){
            Some(dt) => {
                dt
            },
            None => panic!("crate rust-calendar,fn from_year_month,error year"),
        };
        date = match date.with_month(month as u32){
            Some(dt) => {
                dt
            },
            None => panic!("crate rust-calendar,fn from_year_month,error month")
        };
        let first_day = date.clone().with_day(1);
        let mut c: Calendar = Calendar::new(
            first_day.unwrap().weekday().num_days_from_sunday(),
            Self::num_day_of_month(date.year(),date.month()),
        );
        c.init();
        c
    }
}
