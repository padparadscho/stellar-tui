use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::BorderType,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::ui::palette::{GREEN, MUTED, RED, TEXT, YELLOW};

use ratatui::style::Color;

/// Width threshold below which the header stacks title and status vertically
const HEADER_COMPACT_THRESHOLD: u16 = 80;

/// Braille spinner frames for the loading animation
const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Renders the header bar
///
/// - Wide: single row (title left, status right)
/// - Narrow: two rows (title on row 1, status on row 2, both centered)
pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(MUTED));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled(
            "Stellar RPC TUI",
            Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
        ),
    ]);

    let effective_status = app.effective_status();
    let (status_icon, status_color) = status_indicator(&effective_status, app.spinner_frame);

    // Advance spinner frame on every render tick when calling
    if effective_status.starts_with("Calling") {
        app.spinner_frame = (app.spinner_frame + 1) % SPINNER_FRAMES.len();
    }

    let status = Line::from(vec![Span::styled(
        format!("{} {} ", status_icon, effective_status),
        Style::default().fg(status_color),
    )]);

    let compact = area.width < HEADER_COMPACT_THRESHOLD;

    if compact {
        // Two row layout: title row 1, status row(s) 2+, status wraps if needed
        let status_rows = inner.height.saturating_sub(1).max(1);
        let [row1, status_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(status_rows)]).areas(inner);

        frame.render_widget(Paragraph::new(title).alignment(Alignment::Center), row1);
        frame.render_widget(
            Paragraph::new(status)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true }),
            status_area,
        );
    } else {
        // Single row layout: title left, status right
        let [left_area, right_area] =
            Layout::horizontal([Constraint::Min(20), Constraint::Min(0)]).areas(inner);

        frame.render_widget(Paragraph::new(title), left_area);
        frame.render_widget(
            Paragraph::new(status).alignment(Alignment::Right),
            right_area,
        );
    }
}

/// Returns an icon and color pair based on the current status text
fn status_indicator(status: &str, spinner_frame: usize) -> (&'static str, Color) {
    if status.starts_with("Calling") {
        (SPINNER_FRAMES[spinner_frame % SPINNER_FRAMES.len()], YELLOW)
    } else if status == "Completed!" {
        ("✓", GREEN)
    } else if status == "Data cleared!" {
        ("🗑", GREEN)
    } else if status == "Copied to clipboard!"
        || status == "Selection copied!"
        || status == "Current page copied!"
    {
        ("⧉", GREEN)
    } else if status.starts_with("Error")
        || status.starts_with("Request failed!")
        || status.starts_with("Params error.")
        || status.starts_with("Copy failed!")
        || status.starts_with("Failed to save settings.")
    {
        ("✕", RED)
    } else if status == "Ready" {
        ("●", MUTED)
    } else {
        ("●", TEXT)
    }
}
