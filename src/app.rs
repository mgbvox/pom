use crate::time::timestamp;
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
        if self.current_division.is_none() {
            self.current_division = Some(Timecard::begin());
        }
    }

    pub fn punch_out(&mut self) {
        if let Some(div) = &self.current_division {
            self.divisions.push(div.finalize());
            self.current_division = None
        }
    }

    pub fn duration(&self) -> f64 {
        let division_sum = self.divisions.iter().map(|d| d.duration()).sum();
        if let Some(div) = self.current_division {
            // all divisions, plus time elapsed on current division
            division_sum + div.elapsed()
        } else {
            division_sum
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use mockall::predicate::*;
    use mockall::*;
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    fn sleep(ms: u64) {
        thread::sleep(Duration::from_millis(ms));
    }

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
        sleep(1);
        app.punch_out();
        assert!(app.duration() > 0.0);
    }

    #[test]
    fn test_punch_in_multiple_times() {
        let mut app = App::new();
        // no div at first
        assert!(app.current_division.is_none());
        app.punch_in();
        assert!(app.current_division.is_some());
        // nothing should change on second punch in
        let first_card = app.current_division;
        app.punch_in();
        assert_eq!(first_card, app.current_division);
        assert_eq!(app.divisions.len(), 0)
    }

    #[test]
    fn test_multiple_in_out_duration_correct() {
        let mut app = App::new();
        let now = Instant::now();
        app.punch_in();
        sleep(5);
        app.punch_out();
        app.punch_in();
        sleep(50);
        app.punch_out();
        // should be 55ish ms of gap
        let dur = now.elapsed().as_nanos() as f64 / 1e9;
        let appdur = app.duration();
        let diff = dur - appdur;
        // todo: probably want to mock with specific values
        assert!(diff > 0.0);
    }

    #[test]
    fn test_duration_increases_when_card_open() {
        let mut app = App::new();
        app.punch_in();
        sleep(1);
        let d1 = app.duration();
        sleep(1);
        assert_eq!(app.divisions.len(), 0);
        assert!(d1 < app.duration());
    }
}
