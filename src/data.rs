use std::str::FromStr;

use super::keys::{CourseKey, UsageKey};


fn parseable<T: FromStr>(entries: Vec<&str>) -> Vec<T> {
    entries.iter()
        .map(|x| x.parse::<T>())
        .filter_map(|x| x.ok())
        .collect()
}


pub fn get_courses(learner: &str) -> Vec<CourseKey> {
    match learner {
        "sandy@edx.org" => parseable(vec![
            "course-v1:EduCauseX+TeamBasedLearning+2017T1",
        ]),
        "cdyer@edx.org" => parseable(vec![
            "course-v1:EduCauseX+TeamBasedLearning+2017T1",
            "course-v1:LongfellowX+PaulReveresRide+1775T1",
        ]),
        "greg@edx.org" => parseable(vec![
            "course-v1:LongfellowX+PaulReveresRide+1775T1",
        ]),
        "nimisha@edx.org" => parseable(vec![
            "course-v1:EduCauseX+TeamBasedLearning+2017T1",
            "course-v1:LongfellowY+PaulReveresRide+1775T1",
        ]),
        _ => vec![],
    }
}


pub fn get_blocks(course: &CourseKey) -> Vec<UsageKey> {
    match course {
        &CourseKey { ref org, .. } if org == &"LongfellowX" => parseable(vec![
            "block-v1:LongfellowX+PaulReveresRide+1775T1+type@vertical+block@signal",
            "block-v1:LongfellowX+PaulReveresRide+1775T1+type@vertical+block@church",
        ]),
        &CourseKey { ref org, .. } if org == &"EduCauseX" => parseable(vec![
            "block-v1:EduCauseX+TeamBasedLearning+2017T1+type@vertical+block@think-pair-share",
            "block-v1:EduCauseX+TeamBasedLearning+2017T1+type@vertical+block@flipped-class",
        ]),
        _ => vec![],
    }
}
