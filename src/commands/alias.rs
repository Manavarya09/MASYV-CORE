use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alias {
    pub name: String,
    pub command: String,
    pub description: Option<String>,
    pub use_count: u32,
}

impl Alias {
    pub fn new(name: String, command: String) -> Self {
        Self {
            name,
            command,
            description: None,
            use_count: 0,
        }
    }
}

pub struct AliasManager {
    aliases: HashMap<String, Alias>,
}

impl AliasManager {
    pub fn new() -> Self {
        Self {
            aliases: HashMap::new(),
        }
    }

    pub fn create(
        &mut self,
        name: String,
        command: String,
        description: Option<String>,
    ) -> Result<(), String> {
        if name.contains(' ') || name.contains('|') || name.contains('&') {
            return Err("Alias name cannot contain spaces, |, or &".to_string());
        }

        let mut alias = Alias::new(name.clone(), command);
        alias.description = description;

        self.aliases.insert(name, alias);
        Ok(())
    }

    pub fn delete(&mut self, name: &str) -> Result<(), String> {
        if self.aliases.remove(name).is_some() {
            Ok(())
        } else {
            Err(format!("Alias '{}' not found", name))
        }
    }

    pub fn get(&self, name: &str) -> Option<&Alias> {
        self.aliases.get(name)
    }

    pub fn resolve(&mut self, name: &str) -> Option<String> {
        let alias = self.aliases.get(name)?;
        let mut alias = alias.clone();
        alias.use_count += 1;
        self.aliases.insert(name.to_string(), alias.clone());
        Some(alias.command)
    }

    pub fn list(&self) -> Vec<&Alias> {
        self.aliases.values().collect()
    }

    pub fn find_by_prefix(&self, prefix: &str) -> Vec<&Alias> {
        self.aliases
            .values()
            .filter(|a| a.name.starts_with(prefix))
            .collect()
    }

    pub fn get_stats(&self) -> (usize, usize, String) {
        let total = self.aliases.len();
        let most_used = self
            .aliases
            .values()
            .max_by_key(|a| a.use_count)
            .map(|a| format!("{} ({} uses)", a.name, a.use_count))
            .unwrap_or_else(|| "none".to_string());

        let names: Vec<String> = self.aliases.keys().cloned().collect();
        let all_names = names.join(", ");

        (total, if most_used != "none" { 1 } else { 0 }, all_names)
    }
}

impl Default for AliasManager {
    fn default() -> Self {
        Self::new()
    }
}
