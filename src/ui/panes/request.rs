use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::Frame;

use crate::app::{App, FocusPane, ModalState};
use crate::ui::components::form;
use crate::ui::styles::{esc_hint, shortcut_hint};

/// Renders the request form for the currently selected method
pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let errors = app.active_errors().clone();
    let focused = app.focus == FocusPane::Request;

    let is_fullscreen = app.zoomed_pane == Some(FocusPane::Request);
    let modal_active = app.modal != ModalState::None;

    let hint = if modal_active {
        None
    } else if is_fullscreen {
        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span::raw(" "));
        spans.extend(shortcut_hint("Run"));
        spans.push(Span::raw(" "));
        spans.extend(shortcut_hint("Purge"));
        spans.push(Span::raw(" "));
        spans.extend(esc_hint());
        spans.push(Span::raw(" "));
        Some(Line::from(spans))
    } else if focused {
        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span::raw(" "));
        spans.extend(shortcut_hint("Run"));
        spans.push(Span::raw(" "));
        spans.extend(shortcut_hint("Purge"));
        spans.push(Span::raw(" "));
        spans.extend(shortcut_hint("Fullscreen"));
        spans.push(Span::raw(" "));
        Some(Line::from(spans))
    } else {
        None
    };

    form::render(
        frame,
        area,
        &app.active_request_title(),
        app.active_request_form_mut(),
        focused,
        &errors,
        hint,
        modal_active,
    );
}
