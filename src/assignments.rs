use super::keys;
use super::date::Date;


#[derive(Clone, Debug, Serialize)]
pub struct Assignment {
    learner: String,
    completed: u32,
    date: Date,
    pub units: Vec<keys::UsageKey>,
}


impl Assignment {
    pub fn new(learner: String, dt: Date, units: Vec<keys::UsageKey>) -> Assignment {
        Assignment {
            learner: learner,
            completed: 0,
            date: dt,
            units: units,
        }
    }

    pub fn increment_completed(&mut self) {
        if self.completed < self.units.len() as u32 {
            self.completed += 1;
        }
    }

    pub fn decrement_completed(&mut self) {
        if self.completed > 0 {
            self.completed -= 1;
        }
    }
}
