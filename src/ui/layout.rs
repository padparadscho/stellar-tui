use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::app::{App, FocusPane, UiRegions};

/// Width threshold below which header/footer stack vertically
const COMPACT_THRESHOLD: u16 = 80;

/// Returns how many content rows the header needs (excluding its border)
pub fn header_rows(width: u16) -> u16 {
    if width < COMPACT_THRESHOLD {
        3 // title + up to 2 wrapped status lines
    } else {
        1 // single content row (borders provide spacing)
    }
}

/// Returns how many content rows the footer needs (excluding its border)
pub fn footer_rows(width: u16) -> u16 {
    if width < COMPACT_THRESHOLD {
        2
    } else {
        1
    }
}

pub struct MainLayout {
    pub header: Rect,
    pub footer: Rect,
    /// Full content area between header and footer (used for fullscreen panes)
    pub body: Rect,
    pub methods: Rect,
    pub request: Rect,
    pub response: Rect,
    /// Area for the response search box (only set in fullscreen Response mode)
    pub search: Rect,
}

impl MainLayout {
    pub fn new(area: Rect, app: &App) -> Self {
        let focus = app.focus;
        let header_height = header_rows(area.width) + 2; // +2 for top+bottom border
        let footer_height = footer_rows(area.width) + 2; // +2 for top+bottom border

        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(header_height),
                Constraint::Min(0),
                Constraint::Length(footer_height),
            ])
            .split(area);

        let body = vertical[1];

        // When Response is fullscreen, split body to reserve space for the search box
        let (effective_body, search) = if app.is_response_search_visible() {
            let [pane_area, search_area] =
                Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).areas(body);
            (pane_area, search_area)
        } else {
            (body, Rect::default())
        };

        // Compute dynamic vertical weights for Request/Response panes
        let (request_weight, response_weight) = match focus {
            FocusPane::Request => (3, 1),
            FocusPane::Response => (1, 3),
            FocusPane::Methods => (1, 1),
        };

        // Narrow layout: stack panes vertically
        if area.width < 110 {
            let stack = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(6),
                    Constraint::Fill(request_weight),
                    Constraint::Fill(response_weight),
                ])
                .split(vertical[1]);

            return Self {
                header: vertical[0],
                footer: vertical[2],
                body: effective_body,
                methods: stack[0],
                request: stack[1],
                response: stack[2],
                search,
            };
        }

        // Wide layout: methods on left, request + response stacked on right
        let main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(vertical[1]);

        let right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(request_weight),
                Constraint::Fill(response_weight),
            ])
            .split(main[1]);

        Self {
            header: vertical[0],
            footer: vertical[2],
            body: effective_body,
            methods: main[0],
            request: right[0],
            response: right[1],
            search,
        }
    }

    pub fn to_regions(&self) -> UiRegions {
        UiRegions {
            methods: self.methods,
            request: self.request,
            response: self.response,
        }
    }
}
