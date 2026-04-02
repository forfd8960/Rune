use crossterm::{
    cursor,
    event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use rune::input::{Event, read_event};
use std::io::{self, Write, stdout};

fn main() -> io::Result<()> {
    // Enter raw mode so keypresses are delivered immediately,
    // without waiting for Enter, and without echo.
    enable_raw_mode()?;

    let mut stdout = stdout();

    // Switch to the alternate screen buffer (optional but clean).
    execute!(stdout, EnterAlternateScreen, cursor::MoveTo(0, 0))?;

    println!("Type anything. Press Ctrl+Q to quit.\r");
    println!("─────────────────────────────────────\r");

    loop {
        match read_event() {
            Ok(Some(Event::Display(label))) => {
                // Display the key label on the terminal
                execute!(stdout, Print(label))?;
                stdout.flush()?;
            }
            Ok(Some(Event::Quit)) => break,
            Ok(None) => {} // Ignore other events
            Err(e) => {
                eprintln!("Error reading event: {}", e);
                break;
            }
        }
    }

    // ── Cleanup ──────────────────────────────────────────────────────────────
    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    println!("Bye!");

    Ok(())
}
