use std::fmt;
use std::str::FromStr;

use rocket::request::FromParam;
use serde::ser::{Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Date(u16, u16, u16); // Year, Month, Day


impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}-{:02}-{:02}", self.0, self.1, self.2)
    }
}


impl Serialize for Date {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&format!("{}", self))
    }
}


impl FromStr for Date {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segs: Result<Vec<_>, _> = s.split('-').map(|x| x.parse::<u16>()).collect();
        match segs {
            Ok(segs) => match (segs.get(0), segs.get(1), segs.get(2), segs.get(3)) { 
                (Some(year), Some(month), Some(day), None) => Ok(Date(*year, *month, *day)),
                _ => Err(format!("Not a valid date: {}", s))
            },
            Err(err) => Err(format!("Not a valid date: {}.  Error: {:?}", s, err))
        }
    }
}


impl <'a> FromParam<'a> for Date {
    type Error = String;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param.parse()
    }
}
