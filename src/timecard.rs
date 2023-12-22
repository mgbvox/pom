use crate::time;
use crate::time::timestamp;
use std::marker::PhantomData;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Phase {
    Work,
    Short,
    Long,
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Open;
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Closed;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Timecard<State = Open> {
    state: PhantomData<State>,
    pub start: f64,
    pub end: f64,
}

impl Timecard<Open> {
    pub fn begin() -> Timecard<Open> {
        Timecard {
            state: PhantomData::<Open>,
            start: timestamp(),
            end: -1.0,
        }
    }

    pub fn elapsed(&self) -> f64 {
        timestamp() - self.start
    }

    pub fn finalize(&self) -> Timecard<Closed> {
        Timecard {
            state: PhantomData::<Closed>,
            start: self.start,
            end: time::timestamp(),
        }
    }
}

impl Timecard<Closed> {
    pub fn duration(&self) -> f64 {
        self.end - self.start
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_can_get_duration() {
        let card = Timecard::begin();
        thread::sleep(Duration::from_millis(1));
        let finalized = card.finalize();
        assert!(finalized.duration() > 0.0);
    }
}
