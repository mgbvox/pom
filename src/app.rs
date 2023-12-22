use std::time::Instant;

use crate::timecard::{Closed, Open, Phase, Timecard};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Division {
    start: Instant,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct App {
    pub should_quit: bool,
    pub phase: Phase,
    pub divisions: Vec<Timecard<Closed>>,
    pub current_division: Option<Timecard<Open>>,
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
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
        self.current_division = Some(Timecard::punch_in());
    }

    pub fn punch_out(&mut self) {
        if let Some(div) = &self.current_division {
            self.divisions.push(div.punch_out());
            self.current_division = None
        }
    }

    pub fn duration(&self) -> f64 {
        self.divisions.iter().map(|d| d.duration()).sum()
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use pretty_assertions::{assert_eq, assert_ne};

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
