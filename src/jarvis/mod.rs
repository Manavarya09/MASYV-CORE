#[derive(Debug, Clone)]
pub struct JarviState {
    pub activation_level: f32,
    pub system_mode: String,
    pub threat_level: String,
    pub power_consumption: f32,
    pub network_activity: f32,
    pub security_status: String,
    pub core_temp: f32,
    pub memory_usage: f32,
    pub processing_load: f32,
    pub active_connections: usize,
    pub alerts: Vec<String>,
    pub encryption_level: String,
    pub system_integrity: f32,
    pub shield_status: String,
    pub last_scan: String,
    pub cpu_cores: usize,
    pub gpu_usage: f32,
}

impl JarviState {
    pub fn new() -> Self {
        Self {
            activation_level: 100.0,
            system_mode: "OPERATIONAL".to_string(),
            threat_level: "NONE".to_string(),
            power_consumption: 45.2,
            network_activity: 12.5,
            security_status: "SECURE".to_string(),
            core_temp: 42.5,
            memory_usage: 38.0,
            processing_load: 25.0,
            active_connections: 7,
            alerts: Vec::new(),
            encryption_level: "AES-256".to_string(),
            system_integrity: 99.9,
            shield_status: "ACTIVE".to_string(),
            last_scan: "Clean".to_string(),
            cpu_cores: 8,
            gpu_usage: 35.0,
        }
    }

    pub fn update(&mut self) {
        use std::time::SystemTime;
        let time = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as f32;

        self.power_consumption = 40.0 + (time.sin().abs() * 10.0);
        self.network_activity = 10.0 + (time.cos().abs() * 20.0);
        self.core_temp = 40.0 + (time.sin().abs() * 8.0);
        self.memory_usage = 35.0 + (time.sin().abs() * 15.0);
        self.processing_load = 20.0 + (time.cos().abs() * 20.0);
        self.system_integrity = 98.0 + (time.sin().abs() * 2.0);
        self.gpu_usage = 30.0 + (time.cos().abs() * 25.0);
    }

    pub fn status_summary(&self) -> String {
        format!(
            "MODE: {} | THREAT: {} | POWER: {:.1}% | TEMP: {:.1}C",
            self.system_mode, self.threat_level, self.power_consumption, self.core_temp
        )
    }

    pub fn get_status_emoji(&self) -> &'static str {
        match self.system_mode.as_str() {
            "OPERATIONAL" => "[+]",
            "STANDBY" => "[-]",
            "HIGH ALERT" => "[!]",
            "MAINTENANCE" => "[~]",
            _ => "[?]",
        }
    }
}

impl Default for JarviState {
    fn default() -> Self {
        Self::new()
    }
}
