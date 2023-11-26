use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};

use color_eyre::eyre::{Result};

pub mod app;

use crate::app::App;


fn main() -> Result<()> {
    color_eyre::install()?;


    // startup: Enable raw mode for the terminal, giving us fine control over user input
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    // Initialize the terminal backend using crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // start out with counter example, will migrate to pom soon
    let mut counter = 0;

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            f.render_widget(Paragraph::new(format!("Divs: {:?}\nDuration: {}", app.divisions, app.duration())), f.size());
        })?;

        // Check for user input every 250 milliseconds
        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            // If a key event occurs, handle it
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('j') => counter += 1,
                        crossterm::event::KeyCode::Char('k') => counter -= 1,
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