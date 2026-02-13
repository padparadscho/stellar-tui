//! Verifies keyboard-driven focus navigation order across the main application panes

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use stellar_tui::app::{App, FocusPane, ModalState};
use stellar_tui::settings::Settings;

#[test]
fn tab_cycles_focus_order() {
    let mut app = App::new(Settings::default_settings());

    let tab = KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE);
    app.handle_key(tab);
    assert_eq!(app.focus, FocusPane::Request);

    let tab = KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE);
    app.handle_key(tab);
    assert_eq!(app.focus, FocusPane::Response);

    let tab = KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE);
    app.handle_key(tab);
    assert_eq!(app.focus, FocusPane::Methods);
}

#[test]
fn backtab_cycles_focus_in_reverse_order() {
    let mut app = App::new(Settings::default_settings());

    let backtab = KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT);
    app.handle_key(backtab);
    assert_eq!(app.focus, FocusPane::Response);

    let backtab = KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT);
    app.handle_key(backtab);
    assert_eq!(app.focus, FocusPane::Request);

    let backtab = KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT);
    app.handle_key(backtab);
    assert_eq!(app.focus, FocusPane::Methods);
}

#[test]
fn tab_does_not_change_focus_in_fullscreen_mode() {
    let mut app = App::new(Settings::default_settings());
    app.focus = FocusPane::Request;
    app.zoomed_pane = Some(FocusPane::Request);

    let tab = KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE);
    app.handle_key(tab);
    assert_eq!(app.focus, FocusPane::Request);
}

#[test]
fn esc_clears_zoom_and_response_search_state() {
    let mut app = App::new(Settings::default_settings());
    app.focus = FocusPane::Response;
    app.last_response = "first foo\nsecond\nthird foo".to_string();

    let fullscreen = KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE);
    app.handle_key(fullscreen);

    app.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));

    app.response_selection_start = Some((0, 0));
    app.response_selection_end = Some((0, 2));

    assert_eq!(app.zoomed_pane, Some(FocusPane::Response));
    assert_eq!(app.response_search_query, "foo");
    assert_eq!(app.response_search_matches.len(), 2);

    let esc = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
    app.handle_key(esc);

    assert_eq!(app.zoomed_pane, None);
    assert!(app.response_search_query.is_empty());
    assert!(app.response_search_matches.is_empty());
    assert_eq!(app.response_selection_start, None);
    assert_eq!(app.response_selection_end, None);
}

#[test]
fn q_closes_modal_without_quitting() {
    let mut app = App::new(Settings::default_settings());
    app.modal = ModalState::About;

    let result = app.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));

    assert!(result.is_none());
    assert_eq!(app.modal, ModalState::None);
}

#[test]
fn tab_and_backtab_cycle_search_matches_when_search_is_enabled() {
    let mut app = App::new(Settings::default_settings());
    app.focus = FocusPane::Response;
    app.last_response = "foo one\nbar\nfoo two".to_string();

    app.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));

    assert_eq!(app.focus, FocusPane::Response);
    assert_eq!(app.response_search_matches, vec![0, 2]);
    assert_eq!(app.response_search_current, 0);

    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.response_search_current, 1);
    assert_eq!(app.focus, FocusPane::Response);

    app.handle_key(KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT));
    assert_eq!(app.response_search_current, 0);
    assert_eq!(app.focus, FocusPane::Response);
}
