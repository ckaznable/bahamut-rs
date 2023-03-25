use std::{error::Error, io};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode}, execute, event::{EnableMouseCapture, DisableMouseCapture, Event, self}};
use ratatui::{backend::{CrosstermBackend, Backend}, Terminal};
use ui::{state::AppState, key::{handle_key, KeyBindEvent}};

mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = AppState::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: AppState) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, &mut app))?;

        if let Event::Key(event) = event::read()? {
            if handle_key(&mut app, event).is_quit() {
                return Ok(());
            }
        }
    }
}
