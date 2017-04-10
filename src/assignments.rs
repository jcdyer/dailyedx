use super::keys;


#[derive(Debug, Serialize)]
pub struct Assignment {
    learner: String,
    completed: u32,
    units: Vec<keys::UsageKey>,
}


impl Assignment {
    pub fn new(learner: String, units: Vec<keys::UsageKey>) -> Assignment {
        Assignment {
            learner: learner,
            completed: 0,
            units: units,
        }
    }
}
