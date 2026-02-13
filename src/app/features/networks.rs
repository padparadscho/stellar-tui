use std::collections::HashMap;

use crate::app::core::forms::{FieldSpec, FieldType, FormState};
use crate::app::core::state::App;
use crate::app::core::types::NetworkEditor;
use crate::settings::Network;

impl App {
    /// Selects next network in settings list
    pub(in crate::app) fn select_next_network(&mut self) {
        if self.selected_network + 1 < self.settings.networks.len() {
            self.selected_network += 1;
        }
    }

    /// Selects previous network in settings list
    pub(in crate::app) fn select_prev_network(&mut self) {
        if self.selected_network > 0 {
            self.selected_network -= 1;
        }
    }

    /// Opens network editor with optional existing entry
    pub(in crate::app) fn start_network_editor(&mut self, index: Option<usize>) {
        let specs = network_field_specs();
        let mut form = FormState::from_specs(&specs);
        if let Some(index) = index {
            if let Some(network) = self.settings.networks.get(index) {
                form.set_value("name", network.name.clone());
                form.set_value("endpoint", network.endpoint.clone());
            }
        }
        self.network_editor = Some(NetworkEditor {
            form,
            editing_index: index,
        });
        self.refresh_network_errors();
    }

    /// Validates and saves network editor values
    pub(in crate::app) fn save_network_editor(&mut self) {
        let editor = match self.network_editor.take() {
            Some(editor) => editor,
            None => return,
        };
        let name = editor.form.field_value("name").unwrap_or("").trim();
        let endpoint = editor.form.field_value("endpoint").unwrap_or("").trim();

        if name.is_empty() || endpoint.is_empty() {
            self.set_timed_status("Network name and endpoint are required".to_string(), 5);
            self.network_errors = network_errors_from_values(name, endpoint);
            self.network_editor = Some(editor);
            return;
        }
        if !endpoint.starts_with("http://") && !endpoint.starts_with("https://") {
            self.set_timed_status(
                "Endpoint must start with http:// or https://".to_string(),
                5,
            );
            self.network_errors = network_errors_from_values(name, endpoint);
            self.network_editor = Some(editor);
            return;
        }

        let network = Network {
            name: name.to_string(),
            endpoint: endpoint.to_string(),
        };

        match editor.editing_index {
            Some(index) => {
                if index < self.settings.networks.len() {
                    self.settings.networks[index] = network;
                }
            }
            None => self.settings.networks.push(network),
        }

        if self.selected_network >= self.settings.networks.len() {
            self.selected_network = self.settings.networks.len().saturating_sub(1);
        }

        if let Err(err) = self.settings.save() {
            self.set_timed_status(format!("Failed to save settings: {}", err), 5);
        } else {
            self.set_timed_status("Network saved".to_string(), 5);
        }
    }

    /// Deletes selected network when allowed
    pub(in crate::app) fn delete_network(&mut self) {
        if self.settings.networks.len() <= 1 {
            self.set_timed_status("At least one network is required".to_string(), 5);
            return;
        }
        if self.selected_network < self.settings.networks.len() {
            self.settings.networks.remove(self.selected_network);
            if self.settings.active_network >= self.settings.networks.len() {
                self.settings.active_network = self.settings.networks.len() - 1;
            }
            if self.selected_network >= self.settings.networks.len() {
                self.selected_network = self.settings.networks.len() - 1;
            }
            if let Err(err) = self.settings.save() {
                self.set_timed_status(format!("Failed to save settings: {}", err), 5);
            } else {
                self.set_timed_status("Network deleted".to_string(), 5);
            }
        }
    }

    /// Rebuilds network editor validation errors
    pub(in crate::app) fn refresh_network_errors(&mut self) {
        if let Some(editor) = &self.network_editor {
            let name = editor.form.field_value("name").unwrap_or("");
            let endpoint = editor.form.field_value("endpoint").unwrap_or("");
            self.network_errors = network_errors_from_values(name, endpoint);
        }
    }
}

fn network_field_specs() -> Vec<FieldSpec> {
    vec![
        FieldSpec {
            key: "name",
            label: "Network name",
            field_type: FieldType::Text,
            optional: false,
            default_value: "",
            hint: "Display name",
        },
        FieldSpec {
            key: "endpoint",
            label: "RPC endpoint",
            field_type: FieldType::Text,
            optional: false,
            default_value: "",
            hint: "http(s) endpoint",
        },
    ]
}

fn network_errors_from_values(name: &str, endpoint: &str) -> HashMap<String, String> {
    let mut errors = HashMap::new();
    if name.trim().is_empty() {
        errors.insert("name".to_string(), "Required".to_string());
    }
    if endpoint.trim().is_empty() {
        errors.insert("endpoint".to_string(), "Required".to_string());
    } else if !endpoint.starts_with("http://") && !endpoint.starts_with("https://") {
        errors.insert(
            "endpoint".to_string(),
            "Must start with http(s)".to_string(),
        );
    }
    errors
}
