use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::{App, FocusPane, ModalState};
use crate::ui::palette::{ACCENT, PEACH};
use crate::ui::styles::{pane_border_style, pane_border_type, shortcut_hint};

/// Renders the methods list with an HTTP method badge next to each name
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == FocusPane::Methods;
    let modal_active = app.modal != ModalState::None;
    let mut block = Block::default()
        .borders(Borders::ALL)
        .title(" Methods ")
        .border_style(pane_border_style(focused, modal_active))
        .border_type(pane_border_type(focused, modal_active))
        .title_style(pane_border_style(focused, modal_active));

    if focused && !modal_active {
        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span::raw(" "));
        spans.extend(shortcut_hint("Run"));
        spans.push(Span::raw(" "));
        spans.extend(shortcut_hint("Info"));
        spans.push(Span::raw(" "));
        block = block.title(Line::from(spans).right_aligned());
    }

    let items: Vec<ListItem> = app
        .methods
        .iter()
        .map(|method| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{} ", method.http_method),
                    Style::default().fg(PEACH).add_modifier(Modifier::DIM),
                ),
                Span::raw(method.name),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .highlight_symbol("❯ ");

    let mut state = ListState::default();
    state.select(Some(app.selected_method));
    frame.render_stateful_widget(list, area, &mut state);
}
