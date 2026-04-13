use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskScheduler {
    pub tasks: Vec<ScheduledTask>,
    pub task_history: VecDeque<TaskHistoryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: String,
    pub name: String,
    pub command: String,
    pub interval_seconds: u64,
    pub next_run: u64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub success: bool,
    pub output: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskHistoryEntry {
    pub task_id: String,
    pub task_name: String,
    pub result: TaskResult,
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            task_history: VecDeque::new(),
        }
    }

    pub fn add_task(&mut self, name: String, command: String, interval: u64) -> String {
        let id = format!("task_{}", self.tasks.len() + 1);

        self.tasks.push(ScheduledTask {
            id: id.clone(),
            name,
            command,
            interval_seconds: interval,
            next_run: 0,
            enabled: true,
        });

        id
    }

    pub fn list_tasks(&self) -> &[ScheduledTask] {
        &self.tasks
    }

    pub fn enable_task(&mut self, id: &str) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.enabled = true;
            true
        } else {
            false
        }
    }

    pub fn disable_task(&mut self, id: &str) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.enabled = false;
            true
        } else {
            false
        }
    }

    pub fn get_summary(&self) -> String {
        let total = self.tasks.len();
        let enabled = self.tasks.iter().filter(|t| t.enabled).count();
        format!("Tasks: {} | Active: {}", total, enabled)
    }
}

impl Default for TaskScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroSystem {
    pub macros: Vec<Macro>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    pub id: String,
    pub name: String,
    pub trigger: String,
    pub actions: Vec<String>,
    pub enabled: bool,
}

impl MacroSystem {
    pub fn new() -> Self {
        Self { macros: Vec::new() }
    }

    pub fn create_macro(&mut self, name: String, trigger: String) -> String {
        let id = format!("macro_{}", self.macros.len() + 1);

        self.macros.push(Macro {
            id: id.clone(),
            name,
            trigger,
            actions: Vec::new(),
            enabled: true,
        });

        id
    }

    pub fn delete_macro(&mut self, id: &str) -> bool {
        if let Some(pos) = self.macros.iter().position(|m| m.id == id) {
            self.macros.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn list_macros(&self) -> &[Macro] {
        &self.macros
    }

    pub fn get_summary(&self) -> String {
        let total = self.macros.len();
        let enabled = self.macros.iter().filter(|m| m.enabled).count();
        format!("Macros: {} | Active: {}", total, enabled)
    }
}

impl Default for MacroSystem {
    fn default() -> Self {
        Self::new()
    }
}
