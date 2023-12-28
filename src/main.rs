use clap::Parser;
use color_eyre::eyre::Result;
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};
use std::ops::Index;

use crate::app::App;
use crate::time::timestamp;
use crate::timecard::Phase;

mod app;
mod time;
mod timecard;

#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Matthew Billman", about = "Pomodoro Timer")]
struct PomArgs {
    /// Duration for work in mm:ss
    #[arg(short, long, default_value = "25:00")]
    work: String,

    /// Duration for a short break in mm:ss
    #[arg(short, long, default_value = "5:00")]
    short: String,

    /// Duration for a long break in mm:ss
    #[arg(short, long, default_value = "15:00")]
    long: String,

    #[arg(short, long, default_value = "wswswl")]
    pattern: String,
}

impl Default for PomArgs {
    fn default() -> Self {
        PomArgs {
            work: "25:00".to_string(),
            short: "5:00".to_string(),
            long: "15:00".to_string(),
            pattern: "wswswl".to_string(),
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = PomArgs::parse();

    // startup: Enable raw mode for the terminal, giving us fine control over user input
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    // Initialize the terminal backend using crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut app = App::new();

    let mut index = 0;
    let modulus = args.pattern.len();

    loop {
        let step_idx = index % modulus;
        let phase_char = &args
            .pattern
            .chars()
            .nth(step_idx)
            .expect("Failure to get pattern char!");

        let phase = Phase::from_char(phase_char).expect(&format!(
            "Failed to cast pattern char <{}> to Phase!",
            phase_char
        ));

        let remaining = phase.to_duration(&args).as_secs() as f64 - app.duration();

        terminal.draw(|f| {
            f.render_widget(
                Paragraph::new(format!(
                    "dur: {}, splits: {}, ts: {}, phase: {:?}, duration: {:?}, remaining: {}",
                    app.duration(),
                    app.divisions.len(),
                    timestamp(),
                    phase,
                    phase.to_duration(&args),
                    remaining,
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

#[cfg(test)]
mod tests {
    // Test cli with "https://crates.io/crates/trycmd"?
    fn test_a_thing() {
        todo!();
    }
}
