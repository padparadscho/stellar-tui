use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::ui::modals::layout::centered_rect;
use crate::ui::palette::{ACCENT, GREEN, MUTED, RED, SUBTEXT};
use crate::ui::styles::{esc_hint, shortcut_hint};

fn display_endpoint(endpoint: &str) -> &str {
    endpoint
        .strip_prefix("https://")
        .or_else(|| endpoint.strip_prefix("http://"))
        .unwrap_or(endpoint)
}

/// Renders the Settings modal
pub fn render(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 70, frame.area());
    frame.render_widget(Clear, area);

    let mut esc_spans = vec![Span::raw(" ")];
    esc_spans.extend(esc_hint());
    esc_spans.push(Span::raw(" "));
    let esc_hint_line = Line::from(esc_spans);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Settings ")
        .title(esc_hint_line.right_aligned())
        .border_style(Style::default().fg(ACCENT))
        .border_type(BorderType::Thick);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let [content_area, _, hint_area] = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(inner);

    render_networks(frame, content_area, app);

    let hints = if app.network_editor.is_some() {
        let mut spans = vec![Span::raw(" ")];
        spans.extend(shortcut_hint("Save"));
        spans.push(Span::raw("  "));
        spans.extend(esc_hint());
        Line::from(spans)
    } else {
        let mut spans = vec![Span::raw(" ")];
        spans.extend(shortcut_hint("Add"));
        spans.push(Span::raw("  "));
        spans.extend(shortcut_hint("Edit"));
        spans.push(Span::raw("  "));
        spans.extend(shortcut_hint("Delete"));
        Line::from(spans)
    };

    frame.render_widget(
        Paragraph::new(hints).alignment(Alignment::Center),
        hint_area,
    );
}

/// Networks section
fn render_networks(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled(
            " Configure RPC endpoint connections",
            Style::default().fg(SUBTEXT),
        )),
        Line::from(""),
    ];

    if let Some(editor) = &app.network_editor {
        lines.push(Line::from(Span::styled(
            if editor.editing_index.is_some() {
                "  Edit Network"
            } else {
                "  Add Network"
            },
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));

        for (i, field) in editor.form.fields.iter().enumerate() {
            let label = field.spec.label;
            let is_selected = i == editor.form.selected;
            let cursor = if is_selected { "❯ " } else { "  " };
            let error = app.network_errors.get(field.spec.key);

            let value_text = if field.value.is_empty() {
                ""
            } else {
                &field.value
            };

            let value_style = if !field.value.is_empty() && field.value == field.spec.default_value
            {
                Style::default().fg(MUTED).add_modifier(Modifier::DIM)
            } else {
                Style::default()
            };

            let mut spans = vec![
                Span::styled(cursor, Style::default().fg(ACCENT)),
                Span::styled(format!("{}: ", label), Style::default().fg(MUTED)),
            ];

            spans.push(Span::styled(value_text, value_style));

            if let Some(err) = error {
                spans.push(Span::styled(
                    format!("  !! {}", err),
                    Style::default().fg(RED),
                ));
            }
            lines.push(Line::from(spans));
        }
    } else {
        for (i, network) in app.settings.networks.iter().enumerate() {
            let is_active = i == app.settings.active_network;
            let is_selected = i == app.selected_network;

            let marker = if is_active { "●" } else { "○" };
            let marker_style = if is_active {
                Style::default().fg(GREEN)
            } else {
                Style::default().fg(MUTED)
            };
            let name_style = if is_selected {
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let cursor = if is_selected { "❯ " } else { "  " };

            lines.push(Line::from(vec![
                Span::styled(cursor, Style::default().fg(ACCENT)),
                Span::styled(format!("{} ", marker), marker_style),
                Span::styled(&network.name, name_style),
            ]));
            lines.push(Line::from(Span::styled(
                format!("    {}", display_endpoint(&network.endpoint)),
                Style::default().fg(MUTED),
            )));
        }
    }

    frame.render_widget(
        Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false }),
        area,
    );
}
