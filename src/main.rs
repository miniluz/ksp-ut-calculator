mod time;
use time::{HumanTime, UT, UTOffset};

fn main() {
    dbg!(UT::new( HumanTime {
        year: 7,
        day: 165,
        hour: 3,
        minute: 16,
        second: 37
    }).to_ut());
}
