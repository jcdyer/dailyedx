
use super::keys::{CourseKey, UsageKey, BlockType};


pub fn get_courses(learner: &str) -> Vec<CourseKey> {
    match learner {
        "sandy@edx.org" => vec![
            "course-v1:EduCauseX+TeamBasedLearning+2017T1".parse().unwrap(),
        ],
        "cdyer@edx.org" => vec![
            "course-v1:EduCauseX+TeamBasedLearning+2017T1".parse().unwrap(),
            "course-v1:LongfellowX+PaulReveresRide+1775T1".parse().unwrap(),
        ],
        _ => vec![
            "course-v1:LongfellowX+PaulReveresRide+1775T1".parse().unwrap(),
        ],
    }
}


pub fn get_blocks(course: &CourseKey) -> Vec<UsageKey> {
    match course {
        &CourseKey { ref org, .. } if org == &"LongfellowX" => {
            vec![
                "block-v1:EduCauseX+TeamBasedLearning+2017T1+type@vertical+block@signal".parse().unwrap(),
                "block-v1:EduCauseX+TeamBasedLearning+2017T1+type@vertical+block@church".parse().unwrap(),
            ]
        },
        &CourseKey { ref org, .. } if org == &"EduCauseX" => {
            vec![
                UsageKey::new(
                    course.clone(),
                    BlockType::Vertical,
                    "think-pair-share".to_string()
                ),
                UsageKey::new(
                    course.clone(),
                    BlockType::Vertical,
                    "flippedclass".to_string()
                ),
            ]
        },
        _ => vec![],
    }
}
