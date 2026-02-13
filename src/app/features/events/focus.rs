use ratatui::layout::Rect;

use crate::app::FocusPane;

/// Keeps tab navigation aligned with left to right pane order
pub(super) fn next_focus(current: FocusPane) -> FocusPane {
    match current {
        FocusPane::Methods => FocusPane::Request,
        FocusPane::Request => FocusPane::Response,
        FocusPane::Response => FocusPane::Methods,
    }
}

/// Keeps reverse tab navigation aligned with right to left pane order
pub(super) fn prev_focus(current: FocusPane) -> FocusPane {
    match current {
        FocusPane::Methods => FocusPane::Response,
        FocusPane::Request => FocusPane::Methods,
        FocusPane::Response => FocusPane::Request,
    }
}

/// Uses saturating bounds so extreme sizes never overflow coordinate math
pub(super) fn contains_point(rect: Rect, column: u16, row: u16) -> bool {
    column >= rect.x
        && column < rect.x.saturating_add(rect.width)
        && row >= rect.y
        && row < rect.y.saturating_add(rect.height)
}
