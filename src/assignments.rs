use super::keys;


#[derive(Clone, Debug, Serialize)]
pub struct Assignment {
    learner: String,
    completed: u32,
    pub units: Vec<keys::UsageKey>,
}


impl Assignment {
    pub fn new(learner: String, units: Vec<keys::UsageKey>) -> Assignment {
        Assignment {
            learner: learner,
            completed: 0,
            units: units,
        }
    }
    pub fn increment_completed(&mut self) {
        self.completed += 1;
    }

}
