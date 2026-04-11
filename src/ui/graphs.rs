use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    pub values: VecDeque<f32>,
    pub max_points: usize,
    pub min_val: f32,
    pub max_val: f32,
}

impl GraphData {
    pub fn new(max_points: usize) -> Self {
        Self {
            values: VecDeque::new(),
            max_points,
            min_val: 0.0,
            max_val: 100.0,
        }
    }

    pub fn add(&mut self, value: f32) {
        self.values.push_front(value);
        if self.values.len() > self.max_points {
            self.values.pop_back();
        }
        if value > self.max_val {
            self.max_val = value;
        }
        if value < self.min_val {
            self.min_val = value;
        }
    }

    pub fn get_normalized(&self) -> Vec<f32> {
        let range = self.max_val - self.min_val;
        if range == 0.0 {
            return self.values.iter().map(|_| 0.5).collect();
        }
        self.values
            .iter()
            .map(|v| (v - self.min_val) / range)
            .collect()
    }
}

pub struct RealtimeGraph {
    pub cpu: GraphData,
    pub memory: GraphData,
    pub network: GraphData,
    pub power: GraphData,
    pub temperature: GraphData,
    pub processing: GraphData,
}

impl RealtimeGraph {
    pub fn new() -> Self {
        Self {
            cpu: GraphData::new(50),
            memory: GraphData::new(50),
            network: GraphData::new(50),
            power: GraphData::new(50),
            temperature: GraphData::new(50),
            processing: GraphData::new(50),
        }
    }

    pub fn update(
        &mut self,
        cpu: f32,
        memory: f32,
        network: f32,
        power: f32,
        temp: f32,
        load: f32,
    ) {
        self.cpu.add(cpu);
        self.memory.add(memory);
        self.network.add(network);
        self.power.add(power);
        self.temperature.add(temp);
        self.processing.add(load);
    }
}

impl Default for RealtimeGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SystemAlert {
    pub id: usize,
    pub message: String,
    pub level: AlertLevel,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}

impl SystemAlert {
    pub fn new(message: String, level: AlertLevel) -> Self {
        Self {
            id: rand_id(),
            message,
            level,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

fn rand_id() -> usize {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as usize
}

pub struct AlertManager {
    pub alerts: Vec<SystemAlert>,
    pub max_alerts: usize,
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            alerts: Vec::new(),
            max_alerts: 10,
        }
    }

    pub fn add(&mut self, message: String, level: AlertLevel) {
        self.alerts.insert(0, SystemAlert::new(message, level));
        if self.alerts.len() > self.max_alerts {
            self.alerts.pop();
        }
    }

    pub fn get_critical(&self) -> Vec<&SystemAlert> {
        self.alerts
            .iter()
            .filter(|a| a.level == AlertLevel::Critical)
            .collect()
    }

    pub fn clear(&mut self) {
        self.alerts.clear();
    }
}

impl Default for AlertManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct HexagonMonitor {
    pub segments: Vec<f32>,
}

impl HexagonMonitor {
    pub fn new() -> Self {
        Self {
            segments: vec![50.0; 6],
        }
    }

    pub fn update(&mut self, values: [f32; 6]) {
        for (i, v) in values.iter().enumerate() {
            if i < self.segments.len() {
                self.segments[i] = *v;
            }
        }
    }
}

impl Default for HexagonMonitor {
    fn default() -> Self {
        Self::new()
    }
}
