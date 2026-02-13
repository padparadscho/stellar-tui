use arboard::Clipboard;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::mpsc;
use std::time::Instant;

use crate::app::core::forms::FormState;
use crate::app::core::types::{
    FocusPane, ModalState, NetworkEditor, PaginatedResponse, TimedStatus, UiRegions, WrapMetrics,
};
use crate::app::methods::MethodSpec;
use crate::rpc::types::RpcResponse;
use crate::settings::Settings;

/// Default number of lines per page for paginated responses
pub(crate) const DEFAULT_PAGE_SIZE: usize = 200;

/// Root application state
pub struct App {
    // Navigation and settings
    pub focus: FocusPane,
    pub settings: Settings,
    pub selected_network: usize,
    pub ui_regions: Option<UiRegions>,
    pub modal: ModalState,
    /// Scroll offset for modal content
    pub modal_scroll: u16,
    /// Maximum scroll offset for current modal content
    pub modal_max_scroll: u16,
    /// Fullscreen target pane when active
    pub zoomed_pane: Option<FocusPane>,

    // Methods and forms
    pub methods: Vec<MethodSpec>,
    pub selected_method: usize,
    pub request_forms: Vec<FormState>,
    pub method_errors: Vec<HashMap<String, String>>,

    // Network editor
    pub network_editor: Option<NetworkEditor>,
    pub network_errors: HashMap<String, String>,

    // Status and runtime
    pub status: String,
    /// Transient status message with timeout
    pub timed_status: Option<TimedStatus>,
    /// Frame counter for spinner animation
    pub spinner_frame: usize,
    /// Receiver for pending request result
    pub pending_request: Option<mpsc::Receiver<Result<RpcResponse<Value>, String>>>,

    // Response content and paging
    pub last_response: String,
    /// Pre split response pages for rendering
    pub paginated_response: Option<PaginatedResponse>,
    /// Current page index in paginated response
    pub response_page: usize,
    pub response_scroll: u16,

    // Search and selection
    /// Search query for response content
    pub response_search_query: String,
    /// Cursor position in search query
    pub response_search_cursor: usize,
    /// Line indices that match search query
    pub response_search_matches: Vec<usize>,
    /// Active index into search matches
    pub response_search_current: usize,
    /// Timestamp of last search query update
    pub search_last_changed: Option<Instant>,
    /// Selection start in wrapped row and column
    pub response_selection_start: Option<(usize, usize)>,
    /// Selection end in wrapped row and column
    pub response_selection_end: Option<(usize, usize)>,
    /// True while drag selection is active
    pub response_selecting: bool,

    // Render and clipboard cache
    /// Cached wrapped line count for current page width
    pub(crate) wrap_metrics: Option<WrapMetrics>,
    /// Clipboard handle kept alive for Linux session stability
    pub(crate) clipboard: Option<Clipboard>,
}
