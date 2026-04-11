use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub key: String,
    pub action: String,
    pub description: String,
}

pub fn get_shortcuts() -> Vec<Shortcut> {
    vec![
        Shortcut {
            key: "Enter".to_string(),
            action: "execute".to_string(),
            description: "Execute command".to_string(),
        },
        Shortcut {
            key: "Arrow Up".to_string(),
            action: "history_up".to_string(),
            description: "Previous command".to_string(),
        },
        Shortcut {
            key: "Arrow Down".to_string(),
            action: "history_down".to_string(),
            description: "Next command".to_string(),
        },
        Shortcut {
            key: "Ctrl+C".to_string(),
            action: "clear_input".to_string(),
            description: "Clear input".to_string(),
        },
        Shortcut {
            key: "Ctrl+L".to_string(),
            action: "clear_output".to_string(),
            description: "Clear output".to_string(),
        },
        Shortcut {
            key: "Ctrl+S".to_string(),
            action: "save_output".to_string(),
            description: "Save output to file".to_string(),
        },
        Shortcut {
            key: "?".to_string(),
            action: "toggle_help".to_string(),
            description: "Toggle shortcuts overlay".to_string(),
        },
        Shortcut {
            key: "T".to_string(),
            action: "toggle_theme".to_string(),
            description: "Cycle theme".to_string(),
        },
        Shortcut {
            key: "Esc".to_string(),
            action: "close_overlay".to_string(),
            description: "Close overlay".to_string(),
        },
        Shortcut {
            key: "Ctrl+Q".to_string(),
            action: "quit".to_string(),
            description: "Quit application".to_string(),
        },
    ]
}
