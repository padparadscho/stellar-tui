use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use std::collections::HashMap;

use ratatui::widgets::BorderType;

use crate::app::core::forms::{FieldType, FormState};
use crate::ui::palette::{ACCENT, MUTED, RED, YELLOW};
use crate::ui::styles::{focus_style, unfocus_style};

/// Height reserved for the detail area at the bottom (separator + content)
const DETAIL_HEIGHT: u16 = 3;

/// Renders an interactive form as a selectable list with a detail area
///
/// - When focused, the bottom of the pane shows the selected field's type, description, and any validation error
/// - When unfocused, the full area is used for the field list
#[allow(clippy::too_many_arguments)]
pub fn render(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    form: &mut FormState,
    focused: bool,
    errors: &HashMap<String, String>,
    hint: Option<Line<'static>>,
    modal_active: bool,
) {
    let border_style = if modal_active && focused {
        Style::default().fg(ACCENT).add_modifier(Modifier::DIM)
    } else if focused {
        focus_style()
    } else {
        unfocus_style()
    };
    let border_type = if modal_active {
        BorderType::Rounded
    } else if focused {
        BorderType::Thick
    } else {
        BorderType::Rounded
    };
    let mut block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", title))
        .border_style(border_style)
        .border_type(border_type)
        .title_style(border_style);

    if let Some(h) = hint {
        block = block.title(h.right_aligned());
    }

    // Compute inner area, then split for detail when focused
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let show_detail = focused && !form.fields.is_empty();
    let (list_area, detail_area) = if show_detail && inner.height > DETAIL_HEIGHT + 2 {
        let [top, bottom] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(DETAIL_HEIGHT)]).areas(inner);
        (top, Some(bottom))
    } else {
        (inner, None)
    };

    let content_height = list_area.height as usize;
    form.ensure_visible(content_height);

    // Field list
    let items: Vec<ListItem> = if form.fields.is_empty() {
        vec![ListItem::new(Span::styled(
            "No parameters",
            Style::default().fg(MUTED),
        ))]
    } else {
        form.fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let is_selected = i == form.selected;
                let has_event_type = form
                    .field_value("type")
                    .map(|value| {
                        let normalized = value.trim().to_ascii_lowercase();
                        normalized == "system" || normalized == "contract"
                    })
                    .unwrap_or(false);
                let is_locked =
                    matches!(field.spec.key, "contractIds" | "topics") && !has_event_type;

                let label_style = if is_locked {
                    Style::default().fg(MUTED).add_modifier(Modifier::DIM)
                } else if is_selected && focused {
                    Style::default().fg(ACCENT)
                } else {
                    Style::default().fg(MUTED)
                };

                let value_style = if is_locked || field.value == field.spec.default_value {
                    Style::default().fg(MUTED).add_modifier(Modifier::DIM)
                } else {
                    Style::default()
                };

                let value_span = if field.value.is_empty() {
                    // No placeholder for unselected empty fields
                    Span::raw("")
                } else {
                    Span::styled(field.value.clone(), value_style)
                };

                let mut spans = vec![
                    Span::styled(field.spec.label, label_style),
                    Span::styled(": ", label_style),
                ];
                spans.push(value_span);

                ListItem::new(Line::from(spans))
            })
            .collect()
    };

    let list = List::new(items)
        .highlight_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .highlight_symbol("❯ ");

    let mut state = ListState::default();
    if !form.fields.is_empty() {
        state.select(Some(form.selected));
        *state.offset_mut() = form.scroll;
    }

    frame.render_stateful_widget(list, list_area, &mut state);

    // Detail area (type + description + error)
    if let Some(detail) = detail_area {
        if let Some(field) = form.fields.get(form.selected) {
            // Dim horizontal separator
            let sep = Line::from(Span::styled(
                "─".repeat(detail.width as usize),
                Style::default().fg(MUTED),
            ));
            let [sep_area, content_area] =
                Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(detail);
            frame.render_widget(Paragraph::new(sep), sep_area);

            let type_label = match field.spec.field_type {
                FieldType::Text => "text",
                FieldType::Number => "num",
                FieldType::List => "list",
                FieldType::Json => "json",
            };

            let mut lines: Vec<Line> = Vec::new();

            // Line 1: [type] + description
            let mut desc_spans = vec![Span::styled(
                format!("[{}]", type_label),
                Style::default().fg(YELLOW).add_modifier(Modifier::DIM),
            )];
            if !field.spec.hint.is_empty() {
                desc_spans.push(Span::raw(format!(" {}", field.spec.hint)));
            }
            lines.push(Line::from(desc_spans));

            // Line 2: validation error (if any)
            if let Some(error) = errors.get(field.spec.key) {
                // "Required" errors are red; format/value errors are yellow
                let color = if error == "Required" { RED } else { YELLOW };
                lines.push(Line::from(Span::styled(
                    error.clone(),
                    Style::default().fg(color),
                )));
            }

            frame.render_widget(
                Paragraph::new(lines).wrap(Wrap { trim: true }),
                content_area,
            );
        }
    }
}
