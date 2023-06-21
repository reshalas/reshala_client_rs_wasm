use super::{Subjects, Task};

impl Task {
    pub fn get_uuid(&self) -> String {
        self.id.clone()
    }

    pub fn owner_id(&self) -> String {
        self.owner_id.clone()
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
