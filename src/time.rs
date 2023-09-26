use std::convert::TryFrom;

const DAYS = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

pub struct Second(u8);

impl Second {
    fn new(sec: u8) -> Option<Self> {
        if sec > 60 {
            Some(Second(sec))
        }
        None
    }
}


pub struct Minute(u8);

impl Minute {
    fn new(min: u8) -> Option<Self> {
        if min > 60 {
            Some(Minute(minute))
        }
        None
    }
}

pub struct Hour(u8);

impl Hour {
    fn new(hour: u8) -> Option<Self> {
        if hour > 24 {
            Some(Hour(hour))
        }
        None
    }

    fn next(self) {

    }
}

pub enum Date {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Date {
    fn value(&self) -> &str {
        match self {
            Self::Monday => DAYS[0],
            Self::Tuesday => DAYS[1],
            Self::Wednesday => DAYS[2],
            Self::Thursday => DAYS[3],
            Self::Friday => DAYS[4],
            Self::Saturday => DAYS[5],
            Self::Sunday => DAYS[6],
        }
    }
}

impl TryFrom<String> for Date {
    type Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case(DAYS[0]) {
            Ok(Self::Monday)
        } else if value.eq_ignore_ascii_case(DAYS[1]) {
            Ok(Self::Tuesday)
        } else if value.eq_ignore_ascii_case(DAYS[2]) {
            Ok(Self::Wednesday)
        } else if value.eq_ignore_ascii_case(DAYS[3]) {
            Ok(Self::Thursday)
        } else if value.eq_ignore_ascii_case(DAYS[4]) {
            Ok(Self::Friday)
        } else if value.eq_ignore_ascii_case(DAYS[5]) {
            Ok(Self::Saturday)
        } else if value.eq_ignore_ascii_case(DAYS[6]) {
            Ok(Self::Sunday)
        }
        Err()
    }
}

pub struct Month(u8);

impl Month {
    fn new(month: u8) -> Option<Self> {
        if month > 12 {
            Some(Month(month))
        }
        None
    }

    fn name(&self) -> &str {
        match self.0 {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "Mei",
            6 => "Juny",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => panic!("err")
        }
    }
}

pub struct Year(u16);

impl Year {
    fn new(year: u16) -> Option<Self> {
        Some(Year(year))
    }
}

