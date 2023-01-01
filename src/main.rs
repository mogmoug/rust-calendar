pub(crate) struct Calendar {
    first_day: u8,
    num_of_days: i32,
    days: [[i32; 7]; 6],
}
impl Calendar {
    fn new(first_day: u8, num_of_days: i32) -> Calendar {
        Calendar {
            first_day,
            num_of_days,
            days: [[0; 7]; 6],
        }
    }
    fn init(&mut self) {
        let mut curday: u8 = self.first_day;
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
    fn print_calendar(self) {
        println!("Sun\tMon\tTue\tWed\tThu\tFri\tSat");
        for w in self.days {
            for d in w {
                print!("{}\t", d);
            }
            println!()
        }
    }
}
fn main() {
    let mut c: Calendar = Calendar::new(6, 29);
    c.init();
    c.print_calendar();
}
