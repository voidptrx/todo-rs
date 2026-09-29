use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: u32,
    pub header: String,
    pub summary: String,
    pub is_done: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TodoData {
    pub tasks: Vec<Task>,
}

impl TodoData {
    pub fn add_task(&mut self, header: String, summary: String) {
        let id = self.tasks.iter().map(|t| t.id).max().map_or(1, |m| m + 1);
        let task = Task {
            id,
            header,
            summary,
            is_done: false,
        };
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, target_id: u32) -> bool {
        let before = self.tasks.len();
        self.tasks.retain(|t| t.id != target_id);
        self.tasks.len() != before
    }

    pub fn complete_task(&mut self, target_id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == target_id) {
            task.is_done = true;
            return true;
        }
        false
    }

    pub fn list_task(&self) {
        for task in &self.tasks {
            if task.is_done {
                println!("[x] {}: {} - {}", task.id, task.header, task.summary);
            } else {
                println!("[ ] {}: {} - {}", task.id, task.header, task.summary);
            }
        }
    }
}
