use super::{Subjects, Task};

impl Task {
    pub fn uuid(&self) -> String {
        self.id.clone()
    }

    pub fn owner_username(&self) -> String {
        self.owner_username.clone()
    }

    pub fn task(&self) -> String {
        self.task.clone()
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn subject(&self) -> Subjects {
        self.subject
    }
}
