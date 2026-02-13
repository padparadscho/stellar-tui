//! Verifies response search controls become visible and enabled in fullscreen response mode.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use stellar_tui::app::{App, FocusPane};
use stellar_tui::settings::Settings;

#[test]
fn response_search_visibility_depends_on_zoomed_response() {
    let mut app = App::new(Settings::default_settings());
    app.last_response = "hello".to_string();

    app.focus = FocusPane::Response;
    let fullscreen = KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE);
    app.handle_key(fullscreen);

    assert!(app.is_response_search_visible());
    assert!(app.is_response_search_enabled());
}

#[test]
fn search_cursor_left_and_right_respect_bounds() {
    let mut app = App::new(Settings::default_settings());
    app.focus = FocusPane::Response;
    app.last_response = "alpha\nbeta".to_string();

    app.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));

    assert_eq!(app.response_search_query, "ab");
    assert_eq!(app.response_search_cursor, 2);

    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.response_search_cursor, 2);

    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(app.response_search_cursor, 0);
}

#[test]
fn backspace_and_delete_edit_search_query_and_recalculate_matches() {
    let mut app = App::new(Settings::default_settings());
    app.focus = FocusPane::Response;
    app.last_response = "foo one\nbar two\nzoo".to_string();

    app.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));

    assert_eq!(app.response_search_query, "foo");
    assert_eq!(app.response_search_cursor, 3);
    assert_eq!(app.response_search_matches, vec![0]);

    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(app.response_search_cursor, 1);

    app.handle_key(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
    assert_eq!(app.response_search_query, "fo");
    assert_eq!(app.response_search_cursor, 1);
    assert_eq!(app.response_search_matches, vec![0]);

    app.handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    assert_eq!(app.response_search_query, "o");
    assert_eq!(app.response_search_cursor, 0);
    assert_eq!(app.response_search_matches, vec![0, 1, 2]);

    app.handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    assert_eq!(app.response_search_query, "o");
    assert_eq!(app.response_search_cursor, 0);
}
