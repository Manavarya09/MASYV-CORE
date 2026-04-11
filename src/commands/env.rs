use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone)]
pub struct EnvManager {
    vars: HashMap<String, String>,
    original_vars: HashMap<String, String>,
}

impl EnvManager {
    pub fn new() -> Self {
        let mut vars = HashMap::new();

        for (key, value) in env::vars() {
            vars.insert(key.clone(), value.clone());
            vars.insert(key, value);
        }

        Self {
            original_vars: vars.clone(),
            vars,
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.vars.get(key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.vars.insert(key.clone(), value.clone());
        unsafe {
            env::set_var(&key, &value);
        }
    }

    pub fn unset(&mut self, key: &str) -> Option<String> {
        if let Some(val) = self.vars.remove(key) {
            unsafe {
                env::remove_var(key);
            }
            Some(val)
        } else {
            None
        }
    }

    pub fn list(&self) -> Vec<(String, String)> {
        let mut vars: Vec<_> = self
            .vars
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        vars.sort_by(|a, b| a.0.cmp(&b.0));
        vars
    }

    pub fn list_filtered(&self, filter: &str) -> Vec<(String, String)> {
        let filter_lower = filter.to_lowercase();
        self.vars
            .iter()
            .filter(|(k, _)| k.to_lowercase().contains(&filter_lower))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn expand(&self, text: &str) -> String {
        let mut result = text.to_string();

        for (key, value) in &self.vars {
            let pattern = format!("${}", key);
            result = result.replace(&pattern, value);
        }

        result
    }

    pub fn export_to_env(&self) {
        for (key, value) in &self.vars {
            unsafe {
                env::set_var(key, value);
            }
        }
    }

    pub fn reset(&mut self) {
        for (key, _) in &self.original_vars {
            unsafe {
                env::remove_var(key);
            }
        }
        self.vars.clear();
        for (key, value) in env::vars() {
            self.vars.insert(key, value);
        }
    }

    pub fn get_path(&self) -> Vec<String> {
        self.get("PATH")
            .unwrap_or_default()
            .split(if cfg!(windows) { ";" } else { ":" })
            .map(|s| s.to_string())
            .collect()
    }

    pub fn add_to_path(&mut self, path: String) {
        let current = self.get("PATH").unwrap_or_default();
        let separator = if cfg!(windows) { ";" } else { ":" };
        let new_path = if current.is_empty() {
            path
        } else {
            format!("{}{}{}", path, separator, current)
        };
        self.set("PATH".to_string(), new_path);
    }
}

impl Default for EnvManager {
    fn default() -> Self {
        Self::new()
    }
}
