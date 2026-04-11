use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub created_at: u64,
    pub tags: Vec<String>,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        Self {
            id: rand_id(),
            title,
            content,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            tags: Vec::new(),
        }
    }
}

fn rand_id() -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as usize
}

pub struct NotesManager {
    notes: HashMap<usize, Note>,
}

impl NotesManager {
    pub fn new() -> Self {
        Self {
            notes: HashMap::new(),
        }
    }

    pub fn add(&mut self, title: String, content: String) -> usize {
        let note = Note::new(title, content);
        let id = note.id;
        self.notes.insert(id, note);
        id
    }

    pub fn get(&self, id: usize) -> Option<&Note> {
        self.notes.get(&id)
    }

    pub fn list(&self) -> Vec<&Note> {
        let mut notes: Vec<_> = self.notes.values().collect();
        notes.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        notes
    }

    pub fn delete(&mut self, id: usize) -> bool {
        self.notes.remove(&id).is_some()
    }

    pub fn search(&self, query: &str) -> Vec<&Note> {
        let query_lower = query.to_lowercase();
        self.notes
            .values()
            .filter(|n| {
                n.title.to_lowercase().contains(&query_lower)
                    || n.content.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn count(&self) -> usize {
        self.notes.len()
    }
}

impl Default for NotesManager {
    fn default() -> Self {
        Self::new()
    }
}
