use std::time::Instant;

use arboard::Clipboard;

use crate::app::core::state::{App, DEFAULT_PAGE_SIZE};
use crate::app::core::types::{FocusPane, PaginatedResponse, UiRegions, WrapMetrics};

impl App {
    pub fn set_ui_regions(&mut self, regions: UiRegions) {
        self.ui_regions = Some(regions);
    }

    pub(in crate::app) fn scroll_response(&mut self, delta: i16) {
        let line_count = self.current_page_wrapped_line_count();
        let visible = self
            .ui_regions
            .map(|r| r.response.height.saturating_sub(2))
            .unwrap_or(10);
        let max_scroll = line_count.saturating_sub(visible);
        if delta < 0 {
            self.response_scroll = self.response_scroll.saturating_sub(1);
        } else if self.response_scroll < max_scroll {
            self.response_scroll = self.response_scroll.saturating_add(1);
        }
    }

    pub(in crate::app) fn begin_response_selection_from_mouse(&mut self, column: u16, row: u16) {
        let Some(position) = self.response_position_from_mouse(column, row) else {
            return;
        };
        self.response_selection_start = Some(position);
        self.response_selection_end = Some(position);
        self.response_selecting = true;
    }

    pub(in crate::app) fn update_response_selection_from_mouse(&mut self, column: u16, row: u16) {
        if !self.response_selecting {
            return;
        }
        let Some(position) = self.response_position_from_mouse(column, row) else {
            return;
        };
        self.response_selection_end = Some(position);
    }

    pub(in crate::app) fn finish_response_selection(&mut self) {
        self.response_selecting = false;
        if self.response_selection_start == self.response_selection_end {
            self.clear_response_selection();
        }
    }

    pub(in crate::app) fn clear_response_selection(&mut self) {
        self.response_selection_start = None;
        self.response_selection_end = None;
        self.response_selecting = false;
    }

    pub fn response_selection_range(&self) -> Option<((usize, usize), (usize, usize))> {
        let start = self.response_selection_start?;
        let end = self.response_selection_end?;
        if start == end {
            return None;
        }
        if start <= end {
            Some((start, end))
        } else {
            Some((end, start))
        }
    }

    pub fn wrapped_page_lines(&self) -> Vec<String> {
        wrap_text_lines(
            &self.current_page_text(),
            self.response_wrap_width() as usize,
        )
    }

    pub(in crate::app) fn copy_response_selection_or_page(&mut self) {
        if self.last_response.is_empty() {
            self.set_timed_status("Nothing to copy".to_string(), 3);
            return;
        }

        let selected = self.selected_response_text();
        match selected {
            Some(text) if !text.is_empty() => {
                if self.copy_text_to_clipboard(text, "Selection copied!") {
                    self.clear_response_selection();
                }
            }
            _ => {
                self.copy_text_to_clipboard(self.current_page_text(), "Current page copied!");
            }
        }
    }

    /// Returns text for current page or full response
    pub fn current_page_text(&self) -> String {
        if let Some(ref pag) = self.paginated_response {
            pag.page_text(self.response_page)
        } else {
            self.last_response.clone()
        }
    }

    /// Returns line count for current page
    fn current_page_line_count(&self) -> usize {
        if let Some(ref pag) = self.paginated_response {
            pag.page_line_count(self.response_page)
        } else {
            self.last_response.lines().count()
        }
    }

    /// Rebuilds paginated response from last response
    pub(in crate::app) fn rebuild_pagination(&mut self) {
        self.response_scroll = 0;
        self.response_page = 0;
        self.wrap_metrics = None;
        self.clear_response_selection();
        if self.last_response.is_empty() {
            self.paginated_response = None;
        } else {
            self.paginated_response = Some(PaginatedResponse::from_text(
                &self.last_response,
                DEFAULT_PAGE_SIZE,
            ));
        }
    }

    /// Moves to next response page when available
    pub(in crate::app) fn next_response_page(&mut self) {
        if let Some(ref pag) = self.paginated_response {
            if self.response_page + 1 < pag.total_pages {
                self.response_page += 1;
                self.response_scroll = 0;
                self.wrap_metrics = None;
                self.clear_response_selection();
            }
        }
    }

    /// Moves to previous response page when available
    pub(in crate::app) fn prev_response_page(&mut self) {
        if self.response_page > 0 {
            self.response_page -= 1;
            self.response_scroll = 0;
            self.wrap_metrics = None;
            self.clear_response_selection();
        }
    }

    /// Jumps response scroll to start
    pub(in crate::app) fn jump_response_start(&mut self) {
        self.response_scroll = 0;
    }

    /// Jumps response scroll to end of current page
    pub(in crate::app) fn jump_response_end(&mut self) {
        let line_count = self.current_page_wrapped_line_count();
        let visible = self
            .ui_regions
            .map(|r| r.response.height.saturating_sub(2))
            .unwrap_or(10);
        self.response_scroll = line_count.saturating_sub(visible);
    }

    /// Clears request form and response state
    pub(in crate::app) fn purge_data(&mut self) {
        let specs = self.methods[self.selected_method].fields.clone();
        self.request_forms[self.selected_method].reset_from_specs(&specs);
        self.method_errors[self.selected_method].clear();

        self.last_response.clear();
        self.response_scroll = 0;
        self.response_page = 0;
        self.paginated_response = None;
        self.wrap_metrics = None;
        self.clear_response_search();
        self.clear_response_selection();

        self.set_timed_status("Data cleared!".to_string(), 3);
    }

    fn current_page_wrapped_line_count(&mut self) -> u16 {
        let wrap_width = self.response_wrap_width();
        let page = self.response_page;
        if let Some(metrics) = self.wrap_metrics {
            if metrics.page == page && metrics.wrap_width == wrap_width {
                return metrics.wrapped_lines;
            }
        }

        let wrapped_lines = self
            .wrapped_line_count_for_page(page, wrap_width)
            .min(u16::MAX as usize) as u16;

        self.wrap_metrics = Some(WrapMetrics {
            page,
            wrap_width,
            wrapped_lines,
        });
        wrapped_lines
    }

    fn response_wrap_width(&self) -> u16 {
        self.ui_regions
            .map(|r| r.response.width.saturating_sub(2).max(1))
            .unwrap_or(80)
    }

    fn response_position_from_mouse(&self, column: u16, row: u16) -> Option<(usize, usize)> {
        let rect = self.ui_regions?.response;
        let inner_x = rect.x.saturating_add(1);
        let inner_y = rect.y.saturating_add(1);
        let inner_w = rect.width.saturating_sub(2);
        let inner_h = rect.height.saturating_sub(2);

        if inner_w == 0 || inner_h == 0 {
            return None;
        }
        if column < inner_x || row < inner_y {
            return None;
        }

        let local_x = column.saturating_sub(inner_x);
        let local_y = row.saturating_sub(inner_y);
        if local_x >= inner_w || local_y >= inner_h {
            return None;
        }

        Some((
            self.response_scroll as usize + local_y as usize,
            local_x as usize,
        ))
    }

    fn selected_response_text(&self) -> Option<String> {
        let (start, end) = self.response_selection_range()?;
        let lines = self.wrapped_page_lines();
        if lines.is_empty() {
            return None;
        }

        let mut out = Vec::new();
        for row in start.0..=end.0 {
            let line = match lines.get(row) {
                Some(line) => line,
                None => continue,
            };
            let line_chars = line.chars().count();
            let start_col = if row == start.0 {
                start.1.min(line_chars)
            } else {
                0
            };
            let end_col = if row == end.0 {
                end.1.min(line_chars)
            } else {
                line_chars
            };

            if end_col <= start_col {
                continue;
            }
            out.push(substring_chars(line, start_col, end_col));
        }

        if out.is_empty() {
            None
        } else {
            Some(out.join("\n"))
        }
    }

    fn copy_text_to_clipboard(&mut self, text: String, success_message: &str) -> bool {
        if self.clipboard.is_none() {
            match Clipboard::new() {
                Ok(cb) => self.clipboard = Some(cb),
                Err(err) => {
                    self.set_timed_status(format!("Copy failed: {}", err), 5);
                    return false;
                }
            }
        }

        match self
            .clipboard
            .as_mut()
            .expect("clipboard initialized")
            .set_text(text)
        {
            Ok(_) => {
                self.set_timed_status(success_message.to_string(), 3);
                true
            }
            Err(err) => {
                self.set_timed_status(format!("Copy failed: {}", err), 5);
                false
            }
        }
    }

    fn wrapped_line_count_for_page(&self, page: usize, wrap_width: u16) -> usize {
        let wrap = wrap_width as usize;
        if wrap == 0 {
            return self.current_page_line_count();
        }

        let iter: Box<dyn Iterator<Item = &str>> = if let Some(ref pag) = self.paginated_response {
            let start = page * pag.page_size;
            let end = (start + pag.page_size).min(pag.total_lines);
            if start >= pag.total_lines {
                Box::new(std::iter::empty())
            } else {
                Box::new(pag.lines[start..end].iter().map(|s| s.as_str()))
            }
        } else {
            Box::new(self.last_response.lines())
        };

        iter.map(|line| {
            let len = line.chars().count();
            if len == 0 {
                1
            } else {
                len.div_ceil(wrap)
            }
        })
        .sum()
    }

    /// Clears the response search state entirely
    pub(in crate::app) fn clear_response_search(&mut self) {
        self.response_search_query.clear();
        self.response_search_cursor = 0;
        self.response_search_matches.clear();
        self.response_search_current = 0;
    }

    /// Recalculates search matches from the current query and current page text
    ///
    /// If called within 100ms of the last query change, the recalculation is deferred to the next tick
    pub(in crate::app) fn update_response_search_matches(&mut self) {
        self.search_last_changed = Some(Instant::now());
        self.recalculate_search_matches();
    }

    /// Performs the actual search match calculation on the current page
    fn recalculate_search_matches(&mut self) {
        self.response_search_matches.clear();
        self.response_search_current = 0;

        if self.response_search_query.is_empty() || self.last_response.is_empty() {
            return;
        }

        let page_text = self.current_page_text();
        let query = self.response_search_query.to_lowercase();
        for (i, line) in page_text.lines().enumerate() {
            if line.to_lowercase().contains(&query) {
                self.response_search_matches.push(i);
            }
        }
    }

    /// Scrolls to the current search match so it is visible
    pub(in crate::app) fn scroll_to_current_match(&mut self) {
        if let Some(&line) = self
            .response_search_matches
            .get(self.response_search_current)
        {
            self.response_scroll = line as u16;
        }
    }

    /// Advances to the next search match, wrapping around
    pub(in crate::app) fn next_search_match(&mut self) {
        if self.response_search_matches.is_empty() {
            return;
        }
        self.response_search_current =
            (self.response_search_current + 1) % self.response_search_matches.len();
        self.scroll_to_current_match();
    }

    /// Moves to the previous search match, wrapping around
    pub(in crate::app) fn prev_search_match(&mut self) {
        if self.response_search_matches.is_empty() {
            return;
        }
        if self.response_search_current == 0 {
            self.response_search_current = self.response_search_matches.len() - 1;
        } else {
            self.response_search_current -= 1;
        }
        self.scroll_to_current_match();
    }

    /// Returns true when the response search box should be visible
    pub fn is_response_search_visible(&self) -> bool {
        self.zoomed_pane == Some(FocusPane::Response)
    }

    /// Returns true when the search box should be interactive (response has content)
    pub fn is_response_search_enabled(&self) -> bool {
        self.is_response_search_visible() && !self.last_response.is_empty()
    }
}

fn wrap_text_lines(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return text.lines().map(ToOwned::to_owned).collect();
    }

    let mut wrapped = Vec::new();
    for line in text.lines() {
        if line.is_empty() {
            wrapped.push(String::new());
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        let mut start = 0;
        while start < chars.len() {
            let end = (start + width).min(chars.len());
            wrapped.push(chars[start..end].iter().collect());
            start = end;
        }
    }

    wrapped
}

fn substring_chars(text: &str, start: usize, end: usize) -> String {
    text.chars()
        .skip(start)
        .take(end.saturating_sub(start))
        .collect()
}
