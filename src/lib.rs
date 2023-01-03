#![crate_name = "rust_calendar"]
#![crate_type = "lib"]

pub mod calendar;
pub use calendar::Calendar;
mod test;
pub mod options;