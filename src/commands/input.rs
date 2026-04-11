use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub command: String,
    pub timestamp: u64,
    pub category: String,
}

impl HistoryEntry {
    pub fn new(command: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let category = Self::categorize_command(&command);

        Self {
            command,
            timestamp,
            category,
        }
    }

    fn categorize_command(command: &str) -> String {
        let cmd = command.split_whitespace().next().unwrap_or("");
        match cmd {
            "ls" | "cd" | "pwd" | "read" | "write" | "mkdir" | "delete" | "rm" => {
                "files".to_string()
            }
            "ping" | "curl" | "scan" => "network".to_string(),
            "processes" | "kill" | "info" | "ps" => "system".to_string(),
            "ai" | "chat" => "ai".to_string(),
            "config" | "theme" | "shortcuts" => "settings".to_string(),
            "plugin" | "plugins" => "plugins".to_string(),
            _ => "general".to_string(),
        }
    }

    pub fn formatted_time(&self) -> String {
        let dt = chrono_lite(self.timestamp);
        dt
    }
}

fn chrono_lite(timestamp: u64) -> String {
    let hours = (timestamp / 3600) % 24;
    let mins = (timestamp / 60) % 60;
    let secs = timestamp % 60;
    format!("{:02}:{:02}:{:02}", hours, mins, secs)
}

pub struct CommandInput {
    pub current: String,
    pub history: VecDeque<HistoryEntry>,
    pub history_index: Option<usize>,
    pub suggestions: Vec<String>,
    pub selected_suggestion: Option<usize>,
    pub max_history: usize,
    pub search_query: String,
    pub search_results: Vec<usize>,
    pub search_index: Option<usize>,
}

impl Default for CommandInput {
    fn default() -> Self {
        Self {
            current: String::new(),
            history: VecDeque::new(),
            history_index: None,
            suggestions: Vec::new(),
            selected_suggestion: None,
            max_history: 100,
            search_query: String::new(),
            search_results: Vec::new(),
            search_index: None,
        }
    }
}

impl CommandInput {
    pub fn push_command(&mut self, command: String) {
        if !command.is_empty() {
            self.history.push_front(HistoryEntry::new(command.clone()));
            if self.history.len() > self.max_history {
                self.history.pop_back();
            }
        }
        self.current.clear();
        self.history_index = None;
        self.suggestions.clear();
        self.selected_suggestion = None;
        self.search_query.clear();
        self.search_results.clear();
        self.search_index = None;
    }

    pub fn navigate_history_up(&mut self) {
        if !self.search_results.is_empty() {
            self.navigate_search_up();
            return;
        }

        if self.history.is_empty() {
            return;
        }
        let new_index = match self.history_index {
            None => 0,
            Some(i) => {
                if i < self.history.len() - 1 {
                    i + 1
                } else {
                    return;
                }
            }
        };
        self.history_index = Some(new_index);
        if let Some(entry) = self.history.get(new_index) {
            self.current = entry.command.clone();
        }
    }

    pub fn navigate_history_down(&mut self) {
        if !self.search_results.is_empty() {
            self.navigate_search_down();
            return;
        }

        match self.history_index {
            None => return,
            Some(0) => {
                self.history_index = None;
                self.current.clear();
            }
            Some(i) => {
                let new_index = i - 1;
                self.history_index = Some(new_index);
                if let Some(entry) = self.history.get(new_index) {
                    self.current = entry.command.clone();
                }
            }
        }
    }

    pub fn search_history(&mut self, query: &str) {
        self.search_query = query.to_lowercase();

        if self.search_query.is_empty() {
            self.search_results.clear();
            self.search_index = None;
            return;
        }

        self.search_results.clear();

        for (idx, entry) in self.history.iter().enumerate() {
            if Self::fuzzy_match(&entry.command.to_lowercase(), &self.search_query) {
                self.search_results.push(idx);
            }
        }

        if !self.search_results.is_empty() {
            self.search_index = Some(0);
            self.current = self.history[self.search_results[0]].command.clone();
        }
    }

    fn fuzzy_match(text: &str, pattern: &str) -> bool {
        if text.contains(pattern) {
            return true;
        }

        let mut pattern_chars = pattern.chars().peekable();
        for c in text.chars() {
            if pattern_chars.peek() == Some(&c) {
                pattern_chars.next();
            }
        }
        pattern_chars.peek().is_none()
    }

    fn navigate_search_up(&mut self) {
        if self.search_results.is_empty() {
            return;
        }

        let new_idx = match self.search_index {
            None => 0,
            Some(i) if i < self.search_results.len() - 1 => i + 1,
            Some(_) => return,
        };

        self.search_index = Some(new_idx);
        self.current = self.history[self.search_results[new_idx]].command.clone();
    }

    fn navigate_search_down(&mut self) {
        match self.search_index {
            None => return,
            Some(0) => {
                self.search_index = None;
                self.current.clear();
            }
            Some(i) => {
                let new_idx = i - 1;
                self.search_index = Some(new_idx);
                self.current = self.history[self.search_results[new_idx]].command.clone();
            }
        }
    }

    pub fn get_search_results_count(&self) -> usize {
        self.search_results.len()
    }

    pub fn get_current_search_index(&self) -> Option<usize> {
        self.search_index.map(|i| i + 1)
    }

    pub fn update_suggestions(&mut self) {
        if self.current.is_empty() {
            self.suggestions.clear();
            self.selected_suggestion = None;
            return;
        }

        let commands = vec![
            "help",
            "status",
            "clear",
            "stats",
            "time",
            "ai ",
            "cd ",
            "ls ",
            "pwd",
            "read ",
            "write ",
            "mkdir ",
            "delete ",
            "ping ",
            "curl ",
            "scan ",
            "processes",
            "kill ",
            "info ",
            "theme ",
            "config ",
            "plugin ",
        ];

        self.suggestions = commands
            .iter()
            .filter(|c| c.starts_with(&self.current.to_lowercase()))
            .map(|c| c.to_string())
            .collect();

        if !self.suggestions.is_empty() {
            self.selected_suggestion = Some(0);
        }
    }

    pub fn select_next_suggestion(&mut self) {
        if let Some(idx) = self.selected_suggestion {
            if idx < self.suggestions.len() - 1 {
                self.selected_suggestion = Some(idx + 1);
            }
        }
    }

    pub fn select_prev_suggestion(&mut self) {
        if let Some(idx) = self.selected_suggestion {
            if idx > 0 {
                self.selected_suggestion = Some(idx - 1);
            }
        }
    }

    pub fn apply_selected_suggestion(&mut self) {
        if let Some(idx) = self.selected_suggestion {
            if let Some(suggestion) = self.suggestions.get(idx) {
                self.current = suggestion.clone();
                self.selected_suggestion = None;
                self.suggestions.clear();
            }
        }
    }

    pub fn clear(&mut self) {
        self.current.clear();
        self.history.clear();
        self.history_index = None;
        self.suggestions.clear();
        self.selected_suggestion = None;
        self.search_query.clear();
        self.search_results.clear();
        self.search_index = None;
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
        self.history_index = None;
    }

    pub fn get_history_list(&self, limit: Option<usize>) -> Vec<(String, String)> {
        let limit = limit.unwrap_or(20).min(self.history.len());
        self.history
            .iter()
            .take(limit)
            .map(|e| (e.command.clone(), e.formatted_time()))
            .collect()
    }

    pub fn get_history_by_category(&self, category: &str) -> Vec<String> {
        self.history
            .iter()
            .filter(|e| e.category == category)
            .map(|e| e.command.clone())
            .collect()
    }
}
