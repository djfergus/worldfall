use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Move(i32, i32),
    Quit,
    None,
}

pub fn get_input() -> Action {
    if let Ok(Event::Key(key_event)) = event::read() {
        return handle_key_event(key_event);
    }
    Action::None
}

fn handle_key_event(event: KeyEvent) -> Action {
    match event.code {
        // Arrow keys
        KeyCode::Up => Action::Move(0, -1),
        KeyCode::Down => Action::Move(0, 1),
        KeyCode::Left => Action::Move(-1, 0),
        KeyCode::Right => Action::Move(1, 0),

        // WASD keys
        KeyCode::Char('w') | KeyCode::Char('W') => Action::Move(0, -1),
        KeyCode::Char('s') | KeyCode::Char('S') => Action::Move(0, 1),
        KeyCode::Char('a') | KeyCode::Char('A') => Action::Move(-1, 0),
        KeyCode::Char('d') | KeyCode::Char('D') => Action::Move(1, 0),

        // Quit
        KeyCode::Char('q') | KeyCode::Char('Q') => Action::Quit,
        KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
        KeyCode::Esc => Action::Quit,

        _ => Action::None,
    }
}

pub fn wait_for_key() {
    let _ = event::read();
}
