use crate::ui::palette::{ACCENT, MUTED};
use ratatui::{
    style::{Modifier, Style},
    text::Span,
    widgets::BorderType,
};

/// Returns a border style reflecting whether the pane is focused
pub fn highlight_if_focused(is_focused: bool) -> Style {
    if is_focused {
        focus_style()
    } else {
        unfocus_style()
    }
}

pub fn focus_style() -> Style {
    Style::default().fg(ACCENT)
}

pub fn unfocus_style() -> Style {
    Style::default().fg(MUTED).add_modifier(Modifier::DIM)
}

/// Returns a border type reflecting whether the pane is focused
pub fn highlight_border_type(is_focused: bool) -> BorderType {
    if is_focused {
        BorderType::Thick
    } else {
        BorderType::Rounded
    }
}

/// Returns a border style for panes when a modal overlay is active
pub fn pane_border_style(is_focused: bool, modal_active: bool) -> Style {
    if modal_active {
        unfocus_style()
    } else {
        highlight_if_focused(is_focused)
    }
}

/// Returns a border type for panes when a modal overlay is active
pub fn pane_border_type(is_focused: bool, modal_active: bool) -> BorderType {
    if modal_active {
        BorderType::Rounded
    } else {
        highlight_border_type(is_focused)
    }
}

/// Builds a shortcut hint with the first character underlined and the rest normal
pub fn shortcut_hint(label: &str) -> Vec<Span<'static>> {
    let mut chars = label.chars();
    let first = chars.next().unwrap_or_default();
    let rest: String = chars.collect();
    vec![
        Span::styled(
            first.to_string(),
            Style::default()
                .fg(ACCENT)
                .add_modifier(Modifier::UNDERLINED),
        ),
        Span::styled(rest, Style::default().fg(ACCENT)),
    ]
}

/// Builds an Escape shortcut hint with "Esc" underlined and "ape" normal
pub fn esc_hint() -> Vec<Span<'static>> {
    vec![
        Span::styled(
            "Esc",
            Style::default()
                .fg(ACCENT)
                .add_modifier(Modifier::UNDERLINED),
        ),
        Span::styled("ape", Style::default().fg(ACCENT)),
    ]
}
