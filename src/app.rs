use std::time::{Duration, Instant};
use color_eyre::owo_colors::CssColors::SeaGreen;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Phase {
    Work,
    Short,
    Long,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Division {
    start: Instant,
}


impl Division {
    fn new() -> Self {
        Self {
            start: Instant::now()
        }
    }

    fn duration(&self) -> Duration {
        self.start.elapsed()
    }
}

#[derive(Debug,PartialOrd, PartialEq)]
pub struct App {
    pub should_quit: bool,
    pub phase: Phase,
    pub divisions: Vec<Division>,
    pub current_division: Option<Division>,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            phase: Phase::Work,
            divisions: Vec::new(),
            current_division: None,
        }
    }

    pub fn punch_in(&mut self) {
        self.current_division = Some(Division::new());
    }

    pub fn punch_out(&mut self) {
        match &self.current_division {
            Some(div) => {
                self.divisions.push(div.clone());
                self.current_division = None
            }
            None => { println!("No division to punch out from!") }
        }
    }

    pub fn duration(&self) -> f32 {
        self.divisions.iter().map(|d| d.duration().as_millis() as f32).sum()
    }
}


#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use super::*;

    #[test]
    fn test_punch_in_out() {
        let mut app = App::new();
        // no divisions, no current division
        assert_eq!(app.divisions.len(), 0);
        assert_eq!(app.current_division, None);
        // start tracking
        app.punch_in();
        // this starts a division
        assert_ne!(app.current_division, None);
        // stop tracking
        app.punch_out();
        // this finishes the current division and adds it to the divisions vec
        assert_eq!(app.current_division, None);
        assert_eq!(app.divisions.len(), 1);
    }


    #[test]
    fn test_calc_duration_from_division() {
        let mut app = App::new();
        app.punch_in();
        thread::sleep(Duration::from_millis(1));
        app.punch_out();
        assert!(app.duration() > 0.0);
    }
}