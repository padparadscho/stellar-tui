use ratatui::{layout::Rect, Frame};

use crate::app::{App, FocusPane, ModalState};
use crate::ui::components::search;
use crate::ui::layout::MainLayout;
use crate::ui::panes::{footer, header, methods, request, response};

use super::caret::caret_position;

/// Renders the full TUI frame, handling fullscreen and modal overlays
pub fn frame(frame: &mut Frame, app: &mut App) {
    let layout = MainLayout::new(frame.area(), app);

    header::render(frame, layout.header, app);
    footer::render(frame, layout.footer, app);

    if let Some(zoomed) = app.zoomed_pane {
        let mut regions = layout.to_regions();
        match zoomed {
            FocusPane::Request => regions.request = layout.body,
            FocusPane::Response => regions.response = layout.body,
            FocusPane::Methods => {}
        }
        app.set_ui_regions(regions);
        render_zoomed_pane(frame, layout.body, layout.search, app, zoomed);
    } else {
        app.set_ui_regions(layout.to_regions());
        methods::render(frame, layout.methods, app);
        request::render(frame, layout.request, app);
        response::render(frame, layout.response, app);
    }

    match app.modal {
        ModalState::About => crate::ui::modals::about::render(frame, app),
        ModalState::Settings => crate::ui::modals::settings::render(frame, app),
        ModalState::Info => crate::ui::modals::info::render(frame, app),
        ModalState::None => {}
    }

    if let Some((x, y)) = caret_position(frame.area(), app, &layout) {
        frame.set_cursor_position((x, y));
    }
}

fn render_zoomed_pane(
    frame: &mut Frame,
    area: Rect,
    search_area: Rect,
    app: &mut App,
    pane: FocusPane,
) {
    match pane {
        FocusPane::Request => request::render(frame, area, app),
        FocusPane::Response => {
            response::render(frame, area, app);
            search::render(frame, search_area, app);
        }
        _ => {}
    }
}
