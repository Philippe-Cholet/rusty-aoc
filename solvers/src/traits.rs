use std::time::{Duration, Instant};

use common::{Part, Result};

#[cfg(feature = "trace_alloc")]
use crate::{allocator::AllocInfos, ALLOCATOR};

trait SolverAnswer: std::fmt::Display {}

pub trait AocSolver {
    fn solve(&self, part: Part, input: &str) -> Result<String>;
    fn timed_solve(&self, part: Part, input: &str) -> Result<(String, Duration)>;
    #[cfg(feature = "trace_alloc")]
    fn alloc_solve(&self, part: Part, input: &str) -> Result<(String, AllocInfos)>;
}

#[cfg(test)]
/// Useful for a nice `test_solver` macro.
pub trait TestAnswers<T>: Sized {
    fn test_answers(&self) -> [Option<&T>; 2];
}

impl SolverAnswer for String {}
impl SolverAnswer for &'static str {}
impl SolverAnswer for u8 {}
impl SolverAnswer for u16 {}
impl SolverAnswer for u32 {}
impl SolverAnswer for usize {}
impl SolverAnswer for u64 {}
impl SolverAnswer for u128 {}
impl SolverAnswer for i8 {}
impl SolverAnswer for i16 {}
impl SolverAnswer for i32 {}
impl SolverAnswer for isize {}
impl SolverAnswer for i64 {}
impl SolverAnswer for i128 {}

impl<T, F> AocSolver for F
where
    T: SolverAnswer,
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

    #[cfg(feature = "trace_alloc")]
    fn alloc_solve(&self, part: Part, input: &str) -> Result<(String, AllocInfos)> {
        ALLOCATOR.reset();
        let t = self(part, input)?;
        let infos = ALLOCATOR.infos();
        Ok((t.to_string(), infos))
    }
}

#[cfg(test)]
impl<T: SolverAnswer> TestAnswers<T> for T {
    fn test_answers(&self) -> [Option<&T>; 2] {
        [Some(self), None]
    }
}

#[cfg(test)]
impl<T: SolverAnswer> TestAnswers<T> for (T,) {
    fn test_answers(&self) -> [Option<&T>; 2] {
        [Some(&self.0), None]
    }
}

#[cfg(test)]
impl<T: SolverAnswer> TestAnswers<T> for ((), T) {
    fn test_answers(&self) -> [Option<&T>; 2] {
        [None, Some(&self.1)]
    }
}

#[cfg(test)]
impl<T: SolverAnswer> TestAnswers<T> for (T, T) {
    fn test_answers(&self) -> [Option<&T>; 2] {
        [&self.0, &self.1].map(Some)
    }
}
