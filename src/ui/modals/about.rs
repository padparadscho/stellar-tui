use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::ui::modals::layout::centered_rect;
use crate::ui::palette::{ACCENT, BLUE, MUTED};
use crate::ui::styles::esc_hint;

/// Renders the About modal
pub fn render(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(60, 60, frame.area());
    frame.render_widget(Clear, area);

    let mut esc_spans = vec![Span::raw(" ")];
    esc_spans.extend(esc_hint());
    esc_spans.push(Span::raw(" "));
    let esc_hint_line = Line::from(esc_spans);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" About ")
        .title(esc_hint_line.right_aligned())
        .border_style(Style::default().fg(ACCENT))
        .border_type(BorderType::Thick);

    let lines = vec![
        Line::from(""),
        Line::from(
            " Interactive terminal client for Stellar's JSON-RPC interface. Explore methods, build requests, and inspect responses.",
        ),
        Line::from(""),
        Line::from(Span::styled(
            " ─────────────────────────────────",
            Style::default().fg(MUTED),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(" Source:  ", Style::default().fg(MUTED)),
            Span::styled(
                env!("CARGO_PKG_REPOSITORY"),
                Style::default().fg(BLUE).add_modifier(Modifier::UNDERLINED),
            ),
        ]),
        Line::from(vec![
            Span::styled(" Version: ", Style::default().fg(MUTED)),
            Span::raw(env!("CARGO_PKG_VERSION")),
        ]),
        Line::from(vec![
            Span::styled(" License: ", Style::default().fg(MUTED)),
            Span::raw("MIT"),
        ]),
        Line::from(vec![
            Span::styled(" Author:  ", Style::default().fg(MUTED)),
            Span::raw(env!("CARGO_PKG_AUTHORS")),
        ]),
        Line::from(""),
    ];

    let inner_width = area.width.saturating_sub(2).max(1) as usize;
    let content_height: u16 = lines
        .iter()
        .map(|line| {
            let w = line.width();
            if w <= inner_width {
                1
            } else {
                w.div_ceil(inner_width) as u16
            }
        })
        .sum();
    let visible_height = area.height.saturating_sub(2);
    app.modal_max_scroll = content_height.saturating_sub(visible_height);
    if app.modal_scroll > app.modal_max_scroll {
        app.modal_scroll = app.modal_max_scroll;
    }

    frame.render_widget(
        Paragraph::new(Text::from(lines))
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((app.modal_scroll, 0)),
        area,
    );
}
