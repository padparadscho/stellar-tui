use std::time::Instant;

use ratatui::layout::Rect;

use crate::app::core::forms::FormState;

/// Cached wrapped line metrics for the currently displayed response page
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrapMetrics {
    pub(crate) page: usize,
    pub(crate) wrap_width: u16,
    pub(crate) wrapped_lines: u16,
}

/// Identifies which pane currently has keyboard focus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPane {
    Methods,
    Request,
    Response,
}

/// Active modal overlay state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalState {
    None,
    About,
    Settings,
    Info,
}

/// Commands returned by input handlers to the main event loop
pub enum AppCommand {
    /// Execute the current RPC request
    SendRequest,
    /// Quit the application
    Quit,
}

/// A status message that automatically expires after a given duration
pub struct TimedStatus {
    pub message: String,
    pub created: Instant,
    pub duration_secs: u64,
}

/// Inline editor state for adding or editing a network
#[derive(Debug, Clone)]
pub struct NetworkEditor {
    /// Form fields for network name and endpoint
    pub form: FormState,
    /// Some(index) when editing an existing network, None when adding
    pub editing_index: Option<usize>,
}

/// Cached pixel regions for methods, request, and response panes
#[derive(Debug, Clone, Copy)]
pub struct UiRegions {
    pub methods: Rect,
    pub request: Rect,
    pub response: Rect,
}

/// Split response data for paginated display
pub struct PaginatedResponse {
    /// Lines of the response, split once on receipt
    pub lines: Vec<String>,
    /// Total number of lines
    pub total_lines: usize,
    /// Number of lines per page
    pub page_size: usize,
    /// Total number of pages
    pub total_pages: usize,
}

impl PaginatedResponse {
    /// Builds a paginated view from the raw response text
    pub fn from_text(text: &str, page_size: usize) -> Self {
        let lines: Vec<String> = text.lines().map(|l| l.to_string()).collect();
        let total_lines = lines.len();
        let total_pages = if total_lines == 0 {
            1
        } else {
            total_lines.div_ceil(page_size)
        };
        Self {
            lines,
            total_lines,
            page_size,
            total_pages,
        }
    }

    /// Returns the text for the given 0 based page index
    pub fn page_text(&self, page: usize) -> String {
        let start = page * self.page_size;
        let end = (start + self.page_size).min(self.total_lines);
        if start >= self.total_lines {
            return String::new();
        }
        self.lines[start..end].join("\n")
    }

    /// Returns the line count for a given page
    pub fn page_line_count(&self, page: usize) -> usize {
        let start = page * self.page_size;
        let end = (start + self.page_size).min(self.total_lines);
        if start >= self.total_lines {
            0
        } else {
            end - start
        }
    }
}
