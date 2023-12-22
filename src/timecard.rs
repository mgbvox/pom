use std::marker::PhantomData;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Phase {
    Work,
    Short,
    Long,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Open;
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Closed;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Timecard<State = Open> {
    state: PhantomData<State>,
    start: f64,
    end: f64,
}

impl Timecard<Open> {
    pub fn punch_in() -> Timecard<Open> {
        Timecard {
            state: PhantomData::<Open>,
            start: timestamp(),
            end: -1.0,
        }
    }

    pub fn punch_out(&self) -> Timecard<Closed> {
        Timecard {
            state: PhantomData::<Closed>,
            start: self.start,
            end: timestamp(),
        }
    }
}

fn timestamp() -> f64 {
    let start = SystemTime::now();
    let ts = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as f64
        / 1e9;
    println!("ts: {}", ts);
    ts
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
        let card = Timecard::punch_in();
        thread::sleep(Duration::from_millis(1));
        let finalized = card.punch_out();
        assert!(finalized.duration() > 0.0);
    }
}
