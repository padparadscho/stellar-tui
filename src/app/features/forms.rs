use std::collections::HashMap;

use crate::app::core::forms::FormState;
use crate::app::core::state::App;
use crate::app::methods::MethodId;

impl App {
    /// Returns mutable request form for selected method
    pub fn active_request_form_mut(&mut self) -> &mut FormState {
        &mut self.request_forms[self.selected_method]
    }

    /// Checks whether selected request field can be edited now
    pub fn is_selected_request_editable(&self) -> bool {
        if self.current_method().id != MethodId::GetEvents {
            return true;
        }
        let form = &self.request_forms[self.selected_method];
        let selected_key = form
            .fields
            .get(form.selected)
            .map(|field| field.spec.key)
            .unwrap_or("");
        if selected_key != "contractIds" && selected_key != "topics" {
            return true;
        }
        self.is_get_events_type_enabled()
    }

    /// Checks whether GetEvents type enables gated fields
    pub fn is_get_events_type_enabled(&self) -> bool {
        if self.current_method().id != MethodId::GetEvents {
            return true;
        }
        let form = &self.request_forms[self.selected_method];
        form.field_value("type")
            .map(|value| {
                let normalized = value.trim().to_ascii_lowercase();
                normalized == "system" || normalized == "contract"
            })
            .unwrap_or(false)
    }

    /// Checks whether given request field index is enabled
    pub fn is_request_field_enabled(&self, field_index: usize) -> bool {
        if self.current_method().id != MethodId::GetEvents {
            return true;
        }
        let form = &self.request_forms[self.selected_method];
        let key = form
            .fields
            .get(field_index)
            .map(|field| field.spec.key)
            .unwrap_or("");
        if key != "contractIds" && key != "topics" {
            return true;
        }
        self.is_get_events_type_enabled()
    }

    /// Selects next enabled request field
    pub(in crate::app) fn select_next_request_field(&mut self) {
        let form_len = self.request_forms[self.selected_method].fields.len();
        if form_len == 0 {
            return;
        }

        let start = self.request_forms[self.selected_method].selected;
        let mut index = start;
        for _ in 0..form_len {
            index = (index + 1).min(form_len - 1);
            if self.is_request_field_enabled(index) {
                self.request_forms[self.selected_method].selected = index;
                return;
            }
            if index == form_len - 1 {
                break;
            }
        }
    }

    /// Selects previous enabled request field
    pub(in crate::app) fn select_prev_request_field(&mut self) {
        let form_len = self.request_forms[self.selected_method].fields.len();
        if form_len == 0 {
            return;
        }

        let start = self.request_forms[self.selected_method].selected;
        let mut index = start;
        for _ in 0..form_len {
            if index == 0 {
                break;
            }
            index -= 1;
            if self.is_request_field_enabled(index) {
                self.request_forms[self.selected_method].selected = index;
                return;
            }
        }
    }

    /// Returns validation errors for selected request form
    pub fn active_errors(&self) -> &HashMap<String, String> {
        &self.method_errors[self.selected_method]
    }

    /// Returns title for request pane
    pub fn active_request_title(&self) -> String {
        "Request".to_string()
    }

    /// Selects next method in list
    pub(in crate::app) fn select_next_method(&mut self) {
        if self.selected_method + 1 < self.methods.len() {
            self.selected_method += 1;
        }
    }

    /// Selects previous method in list
    pub(in crate::app) fn select_prev_method(&mut self) {
        if self.selected_method > 0 {
            self.selected_method -= 1;
        }
    }

    /// Refreshes validation errors for selected method
    pub(in crate::app) fn refresh_active_errors(&mut self) {
        self.refresh_method_errors(self.selected_method);
    }

    /// Refreshes validation errors for a method index
    pub(in crate::app) fn refresh_method_errors(&mut self, index: usize) {
        if let Some(method) = self.methods.get(index) {
            let form = &self.request_forms[index];
            self.method_errors[index] = method.validate(form);
        }
    }
}
