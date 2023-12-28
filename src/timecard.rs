use crate::time::timestamp;
use crate::PomArgs;
use std::marker::PhantomData;
use std::time::Duration;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Phase {
    Work,
    Short,
    Long,
}

impl Phase {
    pub fn from_char(value: &char) -> Option<Self> {
        if let Some(cast) = value.to_lowercase().next() {
            return match cast {
                'w' => Some(Phase::Work),
                's' => Some(Phase::Short),
                'l' => Some(Phase::Long),
                _ => None,
            };
        }
        None
    }

    pub fn to_duration(&self, config: &PomArgs) -> Duration {
        let time = match self {
            Phase::Work => &config.work,
            Phase::Short => &config.short,
            Phase::Long => &config.long,
        };
        let parts: Vec<&str> = time.split(':').collect();
        let minutes: u64 = parts[0].parse().expect("Invalid minutes");
        let seconds: u64 = parts[1].parse().expect("Invalid seconds");
        Duration::new(minutes * 60 + seconds, 0)
    }
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
            end: timestamp(),
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

    #[test]
    fn test_phase_from_char() {
        let chars = "wslWSL";
        for c in chars.chars() {
            assert!(Phase::from_char(&c).is_some());
            assert!(
                Phase::from_char(&c)
                    .unwrap()
                    .to_duration(&PomArgs::default())
                    .as_secs()
                    > 0
            );
        }
    }
}
