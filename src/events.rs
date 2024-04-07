use std::io::Result;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::app::App;


pub fn handle_events(app: &mut App) -> Result<bool> {
    if !event::poll(std::time::Duration::from_millis(16))? {
        return Ok(false);
    }

    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(app, key_event)
        }
        _ => { Ok(false) }
    }
}

fn handle_key_event(app: &mut App, key_event: KeyEvent) -> Result<bool> {
    match key_event.code {
        KeyCode::Char('q') => return Ok(true),
        _ => { app.handle_key_event(key_event); }
    }
    Ok(false)
}