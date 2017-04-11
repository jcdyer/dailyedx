use std::fmt;
use std::str::FromStr;

use serde::ser::{Serialize, Serializer};
// use serde::de::Deserialize;
use rocket::request::FromParam;

use keys::BlockType::*;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CourseKey {
    pub org: String,
    pub course: String,
    pub run: String,
}

impl CourseKey {
    pub fn new(org: String, course: String, run: String) -> CourseKey {
        CourseKey {
            org: org,
            course: course,
            run: run,
        }
    }

    fn short_fmt(&self) -> String {
       format!("{}+{}+{}", self.org, self.course, self.run)
    }
}

impl fmt::Display for CourseKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "course-v1:{}", self.short_fmt())
    }
}

impl Serialize for CourseKey {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&format!("{}", self))
    }
}

impl FromStr for CourseKey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = "no".to_string();
        if let Some((prefix, s0)) = s.find(":").map(|i| s.split_at(i)) {
            match prefix {
                "course-v1" => {
                    let s0 = &s0[1..];
                    let parts: Vec<&str> = s0.split("+").collect();
                    if parts.len() == 3 {
                        let (org, course, run) = (parts[0], parts[1], parts[2]);
                        Ok(CourseKey::new(org.to_string(), course.to_string(), run.to_string()))
                    } else {
                        Err(err)
                    }
                },
                _ => Err(format!("Not a valid key type: {}", prefix)),
            }
        } else {
            Err(err)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BlockType {
    Sequential,
    Vertical,
    OtherBlock(String),
}

impl fmt::Display for BlockType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let slug = match self {
            &Sequential => "sequential".to_string(),
            &Vertical => "vertical".to_string(),
            &OtherBlock(ref x) => x.to_string(),
        };
        write!(f, "{}", slug)
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsageKey {
    course_key: CourseKey,
    block_type: BlockType,
    block_id: String,
}

impl UsageKey {
    pub fn new(course_key: CourseKey, block_type: BlockType, block_id: String) -> UsageKey {
        UsageKey {
            course_key: course_key,
            block_type: block_type,
            block_id: block_id,
        }
    }
}

impl fmt::Display for UsageKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "block-v1:{}+type@{}+block@{}", self.course_key.short_fmt(), self.block_type, self.block_id)
    }
}


impl Serialize for UsageKey {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&format!("{}", self))
    }
}

fn at_split<'a>(s: &'a str, category: &'a str) -> Option<&'a str> {
    let mut iter = s.splitn(3, "@");
    match (iter.next(), iter.next(), iter.next()) {
        (Some(cat), Some(second), None) if cat == category => Some(second),
        _ => None,
    }
}

impl FromStr for UsageKey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = "no".to_string();
        if let Some((prefix, s0)) = s.find(":").map(|i| s.split_at(i)) {
            match prefix {
                "block-v1" => {
                    let s0 = &s0[1..];
                    let parts: Vec<&str> = s0.split("+").collect();
                    if parts.len() == 5 {
                        let (org, course, run) = (parts[0], parts[1], parts[2]);
                        let (blocktype, blockid) = (parts[3], parts[4]);
                        let blocktype = at_split(blocktype, "type")
                            .ok_or(format!("Bad type: {}", blocktype))?;
                        let blockid = at_split(blockid, "block")
                            .ok_or(format!("Bad blockid: {}", blockid))?;
                        let course_key = CourseKey::new(
                            org.to_string(),
                            course.to_string(),
                            run.to_string()
                        );
                        let blocktype = match blocktype {
                            "vertical" => Vertical,
                            "sequential" => Sequential,
                            _ => OtherBlock(blocktype.to_string()),
                        };
                        Ok(UsageKey::new(course_key, blocktype, blockid.to_string()))
                    } else {
                        Err(err)
                    }
                },
                _ => Err(format!("Not a valid key type: {}", prefix)),
            }
        } else {
            Err(err)
        }
    }
}

impl <'a> FromParam<'a> for UsageKey {
    type Error = String;
    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn course_key_display() {
        let key = CourseKey {
            org: "BerkeleyX".to_string(),
            course: "BC49".to_string(),
            run: "2017T1".to_string(),
        };
        assert_eq!(&format!("{}", key), "course-v1:BerkeleyX+BC49+2017T1");
    }

    #[test]
    fn block_type_display() {
        let block = BlockType::Sequential;
        assert_eq!(&format!("{}", block), "sequential");
        let block = BlockType::Vertical;
        assert_eq!(&format!("{}", block), "vertical");
        let block = BlockType::OtherBlock("problem".to_string());
        assert_eq!(&format!("{}", block), "problem");
    }

    #[test]
    fn usage_key_display() {
        let course = CourseKey::new(
            "BerkeleyX".to_string(),
            "BC49".to_string(),
            "2017T1".to_string(),
        );
        let usage_key = UsageKey::new(
            course,
            Vertical,
            "23084afedd342d80293f8e39b90c".to_string()
        );
        assert_eq!(
            &format!("{}", usage_key),
            "block-v1:BerkeleyX+BC49+2017T1+type@vertical+block@23084afedd342d80293f8e39b90c"
        );
    }

    /*
    #[test]
    fn block_type_from_str() {
        assert_eq!(BlockType::from("vertical"), Vertical);
        assert_eq!(BlockType::from("sequential"), Sequential);
        assert_eq!(BlockType::from("unit"), OtherBlock("unit".to_string()));
        assert_eq!("unit".parse::<BlockType>().unwrap(), OtherBlock("unit".to_string()));
    }
    */

    #[test]
    fn parse_course_key() {
        let ok_key: Result<CourseKey, String> = "course-v1:org+course+run".parse();
        assert_eq!(
            ok_key,
            Ok(CourseKey::new("org".to_string(), "course".to_string(), "run".to_string()))
        );
    }

    #[test]
    fn parse_usage_key() {
        let ok_key: Result<UsageKey, String> = "block-v1:org+course+run+type@vertical+block@block1".parse();
        assert_eq!(
            ok_key,
            Ok(UsageKey::new("course-v1:org+course+run".parse().unwrap(), Vertical, "block1".to_string()))
        );
    }

    #[test]
    fn round_trip_course_key() {
        let key = "course-v1:BerkeleyX+BC49+2017T1".to_string();
        let parsed_key: CourseKey = key.parse().unwrap();
        assert_eq!(
            key, format!("{}", parsed_key)
        );
    }

    #[test]
    fn round_trip_usage_key() {
        let key = "block-v1:LongfellowX+PaulReveresRide+1775T1+type@vertical+block@signal".to_string();
        let parsed_key: UsageKey = key.parse().unwrap();
        assert_eq!(
            key, format!("{}", parsed_key)
        );
    }
}
