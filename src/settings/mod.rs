use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    /// Display name
    pub name: String,
    /// Stellar RPC endpoint
    pub endpoint: String,
}

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Ordered list of networks
    #[serde(rename = "profiles", alias = "networks")]
    pub networks: Vec<Network>,
    /// Index into networks for the currently active network
    #[serde(rename = "active_profile", alias = "active_network")]
    pub active_network: usize,
}

impl Settings {
    /// Loads settings from disk, falling back to built in defaults when missing or invalid
    pub fn load_or_default() -> Self {
        if let Some(path) = settings_path() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(settings) = serde_json::from_str::<Settings>(&content) {
                    return settings;
                }
            }
        }

        Self::default_settings()
    }

    /// Persists the current settings to disk
    pub fn save(&self) -> anyhow::Result<()> {
        let path = settings_path().unwrap_or_else(default_settings_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Returns a reference to the currently active network, if valid
    pub fn active_network(&self) -> Option<&Network> {
        self.networks.get(self.active_network)
    }

    /// Switches the active network to the given index
    pub fn set_active_network(&mut self, index: usize) {
        if index < self.networks.len() {
            self.active_network = index;
        }
    }

    /// Returns the built in default settings
    pub fn default_settings() -> Self {
        Self {
            networks: vec![Network {
                name: "Testnet".to_string(),
                endpoint: "https://soroban-testnet.stellar.org".to_string(),
            }],
            active_network: 0,
        }
    }
}

/// Resolves the settings file path via directories
fn settings_path() -> Option<PathBuf> {
    // Identifier kept stable to preserve prior settings location
    ProjectDirs::from("org", "stellar", "stellar-tui")
        .map(|dirs| dirs.config_dir().join("config.json"))
}

/// Fallback settings path in the current working directory
fn default_settings_path() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("config.json")
}
