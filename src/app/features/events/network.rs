use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, AppCommand};

impl App {
    /// Handles key input while editing a network entry
    pub(in crate::app::features::events) fn handle_network_edit_key(
        &mut self,
        key: KeyEvent,
    ) -> Option<AppCommand> {
        match key.code {
            KeyCode::Esc => {
                // Esc always abandons unsaved editor state
                self.network_editor = None;
                self.set_timed_status("Network edit canceled".to_string(), 5);
                None
            }
            KeyCode::Up => {
                if let Some(editor) = &mut self.network_editor {
                    editor.form.select_prev();
                }
                None
            }
            KeyCode::Down => {
                if let Some(editor) = &mut self.network_editor {
                    editor.form.select_next();
                }
                None
            }
            KeyCode::Backspace => {
                if let Some(editor) = &mut self.network_editor {
                    editor.form.backspace();
                    self.refresh_network_errors();
                }
                None
            }
            KeyCode::Delete => {
                if let Some(editor) = &mut self.network_editor {
                    editor.form.delete_forward();
                    self.refresh_network_errors();
                }
                None
            }
            KeyCode::Left => {
                if let Some(editor) = &mut self.network_editor {
                    editor.form.cursor_left();
                }
                None
            }
            KeyCode::Right => {
                if let Some(editor) = &mut self.network_editor {
                    editor.form.cursor_right();
                }
                None
            }
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                // Save shortcut is local to the editor and does not close the modal
                self.save_network_editor();
                None
            }
            KeyCode::Char(c) => {
                if let Some(editor) = &mut self.network_editor {
                    editor.form.insert_char(c);
                    self.refresh_network_errors();
                }
                None
            }
            _ => None,
        }
    }
}
