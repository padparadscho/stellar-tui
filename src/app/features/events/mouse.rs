use crossterm::event::{MouseEvent, MouseEventKind};

use crate::app::{App, FocusPane, ModalState, UiRegions};

use super::focus::contains_point;

/// Handles mouse input for focus and scrolling
pub(super) fn handle_mouse(app: &mut App, event: MouseEvent) {
    app.handle_mouse_inner(event);
}

impl App {
    fn handle_mouse_inner(&mut self, event: MouseEvent) {
        // Modal mode remaps wheel input to modal content or settings list navigation
        if self.modal != ModalState::None {
            match event.kind {
                MouseEventKind::ScrollUp => {
                    if self.modal == ModalState::Settings {
                        if let Some(editor) = &mut self.network_editor {
                            editor.form.select_prev();
                        } else {
                            self.select_prev_network();
                        }
                    } else {
                        self.modal_scroll = self.modal_scroll.saturating_sub(1);
                    }
                }
                MouseEventKind::ScrollDown => {
                    if self.modal == ModalState::Settings {
                        if let Some(editor) = &mut self.network_editor {
                            editor.form.select_next();
                        } else {
                            self.select_next_network();
                        }
                    } else if self.modal_scroll < self.modal_max_scroll {
                        self.modal_scroll = self.modal_scroll.saturating_add(1);
                    }
                }
                _ => {}
            }
            return;
        }

        // Zoom keeps interactions scoped to the active fullscreen pane
        if self.zoomed_pane.is_some() {
            match event.kind {
                MouseEventKind::Down(_) => {
                    if self.zoomed_pane == Some(FocusPane::Response)
                        || self.focus == FocusPane::Response
                    {
                        self.begin_response_selection_from_mouse(event.column, event.row);
                    }
                }
                MouseEventKind::Drag(_) => {
                    if self.zoomed_pane == Some(FocusPane::Response)
                        || self.focus == FocusPane::Response
                    {
                        self.update_response_selection_from_mouse(event.column, event.row);
                    }
                }
                MouseEventKind::Up(_) => {
                    if self.zoomed_pane == Some(FocusPane::Response)
                        || self.focus == FocusPane::Response
                    {
                        self.finish_response_selection();
                    }
                }
                MouseEventKind::ScrollUp => {
                    if self.zoomed_pane == Some(FocusPane::Response)
                        || self.focus == FocusPane::Response
                    {
                        self.scroll_response(-1);
                    } else if self.focus == FocusPane::Request {
                        self.select_prev_request_field();
                    }
                }
                MouseEventKind::ScrollDown => {
                    if self.zoomed_pane == Some(FocusPane::Response)
                        || self.focus == FocusPane::Response
                    {
                        self.scroll_response(1);
                    } else if self.focus == FocusPane::Request {
                        self.select_next_request_field();
                    }
                }
                _ => {}
            }
            return;
        }

        let regions = match self.ui_regions {
            Some(regions) => regions,
            None => return,
        };

        // Normal layout uses region hit testing for focus and scroll routing
        match event.kind {
            MouseEventKind::ScrollUp => self.handle_scroll(regions, event.column, event.row, -1),
            MouseEventKind::ScrollDown => self.handle_scroll(regions, event.column, event.row, 1),
            MouseEventKind::Down(_) => {
                self.handle_click_focus(regions, event.column, event.row);
                if contains_point(regions.response, event.column, event.row) {
                    self.begin_response_selection_from_mouse(event.column, event.row);
                }
            }
            MouseEventKind::Drag(_) => {
                if contains_point(regions.response, event.column, event.row) {
                    self.update_response_selection_from_mouse(event.column, event.row);
                }
            }
            MouseEventKind::Up(_) => {
                self.finish_response_selection();
            }
            _ => {}
        }
    }

    /// Updates focus from pointer position
    fn handle_click_focus(&mut self, regions: UiRegions, column: u16, row: u16) {
        if contains_point(regions.methods, column, row) {
            self.focus = FocusPane::Methods;
        } else if contains_point(regions.request, column, row) {
            self.focus = FocusPane::Request;
        } else if contains_point(regions.response, column, row) {
            self.focus = FocusPane::Response;
        }

        if self.focus != FocusPane::Response {
            self.clear_response_selection();
        }
    }

    /// Applies wheel scroll in the focused region
    fn handle_scroll(&mut self, regions: UiRegions, column: u16, row: u16, delta: i16) {
        if contains_point(regions.methods, column, row) {
            self.focus = FocusPane::Methods;
            self.clear_response_selection();
            if delta < 0 {
                self.select_prev_method();
            } else {
                self.select_next_method();
            }
        } else if contains_point(regions.request, column, row) {
            self.focus = FocusPane::Request;
            self.clear_response_selection();
            if delta < 0 {
                self.select_prev_request_field();
            } else {
                self.select_next_request_field();
            }
            self.refresh_active_errors();
        } else if contains_point(regions.response, column, row) {
            self.focus = FocusPane::Response;
            self.scroll_response(delta);
        }
    }
}
