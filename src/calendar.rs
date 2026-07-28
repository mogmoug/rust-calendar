use chrono::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum WeekStartingFrom {
    StartingFromSunday = 0,
    StartingFromMonday = 1,
}

impl WeekStartingFrom {
    #[must_use]
    pub fn from_number(week_starting: i8) -> Self {
        match week_starting {
            0 => Self::StartingFromSunday,
            1 => Self::StartingFromMonday,
            _ => panic!("WeekStartingFrom from_number() Unknown week_starting: {week_starting}"),
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
    #[must_use]
    pub fn new(year: i32, month: u32, first_day_of_week: WeekStartingFrom) -> Self {
        // 判断是否是闰年
        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        
        let num_of_days = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31, // 1，3，5，7，8，10，12 是大月
            4 | 6 | 9 | 11 => 30,              // 4，6, 9, 11 是小月
            2 => if is_leap { 29 } else { 28 }, // 2 月闰年 29 天，平年 28 天
            _ => 30,
        };

        let first_day = match first_day_of_week {
            WeekStartingFrom::StartingFromSunday => {
                NaiveDate::from_ymd_opt(year, month, 1)
                    .expect("Invalid year or month")
                    .weekday()
                    .number_from_sunday()
            }
            WeekStartingFrom::StartingFromMonday => {
                NaiveDate::from_ymd_opt(year, month, 1)
                    .expect("Invalid year or month")
                    .weekday()
                    .number_from_monday()
            }
        } - 1;

        // 推算一个月份的第一天
        let mut days = [[0; 7]; 6];
        let mut current_day = first_day;
        let mut num_of_weeks: usize = 0;
        
        for d in 1..=num_of_days {
            // 如果到了下一个星期
            if current_day >= 7 {
                // 归位到第一天
                num_of_weeks += 1;
                current_day = 0;
                // 填充和自增
                days[num_of_weeks][current_day as usize] = d;
                current_day += 1;
            } else {
                // 否则直接填充和自增
                days[num_of_weeks][current_day as usize] = d;
                current_day += 1;
            }
        }

        Self {
            year,
            month,
            first_day,
            first_day_of_week,
            num_of_days,
            days,
        }
    }

    pub fn print(&self) {
        match self.first_day_of_week {
            WeekStartingFrom::StartingFromSunday => {
                println!("Sun\tMon\tTue\tWed\tThu\tFri\tSat");
            }
            WeekStartingFrom::StartingFromMonday => {
                println!("Mon\tTue\tWed\tThu\tFri\tSat\tSun");
            }
        }
        
        for week in &self.days {
            for day in week {
                if *day != 0 {
                    print!("{day}\t");
                } else {
                    print!(" \t");
                }
            }
            println!();
        }
    }
}

pub struct Calendar;

impl Calendar {
    #[must_use]
    pub fn from_year_month(the_first_day_of_week: WeekStartingFrom, year: i32, month: u32) -> CalendarPage {
        CalendarPage::new(year, month, the_first_day_of_week)
    }

    #[must_use]
    pub fn from_now(the_first_day_of_week: WeekStartingFrom) -> CalendarPage {
        let now = Local::now();
        CalendarPage::new(now.year(), now.month(), the_first_day_of_week)
    }
}
