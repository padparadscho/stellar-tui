//! Confirms response pagination calculates totals and returns the expected page slices.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use stellar_tui::app::{App, FocusPane, PaginatedResponse};
use stellar_tui::settings::Settings;

#[test]
fn paginated_response_splits_pages() {
    let text = "l1\nl2\nl3\nl4\nl5";
    let paged = PaginatedResponse::from_text(text, 2);

    assert_eq!(paged.total_lines, 5);
    assert_eq!(paged.total_pages, 3);
    assert_eq!(paged.page_text(0), "l1\nl2");
    assert_eq!(paged.page_text(1), "l3\nl4");
    assert_eq!(paged.page_text(2), "l5");
}

#[test]
fn left_and_right_keys_navigate_response_pages() {
    let mut app = App::new(Settings::default_settings());
    app.focus = FocusPane::Response;

    let text = (1..=450)
        .map(|n| format!("line-{n}"))
        .collect::<Vec<_>>()
        .join("\n");
    app.paginated_response = Some(PaginatedResponse::from_text(&text, 200));

    app.response_selection_start = Some((0, 0));
    app.response_selection_end = Some((0, 3));

    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.response_page, 1);
    assert_eq!(app.response_selection_start, None);
    assert_eq!(app.response_selection_end, None);

    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.response_page, 2);

    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.response_page, 2);

    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(app.response_page, 1);

    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(app.response_page, 0);

    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(app.response_page, 0);
}
