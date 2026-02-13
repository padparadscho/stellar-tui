use crate::app::App;

impl App {
    /// Moves search cursor one position left
    pub(in crate::app::features::events) fn move_search_cursor_left(&mut self) {
        self.response_search_cursor = self.response_search_cursor.saturating_sub(1);
    }

    /// Moves search cursor one position right
    pub(in crate::app::features::events) fn move_search_cursor_right(&mut self) {
        let max = self.response_search_query.chars().count();
        if self.response_search_cursor < max {
            self.response_search_cursor += 1;
        }
    }

    /// Inserts a character at the current search cursor
    pub(in crate::app::features::events) fn insert_search_char(&mut self, ch: char) {
        // Cursor is tracked in chars, String insertion needs a byte offset
        let insert_at =
            byte_index_at_char(&self.response_search_query, self.response_search_cursor);
        self.response_search_query.insert(insert_at, ch);
        self.response_search_cursor += 1;
    }

    /// Removes the character before the search cursor
    pub(in crate::app::features::events) fn search_backspace(&mut self) {
        if self.response_search_cursor == 0 {
            return;
        }
        let remove_start =
            byte_index_at_char(&self.response_search_query, self.response_search_cursor - 1);
        let remove_end =
            byte_index_at_char(&self.response_search_query, self.response_search_cursor);
        self.response_search_query
            .replace_range(remove_start..remove_end, "");
        self.response_search_cursor -= 1;
    }

    /// Removes the character at the search cursor
    pub(in crate::app::features::events) fn search_delete_forward(&mut self) {
        let max = self.response_search_query.chars().count();
        if self.response_search_cursor >= max {
            return;
        }
        let remove_start =
            byte_index_at_char(&self.response_search_query, self.response_search_cursor);
        let remove_end =
            byte_index_at_char(&self.response_search_query, self.response_search_cursor + 1);
        self.response_search_query
            .replace_range(remove_start..remove_end, "");
    }
}

/// Converts a character index to a byte index for String slicing and edits
fn byte_index_at_char(text: &str, char_index: usize) -> usize {
    text.char_indices()
        .nth(char_index)
        .map(|(idx, _)| idx)
        .unwrap_or(text.len())
}
