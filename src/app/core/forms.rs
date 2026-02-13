/// Data type of a form field, used for validation and UI badges
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType {
    /// Text input
    Text,
    /// Unsigned integer input
    Number,
    /// Comma-separated list of values
    List,
    /// Raw JSON input
    Json,
}

/// Static specification for a single form field
#[derive(Debug, Clone)]
pub struct FieldSpec {
    /// Unique identifier used as the map key in params/errors
    pub key: &'static str,
    /// Label shown in the UI
    pub label: &'static str,
    /// Expected data type
    pub field_type: FieldType,
    /// If true, an empty value is acceptable
    pub optional: bool,
    /// Default value
    pub default_value: &'static str,
    /// Short description shown in the detail area
    pub hint: &'static str,
}

/// Runtime state of a single form field (spec + current value)
#[derive(Debug, Clone)]
pub struct FieldState {
    /// Original specification for this field
    pub spec: FieldSpec,
    /// Current value
    pub value: String,
    /// Cursor position in character units
    pub cursor: usize,
}

/// Mutable state for an interactive form
#[derive(Debug, Clone)]
pub struct FormState {
    /// Ordered list of field states
    pub fields: Vec<FieldState>,
    /// Index of the currently selected field
    pub selected: usize,
    /// Scroll offset for viewport based rendering
    pub scroll: usize,
}

impl FormState {
    /// Creates a new form state from a slice of field specifications
    pub fn from_specs(specs: &[FieldSpec]) -> Self {
        let fields = specs
            .iter()
            .cloned()
            .map(|spec| FieldState {
                value: spec.default_value.to_string(),
                cursor: spec.default_value.chars().count(),
                spec,
            })
            .collect();

        Self {
            fields,
            selected: 0,
            scroll: 0,
        }
    }

    /// Resets all field values and selection to match the given specs
    pub fn reset_from_specs(&mut self, specs: &[FieldSpec]) {
        *self = Self::from_specs(specs);
    }

    /// Returns a mutable reference to the currently selected field
    pub fn current_field_mut(&mut self) -> Option<&mut FieldState> {
        self.fields.get_mut(self.selected)
    }

    /// Moves selection to the next field, clamped to the last index
    pub fn select_next(&mut self) {
        if self.selected + 1 < self.fields.len() {
            self.selected += 1;
        }
    }

    /// Moves selection to the previous field, clamped to zero
    pub fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Adjusts scroll so the selected field is visible in a viewport
    pub fn ensure_visible(&mut self, viewport_height: usize) {
        if viewport_height == 0 || self.fields.is_empty() {
            self.scroll = 0;
            return;
        }

        if self.selected < self.scroll {
            self.scroll = self.selected;
            return;
        }

        let bottom = self.scroll + viewport_height - 1;
        if self.selected > bottom {
            self.scroll = self.selected.saturating_sub(viewport_height - 1);
        }
    }

    /// Looks up the current value for a field by key
    pub fn field_value(&self, key: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|field| field.spec.key == key)
            .map(|field| field.value.as_str())
    }

    /// Appends a character to the selected field's value
    pub fn insert_char(&mut self, ch: char) {
        if let Some(field) = self.current_field_mut() {
            if !field.spec.default_value.is_empty() && field.value == field.spec.default_value {
                let default_is_numeric =
                    field.spec.default_value.chars().all(|c| c.is_ascii_digit());
                let allow_append_zero = default_is_numeric && ch == '0';
                if !allow_append_zero {
                    field.value.clear();
                    field.cursor = 0;
                }
            }
            let insert_at = byte_index_at_char(&field.value, field.cursor);
            field.value.insert(insert_at, ch);
            field.cursor += 1;
        }
    }

    /// Removes the last character from the selected field's value
    pub fn backspace(&mut self) {
        if let Some(field) = self.current_field_mut() {
            if field.cursor == 0 {
                return;
            }
            let remove_start = byte_index_at_char(&field.value, field.cursor - 1);
            let remove_end = byte_index_at_char(&field.value, field.cursor);
            field.value.replace_range(remove_start..remove_end, "");
            field.cursor -= 1;
        }
    }

    /// Removes the character at the cursor in the selected field
    pub fn delete_forward(&mut self) {
        if let Some(field) = self.current_field_mut() {
            let max = field.value.chars().count();
            if field.cursor >= max {
                return;
            }
            let remove_start = byte_index_at_char(&field.value, field.cursor);
            let remove_end = byte_index_at_char(&field.value, field.cursor + 1);
            field.value.replace_range(remove_start..remove_end, "");
        }
    }

    /// Moves the cursor one character left in the selected field
    pub fn cursor_left(&mut self) {
        if let Some(field) = self.current_field_mut() {
            field.cursor = field.cursor.saturating_sub(1);
        }
    }

    /// Moves the cursor one character right in the selected field
    pub fn cursor_right(&mut self) {
        if let Some(field) = self.current_field_mut() {
            let max = field.value.chars().count();
            if field.cursor < max {
                field.cursor += 1;
            }
        }
    }

    /// Sets the value of a field identified by key
    pub fn set_value(&mut self, key: &str, value: String) {
        if let Some(field) = self.fields.iter_mut().find(|field| field.spec.key == key) {
            field.value = value;
            field.cursor = field.value.chars().count();
        }
    }
}

fn byte_index_at_char(text: &str, char_index: usize) -> usize {
    text.char_indices()
        .nth(char_index)
        .map(|(idx, _)| idx)
        .unwrap_or(text.len())
}
