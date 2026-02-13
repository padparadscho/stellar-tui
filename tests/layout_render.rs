//! Validates key TUI layout and rendering invariants for normal and fullscreen modes.

use ratatui::{backend::TestBackend, layout::Rect, Terminal};
use stellar_tui::{
    app::{App, FocusPane},
    settings::Settings,
    ui::{self, layout::MainLayout},
};

#[test]
fn response_search_area_is_present_only_when_response_is_fullscreen() {
    let area = Rect::new(0, 0, 120, 40);
    let mut app = App::new(Settings::default_settings());

    let normal = MainLayout::new(area, &app);
    assert_eq!(normal.search, Rect::default());

    app.focus = FocusPane::Response;
    app.handle_key(crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Char('f'),
        crossterm::event::KeyModifiers::NONE,
    ));

    let fullscreen = MainLayout::new(area, &app);
    assert!(fullscreen.search.height > 0);
    assert!(fullscreen.search.width > 0);
}

#[test]
fn compact_width_stacks_panes_vertically() {
    let area = Rect::new(0, 0, 90, 36);
    let app = App::new(Settings::default_settings());
    let layout = MainLayout::new(area, &app);

    assert_eq!(layout.methods.x, layout.request.x);
    assert_eq!(layout.request.x, layout.response.x);
    assert!(layout.methods.y < layout.request.y);
    assert!(layout.request.y < layout.response.y);
}

#[test]
fn draw_sets_response_region_to_full_body_when_zoomed() {
    let mut app = App::new(Settings::default_settings());
    app.focus = FocusPane::Response;
    app.handle_key(crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Char('f'),
        crossterm::event::KeyModifiers::NONE,
    ));

    let mut terminal =
        Terminal::new(TestBackend::new(120, 40)).expect("test backend should initialize");
    terminal
        .draw(|frame| ui::frame(frame, &mut app))
        .expect("draw should succeed");

    let expected = MainLayout::new(Rect::new(0, 0, 120, 40), &app).body;
    let regions = app
        .ui_regions
        .expect("ui regions should be set during draw");

    assert_eq!(regions.response, expected);
}
