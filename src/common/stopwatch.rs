use std::fmt::Display;
use std::time::Instant;
use tracing::info;

pub struct Stopwatch<S>
where
    S: Display,
{
    desc: S,

    #[cfg(not(target_family = "wasm"))]
    start: Instant,
}

impl<S> Stopwatch<S>
where
    S: Display,
{
    pub fn new(desc: S) -> Self {
        Self {
            desc,
            #[cfg(not(target_family = "wasm"))]
            start: Instant::now(),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl<S> Drop for Stopwatch<S>
where
    S: Display,
{
    fn drop(&mut self) {
        let duration = Instant::now().duration_since(self.start);
        info!("{} took {:?}", self.desc, duration)
    }
}
