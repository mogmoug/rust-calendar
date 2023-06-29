use chrono::prelude::*;

pub enum WeekStartingFrom {
    StartingFromSunday = 0,
    StartingFromMonday = 1,
}

impl WeekStartingFrom {
    pub fn from_number(week_starting: i8) -> WeekStartingFrom {
        if week_starting == 0 {
            WeekStartingFrom::StartingFromSunday
        } else if week_starting == 1 {
            WeekStartingFrom::StartingFromMonday
        } else {
            panic!("WeekStartingFrom from_number() Unknown week_starting")
        }
    }
}

pub struct CalendarPage {
    year: i32,
    month: u32,
    first_day: u32,
    first_day_of_week: WeekStartingFrom,
    num_of_days: u32,
    days: [[u32; 7]; 6],
}

impl CalendarPage {
    pub fn new(year: i32, month: u32, first_day_of_week: WeekStartingFrom) -> CalendarPage {
        let mut new_one: CalendarPage = CalendarPage {
            year,
            month,
            first_day: 0,
            first_day_of_week,
            num_of_days: 0,
            days: [[0; 7]; 6],
        };
        //判断是否是闰年
        let is_rn =
            ((new_one.year % 4 == 0) && (new_one.year % 100 != 0)) || (new_one.year % 400 == 0);
        new_one.num_of_days = match new_one.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31, //1，3，5，7，8，10，12是大月
            4 | 6 | 9 | 11 => 30,              //4，6,9,11是小月
            2 => match is_rn {
                true => 29, //2月闰年29天，平年28天
                false => 28,
            },
            _ => 30,
        };
        new_one.first_day = match new_one.first_day_of_week {
            WeekStartingFrom::StartingFromSunday => NaiveDate::from_ymd_opt(year, month, 1)
                .expect("Unknown year or month")
                .weekday()
                .number_from_sunday(),
            WeekStartingFrom::StartingFromMonday => NaiveDate::from_ymd_opt(year, month, 1)
                .expect("Unknown year or month")
                .weekday()
                .number_from_monday(),
        } - 1;
        // 推算一个月份的第一天
        let mut current_day = new_one.first_day;
        let mut num_of_weeks: usize = 0;
        for d in 1..=new_one.num_of_days {
            // 如果到了下一个星期
            if current_day >= 7 {
                // 归位到第一天
                num_of_weeks += 1;
                current_day = 0;
                // 填充和自增
                new_one.days[num_of_weeks][current_day as usize] = d;
                current_day += 1;
            } else {
                // 否则直接填充和自增
                new_one.days[num_of_weeks][current_day as usize] = d;
                current_day += 1;
            }
        }
        new_one
    }
    pub fn print(&self) {
        match self.first_day_of_week {
            WeekStartingFrom::StartingFromSunday => println!("Sun\tMon\tTue\tWed\tThu\tFri\tSat"),
            WeekStartingFrom::StartingFromMonday => println!("Mon\tTue\tWed\tThu\tFri\tSat\tSun"),
        }
        for w in self.days {
            for d in w {
                if d != 0 {
                    print!("{}\t", d);
                } else {
                    print!(" \t");
                }
            }
            println!()
        }
    }
}

pub struct Calendar {}

impl Calendar {
    pub fn from_year_month(
        the_first_day_of_week: WeekStartingFrom,
        year: i32,
        month: u32,
    ) -> CalendarPage {
        CalendarPage::new(year, month, the_first_day_of_week)
    }
    pub fn from_now(the_first_day_of_week: WeekStartingFrom) -> CalendarPage {
        CalendarPage::new(
            Local::now().year(),
            Local::now().month(),
            the_first_day_of_week,
        )
    }
}
