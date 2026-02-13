use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::ui::palette::{ACCENT, MUTED};

/// Renders the response search box below the response pane in fullscreen mode
///
/// - When the response is empty, shows a disabled/dimmed placeholder
/// - When the response has content, shows the search query with a blinking cursor and match count
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let enabled = app.is_response_search_enabled();
    let is_searching = enabled && !app.response_search_query.is_empty();

    let border_style = if is_searching {
        Style::default().fg(ACCENT)
    } else {
        Style::default().fg(MUTED).add_modifier(Modifier::DIM)
    };

    let border_type = if is_searching {
        BorderType::Thick
    } else {
        BorderType::Rounded
    };

    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(border_style);

    if !enabled {
        // Disabled state: dimmed placeholder text
        let text = Line::from(vec![
            Span::styled(
                "Search: ",
                Style::default().fg(MUTED).add_modifier(Modifier::DIM),
            ),
            Span::styled(
                "Nothing to search",
                Style::default()
                    .fg(MUTED)
                    .add_modifier(Modifier::DIM | Modifier::ITALIC),
            ),
        ]);

        frame.render_widget(Paragraph::new(text).block(block), area);
        return;
    }

    // Build search line text
    let mut spans = vec![Span::styled("Search: ", Style::default().fg(MUTED))];

    if app.response_search_query.is_empty() {
        spans.push(Span::styled("", Style::default().fg(MUTED)));
    } else {
        spans.push(Span::raw(app.response_search_query.clone()));
    }

    let left = Line::from(spans);

    // Match count indicator on the right side (only when there's a query)
    if is_searching {
        let (right_text, right_style) = if app.response_search_matches.is_empty() {
            (
                "0 matches".to_string(),
                Style::default().fg(MUTED).add_modifier(Modifier::DIM),
            )
        } else {
            (
                format!(
                    "{}/{}",
                    app.response_search_current + 1,
                    app.response_search_matches.len()
                ),
                Style::default().fg(ACCENT),
            )
        };

        block = block.title(
            Line::from(Span::styled(format!(" {} ", right_text), right_style)).right_aligned(),
        );
    }

    frame.render_widget(Paragraph::new(left).block(block), area);
}
