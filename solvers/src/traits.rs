use std::time::{Duration, Instant};

use common::{Part, Result};

pub trait AocSolver {
    fn solve(&self, part: Part, input: &str) -> Result<String>;
    fn timed_solve(&self, part: Part, input: &str) -> Result<(String, Duration)>;
}

impl<T, F> AocSolver for F
where
    T: std::fmt::Display,
    F: Fn(Part, &str) -> Result<T>,
{
    fn solve(&self, part: Part, input: &str) -> Result<String> {
        self(part, input).map(|t| t.to_string())
    }

    fn timed_solve(&self, part: Part, input: &str) -> Result<(String, Duration)> {
        let now = Instant::now();
        let t = self(part, input)?;
        let elapsed = now.elapsed();
        Ok((t.to_string(), elapsed))
    }
}
