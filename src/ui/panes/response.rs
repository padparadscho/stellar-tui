use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, FocusPane, ModalState};
use ratatui::style::Modifier;

use crate::ui::palette::{ACCENT, BLUE, GREEN, MUTED, PEACH, SURFACE, YELLOW};
use crate::ui::styles::{esc_hint, pane_border_style, pane_border_type, shortcut_hint};

/// Renders the response body with vertical scrolling and pagination
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == FocusPane::Response;
    let is_fullscreen = app.zoomed_pane == Some(FocusPane::Response);
    let modal_active = app.modal != ModalState::None;
    let mut block = Block::default()
        .borders(Borders::ALL)
        .title(" Response ")
        .border_style(pane_border_style(focused, modal_active))
        .border_type(pane_border_type(focused, modal_active))
        .title_style(pane_border_style(focused, modal_active));

    // Right aligned action hints on the top border
    if is_fullscreen {
        // Fullscreen: Copy + Escape
        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span::raw(" "));
        spans.extend(shortcut_hint("Copy"));
        spans.push(Span::raw(" "));
        spans.extend(esc_hint());
        spans.push(Span::raw(" "));
        block = block.title(Line::from(spans).right_aligned());
    } else if focused && !modal_active {
        // Normal mode: Copy + Purge + Fullscreen
        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span::raw(" "));
        spans.extend(shortcut_hint("Copy"));
        spans.push(Span::raw(" "));
        spans.extend(shortcut_hint("Purge"));
        spans.push(Span::raw(" "));
        spans.extend(shortcut_hint("Fullscreen"));
        spans.push(Span::raw(" "));
        block = block.title(Line::from(spans).right_aligned());
    }

    // Page indicator on the bottom border (always shown when a response exists)
    if let Some(ref pag) = app.paginated_response {
        let current = app.response_page + 1;
        let total = pag.total_pages;
        let page_info = if total == 1 {
            " Page 1/1 ".to_string()
        } else if current == 1 {
            format!(" Page {}/{} ► ", current, total)
        } else if current == total {
            format!(" ◄ Page {}/{} ", current, total)
        } else {
            format!(" ◄ Page {}/{} ► ", current, total)
        };
        block = block.title_bottom(
            Line::from(Span::styled(page_info, Style::default().fg(MUTED))).right_aligned(),
        );
    }

    if app.last_response.is_empty() {
        let text = "No response yet".to_string();
        let style = Style::default().fg(MUTED);
        frame.render_widget(
            Paragraph::new(text)
                .block(block)
                .style(style)
                .wrap(Wrap { trim: false })
                .scroll((app.response_scroll, 0)),
            area,
        );
        return;
    }

    // Use the current page text instead of the full response
    let page_text = app.current_page_text();

    let has_search = is_fullscreen && !app.response_search_query.is_empty();
    let text = if let Some((start, end)) = app.response_selection_range() {
        selection_highlight_text(&app.wrapped_page_lines(), start, end)
    } else if has_search {
        colorize_and_highlight_json_text(&page_text, &app.response_search_query)
    } else {
        colorize_json_text(&page_text)
    };

    frame.render_widget(
        Paragraph::new(text)
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((app.response_scroll, 0)),
        area,
    );
}

/// Colorizes a JSON response with syntax highlighting
///
/// - JSON keys: BLUE
/// - Strings containing `…`: PEACH
/// - String values: GREEN
/// - Numbers / booleans: PEACH
/// - Null: MUTED
/// - Structural characters ({, }, [, ], commas, colons): MUTED
fn colorize_json_text(content: &str) -> Text<'static> {
    let lines: Vec<Line> = content.lines().map(colorize_json_line).collect();
    Text::from(lines)
}

fn colorize_and_highlight_json_text(content: &str, query: &str) -> Text<'static> {
    let colored = colorize_json_text(content);
    if query.trim().is_empty() {
        return colored;
    }
    mark_matching_lines(colored, query)
}

fn mark_matching_lines(text: Text<'static>, query: &str) -> Text<'static> {
    let query_lower = query.to_lowercase();
    let lines: Vec<Line<'static>> = text
        .lines
        .into_iter()
        .map(|line| mark_line_if_matching(line, &query_lower))
        .collect();

    Text::from(lines)
}

fn mark_line_if_matching(line: Line<'static>, query_lower: &str) -> Line<'static> {
    let raw: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
    let raw_lower = raw.to_lowercase();
    if !raw_lower.contains(query_lower) {
        return line;
    }

    let mut spans = Vec::with_capacity(line.spans.len() + 1);
    spans.push(Span::styled(
        "› ",
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
    ));
    spans.extend(line.spans);

    Line::from(spans)
}

/// Colorizes a single JSON line into styled spans
fn colorize_json_line(line: &str) -> Line<'static> {
    let trimmed = line.trim();

    if trimmed.is_empty() {
        return Line::raw(line.to_string());
    }

    let leading_ws = &line[..line.len() - trimmed.len()];

    if matches!(trimmed, "{" | "}" | "[" | "]" | "}," | "]," | "{}" | "[]") {
        return Line::from(vec![
            Span::raw(leading_ws.to_string()),
            Span::styled(trimmed.to_string(), Style::default().fg(MUTED)),
        ]);
    }

    // Try to parse as a JSON key-value pair: "key": value
    if let Some(colon_pos) = find_key_colon(trimmed) {
        let key_part = &trimmed[..colon_pos]; // includes the "key" with quotes
        let separator = ": ";
        let value_part = &trimmed[colon_pos + 2..]; // after ": "

        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span::raw(leading_ws.to_string()));
        spans.push(Span::styled(
            key_part.to_string(),
            Style::default().fg(BLUE),
        ));
        spans.push(Span::styled(
            separator.to_string(),
            Style::default().fg(MUTED),
        ));
        spans.extend(colorize_json_value(value_part));

        return Line::from(spans);
    }

    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::raw(leading_ws.to_string()));
    spans.extend(colorize_json_value(trimmed));
    Line::from(spans)
}

/// Finds the position of `": ` that separates a JSON key from its value
///
/// Returns the byte offset of the colon in the trimmed string
fn find_key_colon(trimmed: &str) -> Option<usize> {
    // A JSON key-value line starts with `"key": `
    if !trimmed.starts_with('"') {
        return None;
    }
    let after_first_quote = &trimmed[1..];
    let close_quote = after_first_quote.find('"')?;
    let after_key = &trimmed[close_quote + 2..]; // after closing quote
    if after_key.starts_with(": ") {
        Some(close_quote + 2) // position of ':'
    } else {
        None
    }
}

/// Colorizes a JSON value portion (after `"key": `)
fn colorize_json_value(value: &str) -> Vec<Span<'static>> {
    let v = value.trim_end_matches(',');
    let has_comma = value.len() > v.len();
    let comma = if has_comma { "," } else { "" };

    if v.starts_with('"') && v.ends_with('"') && v.len() >= 2 {
        let inner = &v[1..v.len() - 1];
        let color = if inner.contains('\u{2026}') {
            PEACH
        } else {
            GREEN
        };
        return vec![
            Span::styled(v.to_string(), Style::default().fg(color)),
            Span::styled(comma.to_string(), Style::default().fg(MUTED)),
        ];
    }

    if v == "null" {
        return vec![
            Span::styled(v.to_string(), Style::default().fg(MUTED)),
            Span::styled(comma.to_string(), Style::default().fg(MUTED)),
        ];
    }

    if v == "true" || v == "false" {
        return vec![
            Span::styled(v.to_string(), Style::default().fg(YELLOW)),
            Span::styled(comma.to_string(), Style::default().fg(MUTED)),
        ];
    }

    if v.parse::<f64>().is_ok() {
        return vec![
            Span::styled(v.to_string(), Style::default().fg(PEACH)),
            Span::styled(comma.to_string(), Style::default().fg(MUTED)),
        ];
    }

    vec![Span::styled(value.to_string(), Style::default().fg(MUTED))]
}

fn selection_highlight_text(
    wrapped_lines: &[String],
    start: (usize, usize),
    end: (usize, usize),
) -> Text<'static> {
    let mut lines = Vec::with_capacity(wrapped_lines.len());
    let selection_style = Style::default().bg(SURFACE);

    for (row, line) in wrapped_lines.iter().enumerate() {
        let base_line = colorize_json_line(line);
        if row < start.0 || row > end.0 {
            lines.push(base_line);
            continue;
        }

        let line_len = line.chars().count();
        let start_col = if row == start.0 {
            start.1.min(line_len)
        } else {
            0
        };
        let end_col = if row == end.0 {
            end.1.min(line_len)
        } else {
            line_len
        };

        if end_col <= start_col {
            lines.push(base_line);
            continue;
        }

        lines.push(highlight_line_range(
            base_line,
            start_col,
            end_col,
            selection_style,
        ));
    }

    Text::from(lines)
}

fn highlight_line_range(
    line: Line<'static>,
    start_col: usize,
    end_col: usize,
    selection_style: Style,
) -> Line<'static> {
    let mut out_spans = Vec::new();
    let mut cursor = 0usize;

    for span in line.spans {
        let content = span.content.to_string();
        let span_len = content.chars().count();
        let span_start = cursor;
        let span_end = cursor + span_len;

        if span_len == 0 || end_col <= span_start || start_col >= span_end {
            out_spans.push(span);
            cursor = span_end;
            continue;
        }

        let local_start = start_col.saturating_sub(span_start).min(span_len);
        let local_end = end_col.saturating_sub(span_start).min(span_len);

        if local_start > 0 {
            out_spans.push(Span::styled(
                substring_chars(&content, 0, local_start),
                span.style,
            ));
        }

        if local_end > local_start {
            out_spans.push(Span::styled(
                substring_chars(&content, local_start, local_end),
                span.style.patch(selection_style),
            ));
        }

        if local_end < span_len {
            out_spans.push(Span::styled(
                substring_chars(&content, local_end, span_len),
                span.style,
            ));
        }

        cursor = span_end;
    }

    Line::from(out_spans)
}

fn substring_chars(text: &str, start: usize, end: usize) -> String {
    text.chars()
        .skip(start)
        .take(end.saturating_sub(start))
        .collect()
}
