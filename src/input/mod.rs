use crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers};

use crate::errors::RuneError;

pub enum Event {
    Display(String),
    Quit,
}

pub fn read_event() -> Result<Option<Event>, RuneError> {
    // `event::read()` blocks until an event arrives.
    match event::read()? {
        // ── Ctrl+Q → quit ───────────────────────────────────────────────
        CEvent::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => return Ok(Some(Event::Quit)),

        CEvent::Key(KeyEvent {
            code: KeyCode::Char(c),
            modifiers,
            ..
        }) => {
            let display = if modifiers.contains(KeyModifiers::CONTROL) {
                format!("[Ctrl+{}]", c.to_uppercase())
            } else if modifiers.contains(KeyModifiers::ALT) {
                format!("[Alt+{}]", c)
            } else {
                format!("{}", c)
            };
            Ok(Some(Event::Display(display)))
        }

        CEvent::Key(KeyEvent { code, .. }) => {
            let label = match code {
                KeyCode::Enter => "[Enter]\r\n".to_string(),
                KeyCode::Backspace => "[Backspace]".to_string(),
                KeyCode::Delete => "[Delete]".to_string(),
                KeyCode::Tab => "[Tab]".to_string(),
                KeyCode::BackTab => "[Shift+Tab]".to_string(),
                KeyCode::Esc => "[Esc]".to_string(),
                KeyCode::Up => "[↑]".to_string(),
                KeyCode::Down => "[↓]".to_string(),
                KeyCode::Left => "[←]".to_string(),
                KeyCode::Right => "[→]".to_string(),
                KeyCode::Home => "[Home]".to_string(),
                KeyCode::End => "[End]".to_string(),
                KeyCode::PageUp => "[PageUp]".to_string(),
                KeyCode::PageDown => "[PageDown]".to_string(),
                KeyCode::F(n) => format!("[F{}]", n),
                _ => "[?]".to_string(),
            };
            Ok(Some(Event::Display(label)))
        }

        // ── Ignore mouse, resize, focus, paste, etc. ─────────────────────
        _ => Ok(None),
    }
}
