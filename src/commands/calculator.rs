use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Calculator {
    history: VecDeque<(String, String)>,
    max_history: usize,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            max_history: 50,
        }
    }

    pub fn evaluate(&mut self, expression: &str) -> Result<String, String> {
        let expr = expression.trim();

        if expr.is_empty() {
            return Err("Empty expression".to_string());
        }

        let result = self.parse_evaluate(expr)?;

        self.history.push_front((expr.to_string(), result.clone()));
        if self.history.len() > self.max_history {
            self.history.pop_back();
        }

        Ok(result)
    }

    fn parse_evaluate(&self, expr: &str) -> Result<String, String> {
        let expr = expr.replace(" ", "");

        if expr.contains("+") {
            let parts: Vec<&str> = expr.split('+').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].parse().map_err(|_| "Invalid number")?;
                let b: f64 = parts[1].parse().map_err(|_| "Invalid number")?;
                return Ok(format!("{}", a + b));
            }
        }

        if expr.contains("-") && !expr.starts_with('-') {
            let parts: Vec<&str> = expr.split('-').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].parse().map_err(|_| "Invalid number")?;
                let b: f64 = parts[1].parse().map_err(|_| "Invalid number")?;
                return Ok(format!("{}", a - b));
            }
        }

        if expr.contains("*") || expr.contains("x") {
            let op = if expr.contains("*") { "*" } else { "x" };
            let parts: Vec<&str> = expr.split(op).collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].parse().map_err(|_| "Invalid number")?;
                let b: f64 = parts[1].parse().map_err(|_| "Invalid number")?;
                return Ok(format!("{}", a * b));
            }
        }

        if expr.contains("/") {
            let parts: Vec<&str> = expr.split('/').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].parse().map_err(|_| "Invalid number")?;
                let b: f64 = parts[1].parse().map_err(|_| "Invalid number")?;
                if b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                return Ok(format!("{}", a / b));
            }
        }

        if expr.contains("%") {
            let parts: Vec<&str> = expr.split('%').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].parse().map_err(|_| "Invalid number")?;
                let b: f64 = parts[1].parse().map_err(|_| "Invalid number")?;
                return Ok(format!("{}", a % b));
            }
        }

        if expr.contains("^") {
            let parts: Vec<&str> = expr.split('^').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].parse().map_err(|_| "Invalid number")?;
                let b: f64 = parts[1].parse().map_err(|_| "Invalid number")?;
                return Ok(format!("{}", a.powf(b)));
            }
        }

        if let Ok(num) = expr.parse::<f64>() {
            return Ok(format!("{}", num));
        }

        Err("Invalid expression. Use: a+b, a-b, a*b, a/b, a^b".to_string())
    }

    pub fn get_history(&self) -> Vec<(String, String)> {
        self.history.iter().cloned().collect()
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}
