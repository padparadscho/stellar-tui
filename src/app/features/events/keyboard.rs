use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, AppCommand, FocusPane, ModalState};

use super::focus::{next_focus, prev_focus};

pub(super) fn handle_key(app: &mut App, key: KeyEvent) -> Option<AppCommand> {
    // Modal shortcuts win over all pane interactions
    if app.modal != ModalState::None {
        return app.handle_modal_key(key);
    }

    // Network editor captures input before global shortcuts
    if app.network_editor.is_some() {
        return app.handle_network_edit_key(key);
    }

    match key.code {
        KeyCode::Tab | KeyCode::BackTab => {
            // While search is active, tab cycles matches instead of changing pane focus
            if app.is_response_search_enabled() {
                if key.code == KeyCode::Tab {
                    app.next_search_match();
                } else {
                    app.prev_search_match();
                }
                return None;
            }
            if app.zoomed_pane.is_some() {
                return None;
            }
            if key.code == KeyCode::Tab {
                app.focus = next_focus(app.focus);
            } else {
                app.focus = prev_focus(app.focus);
            }
            if app.focus != FocusPane::Response {
                app.clear_response_selection();
            }
            None
        }
        KeyCode::Up => {
            handle_up(app);
            None
        }
        KeyCode::Down => {
            handle_down(app);
            None
        }
        KeyCode::Left => {
            if app.is_response_search_enabled() {
                app.move_search_cursor_left();
                return None;
            }
            if app.focus == FocusPane::Request {
                if !app.is_selected_request_editable() {
                    app.set_timed_status("Set Event type before editing this field".to_string(), 3);
                    return None;
                }
                app.active_request_form_mut().cursor_left();
                return None;
            }
            if app.focus == FocusPane::Response || app.zoomed_pane == Some(FocusPane::Response) {
                app.prev_response_page();
            }
            None
        }
        KeyCode::Right => {
            if app.is_response_search_enabled() {
                app.move_search_cursor_right();
                return None;
            }
            if app.focus == FocusPane::Request {
                if !app.is_selected_request_editable() {
                    app.set_timed_status("Set Event type before editing this field".to_string(), 3);
                    return None;
                }
                app.active_request_form_mut().cursor_right();
                return None;
            }
            if app.focus == FocusPane::Response || app.zoomed_pane == Some(FocusPane::Response) {
                app.next_response_page();
            }
            None
        }
        KeyCode::Home => {
            if app.focus == FocusPane::Response || app.zoomed_pane == Some(FocusPane::Response) {
                app.jump_response_start();
            }
            None
        }
        KeyCode::End => {
            if app.focus == FocusPane::Response || app.zoomed_pane == Some(FocusPane::Response) {
                app.jump_response_end();
            }
            None
        }
        KeyCode::Esc => {
            if app.zoomed_pane.is_some() {
                app.clear_response_search();
                app.clear_response_selection();
            }
            app.zoomed_pane = None;
            None
        }
        KeyCode::Backspace => {
            if app.is_response_search_enabled() {
                app.search_backspace();
                app.update_response_search_matches();
                if !app.response_search_matches.is_empty() {
                    app.scroll_to_current_match();
                }
                return None;
            }
            if app.focus == FocusPane::Request {
                if !app.is_selected_request_editable() {
                    app.set_timed_status("Set Event type before editing this field".to_string(), 3);
                    return None;
                }
                app.active_request_form_mut().backspace();
                app.refresh_active_errors();
            }
            None
        }
        KeyCode::Delete => {
            if app.is_response_search_enabled() {
                app.search_delete_forward();
                app.update_response_search_matches();
                if !app.response_search_matches.is_empty() {
                    app.scroll_to_current_match();
                }
                return None;
            }
            if app.focus == FocusPane::Request {
                if !app.is_selected_request_editable() {
                    app.set_timed_status("Set Event type before editing this field".to_string(), 3);
                    return None;
                }
                app.active_request_form_mut().delete_forward();
                app.refresh_active_errors();
            }
            None
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if app.focus == FocusPane::Response || app.zoomed_pane == Some(FocusPane::Response) {
                app.copy_response_selection_or_page();
            }
            None
        }
        KeyCode::Char(c) => {
            // Plain c remains a quick copy gesture in response views
            if c == 'c'
                && !key.modifiers.contains(KeyModifiers::CONTROL)
                && (app.focus == FocusPane::Response
                    || app.zoomed_pane == Some(FocusPane::Response))
            {
                app.copy_response_selection_or_page();
                return None;
            }
            if app.is_response_search_enabled() && !key.modifiers.contains(KeyModifiers::CONTROL) {
                app.insert_search_char(c);
                app.update_response_search_matches();
                if !app.response_search_matches.is_empty() {
                    app.scroll_to_current_match();
                }
                return None;
            }
            if app.focus == FocusPane::Request && !key.modifiers.contains(KeyModifiers::CONTROL) {
                if !app.is_selected_request_editable() {
                    app.set_timed_status("Set Event type before editing this field".to_string(), 3);
                    return None;
                }
                app.active_request_form_mut().insert_char(c);
                app.refresh_active_errors();
                return None;
            }
            handle_char_command(app, c)
        }
        _ => None,
    }
}

/// Handles character shortcuts outside field editing
fn handle_char_command(app: &mut App, c: char) -> Option<AppCommand> {
    match c {
        'q' => Some(AppCommand::Quit),
        'r' if app.focus != FocusPane::Response => Some(AppCommand::SendRequest),
        'f' => {
            toggle_fullscreen(app);
            None
        }
        'p' => {
            handle_purge(app);
            None
        }
        'c' => {
            if app.focus == FocusPane::Response || app.zoomed_pane == Some(FocusPane::Response) {
                app.copy_response_selection_or_page();
            }
            None
        }
        'a' => {
            app.modal = ModalState::About;
            app.modal_scroll = 0;
            None
        }
        's' => {
            app.modal = ModalState::Settings;
            app.modal_scroll = 0;
            None
        }
        'i' if app.focus == FocusPane::Methods => {
            app.modal = ModalState::Info;
            app.modal_scroll = 0;
            None
        }
        'n' => {
            cycle_network(app);
            None
        }
        _ => None,
    }
}

/// Clears active request data in allowed panes
fn handle_purge(app: &mut App) {
    let allowed = match app.focus {
        FocusPane::Request => true,
        FocusPane::Response => app.zoomed_pane.is_none(),
        _ => false,
    };
    if allowed {
        app.purge_data();
    }
}

/// Moves active network to next configured entry
fn cycle_network(app: &mut App) {
    if app.settings.networks.is_empty() {
        return;
    }
    let next = (app.settings.active_network + 1) % app.settings.networks.len();
    app.settings.set_active_network(next);
    app.selected_network = next;
    if let Err(err) = app.settings.save() {
        app.set_timed_status(format!("Failed to save settings: {}", err), 5);
    } else if let Some(network) = app.settings.active_network() {
        app.set_timed_status(format!("Switched to {}", network.name), 5);
    }
}

/// Toggles fullscreen for request and response panes
fn toggle_fullscreen(app: &mut App) {
    if app.zoomed_pane.is_some() {
        app.clear_response_search();
        app.clear_response_selection();
        app.zoomed_pane = None;
    } else {
        match app.focus {
            FocusPane::Request | FocusPane::Response => {
                app.zoomed_pane = Some(app.focus);
            }
            _ => {}
        }
    }
}

/// Applies upward navigation for current focus
fn handle_up(app: &mut App) {
    match app.focus {
        FocusPane::Methods => app.select_prev_method(),
        FocusPane::Request => app.select_prev_request_field(),
        FocusPane::Response => app.scroll_response(-1),
    }
    app.refresh_active_errors();
}

/// Applies downward navigation for current focus
fn handle_down(app: &mut App) {
    match app.focus {
        FocusPane::Methods => app.select_next_method(),
        FocusPane::Request => app.select_next_request_field(),
        FocusPane::Response => app.scroll_response(1),
    }
    app.refresh_active_errors();
}
