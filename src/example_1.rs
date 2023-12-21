pub trait Task {
    fn description(&self) -> &str;
    fn new(description: &str) -> Self;
    fn execute(&self);
    fn modify_description(&mut self, new_description: &str);
}

// Struct implementing Task trait
pub struct SimpleTask {
    description: String,
}

impl Task for SimpleTask {
    fn description(&self) -> &str {
        &self.description
    }

    fn new(description: &str) -> Self {
        SimpleTask {
            description: description.to_string(),
        }
    }

    fn execute(&self) {
        println!("Executing task: {}", self.description);
    }

    fn modify_description(&mut self, new_description: &str) {
        self.description = new_description.to_string();
    }
}

// Function to execute a task
pub fn execute_task<T: Task>(task: &T) {
    task.execute();
}

// Function to modify the description of a task
pub fn modify_task_description<T: Task>(task: &mut T, new_description: &str) {
    task.modify_description(new_description);
}
