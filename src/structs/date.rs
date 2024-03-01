use std::fmt;
use std::fmt::Formatter;

pub struct Date {
    pub day: i16,
    pub month: i16,
    pub year: i16,
}

impl Date {
    #[allow(dead_code)]
    pub fn new(day: i16, month: i16, year: i16) -> Date {
        Date { day, month, year }
    }

    pub fn parse(date: &str) -> Date {
        let items = date
            .split('-')
            .into_iter()
            .map(|s| s.parse().expect("error in parsing date"))
            .collect::<Vec<i16>>();
        Date {
            day: items[0],
            month: items[1],
            year: items[2],
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}/{}", self.day, self.month, self.year)
    }
}

