use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::ui::palette::{ACCENT, BLUE, MUTED, PEACH};
use crate::ui::modals::layout::centered_rect;
use crate::ui::styles::esc_hint;

/// Renders method documentation in a scrollable modal overlay
pub fn render(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(60, 70, frame.area());
    frame.render_widget(Clear, area);

    let mut esc_spans = vec![Span::raw(" ")];
    esc_spans.extend(esc_hint());
    esc_spans.push(Span::raw(" "));
    let esc_hint_line = Line::from(esc_spans);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Method Info ")
        .title(esc_hint_line.right_aligned())
        .border_style(Style::default().fg(ACCENT))
        .border_type(BorderType::Thick);

    let method = app.current_method();

    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("  {} ", method.http_method),
                Style::default().fg(PEACH).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                method.name,
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
    ];

    let description = method.help.replace('\n', " ");
    lines.push(Line::from(format!("  {}", description)));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  ─────────────────────────────────",
        Style::default().fg(MUTED),
    )));
    lines.push(Line::from(""));

    let docs_url = format!(
        "https://developers.stellar.org/docs/data/apis/rpc/api-reference/methods/{}",
        method.name
    );
    lines.push(Line::from(vec![
        Span::styled("  Learn more: ", Style::default().fg(MUTED)),
        Span::styled(
            docs_url,
            Style::default().fg(BLUE).add_modifier(Modifier::UNDERLINED),
        ),
    ]));

    lines.push(Line::from(""));

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
