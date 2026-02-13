use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::ui::layout::footer_rows;
use crate::ui::palette::{GREEN, MUTED};
use crate::ui::styles::shortcut_hint;

/// Renders the footer bar
///
/// - Wide: single row (info left, actions right)
/// - Narrow: two rows (Network on row 1, Settings + About + Quit on row 2, both centered)
pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(MUTED));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let compact = footer_rows(area.width) > 1;

    // Build reusable span groups
    let network = app
        .settings
        .active_network()
        .map(|p| p.name.clone())
        .unwrap_or_else(|| "none".to_string());

    let mut info_spans: Vec<Span> = vec![Span::raw(" ")];
    info_spans.extend(shortcut_hint("Network"));
    info_spans.push(Span::styled(": ", Style::default().fg(MUTED)));
    info_spans.push(Span::styled(network, Style::default().fg(GREEN)));

    let mut action_spans: Vec<Span> = Vec::new();
    action_spans.extend(shortcut_hint("Settings"));
    action_spans.push(Span::styled("  │  ", Style::default().fg(MUTED)));
    action_spans.extend(shortcut_hint("About"));
    action_spans.push(Span::styled("  │  ", Style::default().fg(MUTED)));
    action_spans.extend(shortcut_hint("Quit"));
    action_spans.push(Span::raw(" "));

    if compact {
        // Two row layout, both centered
        let [row1, row2] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(inner);

        frame.render_widget(
            Paragraph::new(Line::from(info_spans)).alignment(Alignment::Center),
            row1,
        );
        frame.render_widget(
            Paragraph::new(Line::from(action_spans)).alignment(Alignment::Center),
            row2,
        );
    } else {
        // Single row layout: info left, actions right
        let [left_area, right_area] =
            Layout::horizontal([Constraint::Min(0), Constraint::Length(34)]).areas(inner);

        frame.render_widget(Paragraph::new(Line::from(info_spans)), left_area);
        frame.render_widget(
            Paragraph::new(Line::from(action_spans)).alignment(Alignment::Right),
            right_area,
        );
    }
}
