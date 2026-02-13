use std::io;

use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use stellar_tui::app::{App, AppCommand};
use stellar_tui::settings::Settings;
use stellar_tui::ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    install_panic_hook();

    let settings = Settings::load_or_default();
    let mut app = App::new(settings);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        cursor::SetCursorStyle::BlinkingBar
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

/// Installs a panic hook that restores terminal state on panic
///
/// [IMPORTANT] Without this, a panic can leave raw mode enabled and the alternate screen active
fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Terminal restoration, ignore errors since this runs during panic
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        original_hook(panic_info);
    }));
}

/// Runs the event loop, rendering the UI and dispatching input events
fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| ui::frame(frame, app))?;

        // Poll for background request completion on every iteration
        app.tick();

        if event::poll(std::time::Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match app.handle_key(key) {
                            Some(AppCommand::Quit) => return Ok(()),
                            Some(AppCommand::SendRequest) => app.execute_request(),
                            None => {}
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    app.handle_mouse(mouse);
                }
                _ => {}
            }
        }
    }
}
