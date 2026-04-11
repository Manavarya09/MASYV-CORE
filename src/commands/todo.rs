use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub priority: u8,
    pub completed: bool,
    pub created_at: u64,
    pub due_date: Option<u64>,
}

impl TodoItem {
    pub fn new(title: String, description: String, priority: u8) -> Self {
        Self {
            id: rand_id(),
            title,
            description,
            priority: priority.min(5),
            completed: false,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            due_date: None,
        }
    }
}

fn rand_id() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as usize
}

pub struct TodoManager {
    items: VecDeque<TodoItem>,
}

impl TodoManager {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn add(&mut self, title: String, description: String, priority: u8) -> usize {
        let item = TodoItem::new(title, description, priority);
        let id = item.id;
        self.items.push_back(item);
        id
    }

    pub fn complete(&mut self, id: usize) -> bool {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = true;
            true
        } else {
            false
        }
    }

    pub fn uncomplete(&mut self, id: usize) -> bool {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = false;
            true
        } else {
            false
        }
    }

    pub fn delete(&mut self, id: usize) -> bool {
        if let Some(pos) = self.items.iter().position(|i| i.id == id) {
            self.items.remove(pos).is_some()
        } else {
            false
        }
    }

    pub fn list(&self, show_completed: bool) -> Vec<&TodoItem> {
        let mut items: Vec<_> = self.items.iter().collect();
        if !show_completed {
            items.retain(|i| !i.completed);
        }
        items.sort_by(|a, b| b.priority.cmp(&a.priority));
        items
    }

    pub fn get_pending_count(&self) -> usize {
        self.items.iter().filter(|i| !i.completed).count()
    }

    pub fn get_completed_count(&self) -> usize {
        self.items.iter().filter(|i| i.completed).count()
    }

    pub fn clear_completed(&mut self) {
        self.items.retain(|i| !i.completed);
    }
}

impl Default for TodoManager {
    fn default() -> Self {
        Self::new()
    }
}
