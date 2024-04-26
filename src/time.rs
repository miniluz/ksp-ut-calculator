use std::ops::Add;
    
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default)]
pub struct HumanTime {
    pub year: i64,
    pub day: i64,
    pub hour: i64,
    pub minute: i64,
    pub second: i64
}

impl ToString for HumanTime {
    fn to_string(&self) -> String {
        format!(
            "Year {}, Day {}, Hour {}, Minute {}, Second {}",
            self.year,
            self.day,
            self.hour,
            self.minute,
            self.second
        )
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct UT(i64);

impl UT {
    pub fn new(time: HumanTime) -> Self {
        let year = time.year - 1;
        let day = time.day - 1;
        let hour = time.hour;
        let minute = time.minute;
        let second = time.second;

        let days = year * 426 + day;
        let hours = days * 6 + hour;
        let minutes = hours * 60 + minute;
        let seconds = minutes * 60 + second;

        Self(seconds.into())
    }

    pub fn to_ut(&self) -> i64 {
        self.0
    }

    pub fn to_human_time(&self) -> HumanTime {
        let rem = self.to_ut();
        let second = rem % 60;
        let rem = rem / 60;
        let minute = rem % 60;
        let rem = rem / 60;
        let hour = rem % 6;
        let rem = rem / 6;
        let day = rem % 426 + 1;
        let rem = rem / 426;
        let year = rem + 1;
        HumanTime {
            year, day, hour, minute, second
        }
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct UTOffset(i64);

impl UTOffset {
    pub fn new(time: HumanTime) -> Self {
        let year = time.year;
        let day = time.day;
        let hour = time.hour;
        let minute = time.minute;
        let second = time.second;

        let days = year * 426 + day;
        let hours = days * 6 + hour;
        let minutes = hours * 60 + minute;
        let seconds = minutes * 60 + second;

        Self(seconds.into())
    }

    pub fn to_ut(&self) -> i64 {
        self.0
    }

    pub fn to_human_time(&self) -> HumanTime {
        let rem = self.to_ut();
        let second = rem % 60;
        let rem = rem / 60;
        let minute = rem % 60;
        let rem = rem / 60;
        let hour = rem % 6;
        let rem = rem / 6;
        let day = rem % 426;
        let rem = rem / 426;
        let year = rem;
        HumanTime {
            year, day, hour, minute, second
        }
    }
}

impl Add<UTOffset> for UT {
    type Output = UT;

    fn add(self, rhs: UTOffset) -> Self::Output {
        UT(self.0 + rhs.0)
    }
}

impl Add<UTOffset> for UTOffset {
    type Output = UTOffset;

    fn add(self, rhs: UTOffset) -> Self::Output {
        UTOffset(self.0 + rhs.0)
    }
}