use color_eyre::eyre::Result;
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};

use crate::app::App;
use crate::time::timestamp;

mod app;
mod time;
mod timecard;

fn main() -> Result<()> {
    color_eyre::install()?;

    // startup: Enable raw mode for the terminal, giving us fine control over user input
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    // Initialize the terminal backend using crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            f.render_widget(
                Paragraph::new(format!(
                    "dur: {}, splits: {}, ts: {}",
                    app.duration(),
                    app.divisions.len(),
                    timestamp()
                )),
                f.size(),
            )
        })?;

        // Paragraph::new(format!("Divs: {:?}\nDuration: {}", app.divisions, app.duration())), f.size());

        // Check for user input every 250 milliseconds
        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            // If a key event occurs, handle it
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('i') => app.punch_in(),
                        crossterm::event::KeyCode::Char('o') => app.punch_out(),
                        crossterm::event::KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
    }

    // shutdown down: reset terminal back to original state
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
