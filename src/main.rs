mod app;
mod render;
mod events;
mod variable;
mod simulation;
mod action;

use crossterm::{
    execute, terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen
    }
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use simulation::step_simulation;
use std::{io::{self, stdout, Result, Stdout}, thread::sleep, time::Duration};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn main() -> Result<()> {
    let mut terminal = init()?;
    terminal.clear()?;

    let mut app = app::App::new()
        .with_start_config();

    

    loop {
        terminal.draw(|frame| render::render_app(&app, frame))?;
        if events::handle_events(&mut app)? {
            break;
        }

        step_simulation(&mut app);

        sleep(Duration::from_millis(100));
    }

    restore()
}
