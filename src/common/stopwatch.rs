use bevy::platform::time::Instant;
use std::fmt::Display;
use tracing::info;

pub struct Stopwatch<S>
where
    S: Display,
{
    desc: S,
    start: Instant,
}

impl<S> Stopwatch<S>
where
    S: Display,
{
    pub fn new(desc: S) -> Self {
        Self {
            desc,
            start: Instant::now(),
        }
    }
}

impl<S> Drop for Stopwatch<S>
where
    S: Display,
{
    fn drop(&mut self) {
        let duration = Instant::now().duration_since(self.start);
        info!("{} took {:?}", self.desc, duration)
    }
}
