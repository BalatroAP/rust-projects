pub struct Todo {
    pub is_complete: bool,
    pub task: String,
}

impl Todo {
    pub fn new(task: String, is_complete: bool) -> Self {
        Self { is_complete, task }
    }

    pub fn invert_completed(&mut self) {
        self.is_complete = !self.is_complete;
    }
}
