use rust_calendar::calendar::*;
fn main(){
    let c = Calendar::from_year_month(2023, 2,0);
    c.print_calendar();
}