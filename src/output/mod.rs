use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutputFormat {
    Plain,
    Json,
    Table,
    Markdown,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Plain
    }
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(OutputFormat::Json),
            "table" => Some(OutputFormat::Table),
            "markdown" | "md" => Some(OutputFormat::Markdown),
            "plain" | "text" => Some(OutputFormat::Plain),
            _ => None,
        }
    }
}

pub struct OutputFormatter {
    pub format: OutputFormat,
    pub indent: usize,
    pub color_enabled: bool,
}

impl OutputFormatter {
    pub fn new() -> Self {
        Self {
            format: OutputFormat::Plain,
            indent: 0,
            color_enabled: true,
        }
    }

    pub fn set_format(&mut self, format: OutputFormat) {
        self.format = format;
    }

    pub fn set_indent(&mut self, indent: usize) {
        self.indent = indent;
    }

    pub fn format_output(&self, data: &str) -> String {
        match self.format {
            OutputFormat::Plain => data.to_string(),
            OutputFormat::Json => Self::format_as_json(data),
            OutputFormat::Table => Self::format_as_table(data, self.indent),
            OutputFormat::Markdown => Self::format_as_markdown(data, self.indent),
        }
    }

    fn format_as_json(data: &str) -> String {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(data) {
            serde_json::to_string_pretty(&parsed).unwrap_or_else(|_| data.to_string())
        } else {
            format!("{{\n  \"output\": {}\n}}", Self::escape_json(data))
        }
    }

    fn format_as_table(data: &str, indent: usize) -> String {
        let lines: Vec<&str> = data.lines().collect();
        if lines.is_empty() {
            return "No data".to_string();
        }

        let max_width = 40;
        let indent_str = "  ".repeat(indent);

        let mut result = String::new();
        result.push_str(&format!(
            "{}{}+{}+{}\n",
            indent_str,
            "-".repeat(15),
            "-".repeat(max_width),
            "-".repeat(15)
        ));

        for line in &lines {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let val = parts[1].trim();
                let truncated = if val.len() > max_width {
                    format!("{}...", &val[..max_width - 3])
                } else {
                    val.to_string()
                };
                result.push_str(&format!(
                    "{}| {:^13} | {:<max_width$} |\n",
                    indent_str,
                    key,
                    truncated,
                    max_width = max_width
                ));
            } else {
                let truncated = if line.len() > max_width {
                    format!("{}...", &line[..max_width - 3])
                } else {
                    line.to_string()
                };
                result.push_str(&format!(
                    "{}| {:^13} | {:<max_width$} |\n",
                    indent_str,
                    "",
                    truncated,
                    max_width = max_width
                ));
            }
        }

        result.push_str(&format!(
            "{}{}+{}+{}\n",
            indent_str,
            "-".repeat(15),
            "-".repeat(max_width),
            "-".repeat(15)
        ));

        result
    }

    fn format_as_markdown(data: &str, indent: usize) -> String {
        let indent_str = "  ".repeat(indent);
        let lines: Vec<&str> = data.lines().collect();

        let mut result = String::new();
        result.push_str(&format!("{}```\n", indent_str));
        for line in lines {
            result.push_str(&format!("{}{}\n", indent_str, line));
        }
        result.push_str(&format!("{}```", indent_str));

        result
    }

    fn escape_json(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    }

    pub fn colorize(&self, text: &str, color: &str) -> String {
        if !self.color_enabled {
            return text.to_string();
        }

        let code = match color.to_lowercase().as_str() {
            "red" | "error" => "31",
            "green" | "success" => "32",
            "yellow" | "warn" => "33",
            "blue" | "info" => "34",
            "magenta" => "35",
            "cyan" => "36",
            "white" => "37",
            _ => "37",
        };

        format!("\x1b[{}m{}\x1b[0m", code, text)
    }
}

impl Default for OutputFormatter {
    fn default() -> Self {
        Self::new()
    }
}
