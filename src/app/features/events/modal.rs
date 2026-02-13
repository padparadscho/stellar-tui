use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, AppCommand, ModalState};

impl App {
    /// Handles keys while a modal is open
    pub(in crate::app::features::events) fn handle_modal_key(
        &mut self,
        key: KeyEvent,
    ) -> Option<AppCommand> {
        match key.code {
            KeyCode::Esc => {
                // Esc exits the inline editor first, then the modal on a second press
                if self.modal == ModalState::Settings && self.network_editor.is_some() {
                    self.network_editor = None;
                    self.set_timed_status("Network edit canceled".to_string(), 5);
                    return None;
                }
                self.modal = ModalState::None;
                self.modal_scroll = 0;
                None
            }
            KeyCode::Char('q') => {
                self.modal = ModalState::None;
                self.modal_scroll = 0;
                None
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.modal == ModalState::Settings && self.network_editor.is_none() {
                    self.select_prev_network();
                } else if self.modal == ModalState::Settings && self.network_editor.is_some() {
                    return self.handle_network_edit_key(key);
                } else {
                    self.modal_scroll = self.modal_scroll.saturating_sub(1);
                }
                None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.modal == ModalState::Settings && self.network_editor.is_none() {
                    self.select_next_network();
                } else if self.modal == ModalState::Settings && self.network_editor.is_some() {
                    return self.handle_network_edit_key(key);
                } else if self.modal_scroll < self.modal_max_scroll {
                    self.modal_scroll = self.modal_scroll.saturating_add(1);
                }
                None
            }
            KeyCode::Home => {
                self.modal_scroll = 0;
                None
            }
            KeyCode::End => {
                self.modal_scroll = self.modal_max_scroll;
                None
            }
            _ => {
                // Non navigation settings keys are handled by the settings action map
                if self.modal == ModalState::Settings {
                    self.handle_settings_modal_key(key)
                } else {
                    None
                }
            }
        }
    }

    /// Handles keys in the settings modal content
    pub(in crate::app::features::events) fn handle_settings_modal_key(
        &mut self,
        key: KeyEvent,
    ) -> Option<AppCommand> {
        if self.network_editor.is_some() {
            return self.handle_network_edit_key(key);
        }

        match key.code {
            KeyCode::Char('a') => {
                self.start_network_editor(None);
                None
            }
            KeyCode::Char('e') => {
                self.start_network_editor(Some(self.selected_network));
                None
            }
            KeyCode::Char('d') => {
                self.delete_network();
                None
            }
            _ => None,
        }
    }
}
