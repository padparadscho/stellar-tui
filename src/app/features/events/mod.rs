mod focus;
mod keyboard;
mod modal;
mod mouse;
mod network;
mod search;

use crossterm::event::{KeyEvent, MouseEvent};

use crate::app::{App, AppCommand};

impl App {
    /// Central keyboard entry so precedence stays in one place
    pub fn handle_key(&mut self, key: KeyEvent) -> Option<AppCommand> {
        keyboard::handle_key(self, key)
    }

    /// Central mouse entry so pane focus logic stays unified
    pub fn handle_mouse(&mut self, event: MouseEvent) {
        mouse::handle_mouse(self, event)
    }
}
