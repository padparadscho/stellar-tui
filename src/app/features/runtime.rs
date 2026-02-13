use serde_json::Value;
use std::collections::HashMap;
use std::sync::mpsc;
use std::time::Instant;

use crate::app::core::forms::FormState;
use crate::app::core::state::App;
use crate::app::core::types::{FocusPane, ModalState, TimedStatus};
use crate::app::methods::{method_specs, MethodSpec};
use crate::rpc::RpcClient;
use crate::settings::Settings;

impl App {
    /// Creates app state from settings and method specs
    pub fn new(settings: Settings) -> Self {
        let methods = method_specs();
        let selected_network = settings.active_network;
        let request_forms = methods
            .iter()
            .map(|method| FormState::from_specs(&method.fields))
            .collect();
        let method_errors = methods.iter().map(|_| HashMap::new()).collect();

        Self {
            focus: FocusPane::Methods,
            settings,
            selected_network,
            ui_regions: None,
            modal: ModalState::None,
            modal_scroll: 0,
            modal_max_scroll: 0,
            zoomed_pane: None,
            methods,
            selected_method: 0,
            request_forms,
            method_errors,
            network_editor: None,
            network_errors: HashMap::new(),
            status: "Ready".to_string(),
            timed_status: None,
            spinner_frame: 0,
            pending_request: None,
            last_response: String::new(),
            paginated_response: None,
            response_page: 0,
            response_scroll: 0,
            response_search_query: String::new(),
            response_search_cursor: 0,
            response_search_matches: Vec::new(),
            response_search_current: 0,
            search_last_changed: None,
            response_selection_start: None,
            response_selection_end: None,
            response_selecting: false,
            wrap_metrics: None,
            clipboard: None,
        }
    }

    pub fn execute_request(&mut self) {
        let method = &self.methods[self.selected_method];
        let form = &self.request_forms[self.selected_method];
        let params = match method.build_params(form) {
            Ok(value) => value,
            Err(message) => {
                self.set_timed_status(format!("Params error: {}", message), 5);
                return;
            }
        };

        let endpoint = match self.settings.active_network() {
            Some(network) => network.endpoint.clone(),
            None => {
                self.set_timed_status("No active network".to_string(), 5);
                return;
            }
        };

        self.status = "Calling...".to_string();
        self.spinner_frame = 0;

        let (tx, rx) = mpsc::channel();
        let method_name = method.name.to_string();
        tokio::spawn(async move {
            let client = RpcClient::new(endpoint);
            let result = client
                .call::<Value, Value>(&method_name, params)
                .await
                .map_err(|e| e.to_string());
            let _ = tx.send(result);
        });
        self.pending_request = Some(rx);
    }

    /// Polls background request receiver on each event loop tick
    pub fn tick(&mut self) {
        if let Some(ref rx) = self.pending_request {
            match rx.try_recv() {
                Ok(Ok(payload)) => {
                    self.pending_request = None;
                    self.status = "Ready".to_string();
                    if let Some(ref error) = payload.error {
                        self.set_timed_status(
                            format!("Error {}: {}", error.code, error.message),
                            5,
                        );
                    } else {
                        self.set_timed_status("Completed".to_string(), 5);
                    }
                    self.last_response = serde_json::to_string_pretty(&payload)
                        .unwrap_or_else(|_| "Failed to format response".to_string());
                    self.rebuild_pagination();
                }
                Ok(Err(err)) => {
                    self.pending_request = None;
                    self.status = "Ready".to_string();
                    self.set_timed_status(format!("Request failed: {}", err), 5);
                    self.last_response.clear();
                    self.paginated_response = None;
                    self.wrap_metrics = None;
                    self.response_page = 0;
                    self.response_scroll = 0;
                    self.clear_response_search();
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending_request = None;
                    self.status = "Ready".to_string();
                    self.set_timed_status("Request failed: channel closed".to_string(), 5);
                }
            }
        }
    }

    pub fn current_method(&self) -> &MethodSpec {
        &self.methods[self.selected_method]
    }

    /// Sets transient status message for given seconds
    pub fn set_timed_status(&mut self, message: String, secs: u64) {
        self.timed_status = Some(TimedStatus {
            message,
            created: Instant::now(),
            duration_secs: secs,
        });
    }

    /// Returns active status text and clears expired timed status
    pub fn effective_status(&mut self) -> String {
        if let Some(ref ts) = self.timed_status {
            if ts.created.elapsed().as_secs() < ts.duration_secs {
                return ts.message.clone();
            }
        }
        self.timed_status = None;
        self.status.clone()
    }
}
