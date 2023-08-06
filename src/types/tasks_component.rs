use super::{TasksComponent, Task};

impl TasksComponent{
    pub fn tasks(&self)->Vec<Task>{
        self.tasks.clone()
    }
}