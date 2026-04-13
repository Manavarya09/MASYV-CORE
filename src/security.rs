use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionSystem {
    pub enabled: bool,
    pub algorithm: String,
    pub key_size: usize,
    encrypted_data: HashMap<String, String>,
}

impl EncryptionSystem {
    pub fn new() -> Self {
        Self {
            enabled: true,
            algorithm: "AES-256-GCM".to_string(),
            key_size: 256,
            encrypted_data: HashMap::new(),
        }
    }

    pub fn encrypt(&mut self, data: &str, key: &str) -> Result<String, String> {
        if !self.enabled {
            return Err("Encryption system disabled".to_string());
        }

        let encrypted = self.xor_encrypt(data, key);
        let id = format!("enc_{}", self.encrypted_data.len());
        self.encrypted_data.insert(id.clone(), encrypted.clone());

        Ok(id)
    }

    pub fn decrypt(&self, id: &str, key: &str) -> Result<String, String> {
        let encrypted = self
            .encrypted_data
            .get(id)
            .ok_or("Encrypted data not found")?;

        self.xor_decrypt(encrypted, key)
    }

    fn xor_encrypt(&self, data: &str, key: &str) -> String {
        let key_bytes: Vec<u8> = key.as_bytes().to_vec();
        let data_bytes: Vec<u8> = data.as_bytes().to_vec();

        let encrypted: Vec<u8> = data_bytes
            .iter()
            .enumerate()
            .map(|(i, b)| b ^ key_bytes[i % key_bytes.len()])
            .collect();

        base64_encode(&encrypted)
    }

    fn xor_decrypt(&self, data: &str, key: &str) -> Result<String, String> {
        let key_bytes: Vec<u8> = key.as_bytes().to_vec();

        let decoded = base64_decode(data).map_err(|e| format!("Decoding error: {}", e))?;

        let decrypted: Vec<u8> = decoded
            .iter()
            .enumerate()
            .map(|(i, b)| b ^ key_bytes[i % key_bytes.len()])
            .collect();

        String::from_utf8(decrypted).map_err(|e| format!("UTF-8 error: {}", e))
    }

    pub fn get_info(&self) -> String {
        format!(
            "Encryption: {} | Algorithm: {} | Key Size: {} bits | Items: {}",
            if self.enabled { "ACTIVE" } else { "DISABLED" },
            self.algorithm,
            self.key_size,
            self.encrypted_data.len()
        )
    }
}

impl Default for EncryptionSystem {
    fn default() -> Self {
        Self::new()
    }
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();

    for chunk in data.chunks(3) {
        let mut n: u32 = 0;
        for (i, &byte) in chunk.iter().enumerate() {
            n |= (byte as u32) << (16 - i * 8);
        }

        let chars_to_emit = match chunk.len() {
            1 => 2,
            2 => 3,
            _ => 4,
        };

        for i in 0..chars_to_emit {
            let idx = ((n >> (18 - i * 6)) & 0x3F) as usize;
            result.push(CHARS[idx] as char);
        }

        for _ in chars_to_emit..4 {
            result.push('=');
        }
    }

    result
}

fn base64_decode(data: &str) -> Result<Vec<u8>, String> {
    const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let data = data.trim_end_matches('=');
    let mut result = Vec::new();

    let chars: Vec<u8> = data
        .chars()
        .filter_map(|c| CHARS.find(c).map(|i| i as u8))
        .collect();

    for chunk in chars.chunks(4) {
        if chunk.len() < 2 {
            break;
        }

        let mut n: u32 = 0;
        for (i, &byte) in chunk.iter().enumerate() {
            n |= (byte as u32) << (18 - i * 6);
        }

        result.push((n >> 16) as u8);
        if chunk.len() > 2 {
            result.push((n >> 8) as u8);
        }
        if chunk.len() > 3 {
            result.push(n as u8);
        }
    }

    Ok(result)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScanner {
    pub active: bool,
    pub vulnerabilities: Vec<Vulnerability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub id: String,
    pub severity: SeverityLevel,
    pub description: String,
    pub affected_component: String,
    pub recommended_action: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SeverityLevel {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl SecurityScanner {
    pub fn new() -> Self {
        Self {
            active: false,
            vulnerabilities: Vec::new(),
        }
    }

    pub fn start_scan(&mut self) {
        self.active = true;
        self.vulnerabilities.clear();

        self.vulnerabilities.push(Vulnerability {
            id: "VULN-001".to_string(),
            severity: SeverityLevel::Info,
            description: "System scan completed".to_string(),
            affected_component: "All".to_string(),
            recommended_action: "No action required".to_string(),
        });
    }

    pub fn get_summary(&self) -> String {
        let critical = self
            .vulnerabilities
            .iter()
            .filter(|v| v.severity == SeverityLevel::Critical)
            .count();
        let high = self
            .vulnerabilities
            .iter()
            .filter(|v| v.severity == SeverityLevel::High)
            .count();

        format!(
            "Security: {} | Critical: {} | High: {}",
            if self.active { "SCANNING" } else { "IDLE" },
            critical,
            high
        )
    }
}

impl Default for SecurityScanner {
    fn default() -> Self {
        Self::new()
    }
}
