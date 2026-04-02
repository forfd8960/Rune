use std::io::Write;
use std::{io, thread::sleep, time::Duration};

use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    execute,
    terminal::{SetSize, SetTitle},
};
use rune::terminal;

fn main() {
    println!("Rune is a Text Editor written in Rust.");

    let _ = match terminal::RawModeGuard::new() {
        Ok(guard) => guard,
        Err(e) => {
            eprintln!("Failed to enter raw mode: {}", e);
            return;
        }
    };

    loop {
        sleep(Duration::from_secs(3));
        execute!(
            io::stdout(),
            Clear(ClearType::All),
            SetSize(10, 10),
            SetTitle("Rune")
        )
        .unwrap();
        write!(io::stdout(), "Hello, Rune!").unwrap();
    }
}
