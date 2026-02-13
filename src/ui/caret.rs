use ratatui::layout::{Constraint, Layout, Margin, Rect};

use crate::app::{App, FocusPane, ModalState};
use crate::ui::layout::MainLayout;

/// Resolves the terminal cursor position for the current UI state
pub(super) fn caret_position(
    frame_area: Rect,
    app: &App,
    layout: &MainLayout,
) -> Option<(u16, u16)> {
    if app.modal == ModalState::Settings {
        return settings_editor_caret(frame_area, app);
    }

    if app.is_response_search_enabled() {
        return response_search_caret(layout.search, app);
    }

    if app.modal != ModalState::None || app.focus != FocusPane::Request {
        return None;
    }

    request_form_caret(layout.request, app)
}

/// Computes caret position for the focused request form field
fn request_form_caret(area: Rect, app: &App) -> Option<(u16, u16)> {
    let form = app.request_forms.get(app.selected_method)?;
    let field = form.fields.get(form.selected)?;
    if !app.is_selected_request_editable() {
        return None;
    }

    let inner = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });
    let show_detail = !form.fields.is_empty() && inner.height > 5;
    let list_height = if show_detail {
        inner.height.saturating_sub(3)
    } else {
        inner.height
    };

    let line_on_screen = form.selected.saturating_sub(form.scroll);
    if line_on_screen >= list_height as usize {
        // Selected field is currently outside the visible viewport
        return None;
    }

    let y = inner.y.saturating_add(line_on_screen as u16);
    let mut x = inner.x;
    x = x.saturating_add(2);
    x = x.saturating_add(field.spec.label.chars().count() as u16);
    x = x.saturating_add(2);
    x = x.saturating_add(field.cursor.min(field.value.chars().count()) as u16);

    let max_x = inner.x.saturating_add(inner.width.saturating_sub(1));
    Some((x.min(max_x), y))
}

/// Computes caret position inside the response search box
fn response_search_caret(search_area: Rect, app: &App) -> Option<(u16, u16)> {
    if search_area.width < 3 || search_area.height < 3 {
        return None;
    }

    let inner = search_area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });
    let mut x = inner.x;
    x = x.saturating_add("Search: ".chars().count() as u16);
    x = x.saturating_add(
        app.response_search_cursor
            .min(app.response_search_query.chars().count()) as u16,
    );
    let max_x = inner.x.saturating_add(inner.width.saturating_sub(1));

    Some((x.min(max_x), inner.y))
}

/// Computes caret position for the settings modal's network form
fn settings_editor_caret(frame_area: Rect, app: &App) -> Option<(u16, u16)> {
    let editor = app.network_editor.as_ref()?;
    let field = editor.form.fields.get(editor.form.selected)?;

    let area = centered_rect(60, 70, frame_area);
    let inner = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let content_area = Rect {
        x: inner.x,
        y: inner.y,
        width: inner.width,
        height: inner.height.saturating_sub(2),
    };

    let field_line_index = 5usize.saturating_add(editor.form.selected);
    let y = content_area.y.saturating_add(field_line_index as u16);

    let mut x = content_area.x;
    x = x.saturating_add(2);
    x = x.saturating_add(field.spec.label.chars().count() as u16);
    x = x.saturating_add(2);
    x = x.saturating_add(field.cursor.min(field.value.chars().count()) as u16);

    let max_x = content_area
        .x
        .saturating_add(content_area.width.saturating_sub(1));
    Some((x.min(max_x), y))
}

/// Builds a centered rectangle using percentage-based width and height
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(area);

    let horizontal = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(vertical[1]);

    horizontal[1]
}
