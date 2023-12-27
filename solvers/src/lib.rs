use std::time::{Duration, Instant};

use common::{bail, Day, Part, Result, Year};

#[macro_use]
mod macros;
pub mod utils;

macro_rules! pub_mod {
    ($($year:literal => $($day:literal)*),* $(,)?) => {
        paste::paste! {
            $(
                pub mod [<year20 $year>] {
                    $(pub mod [<day $day>];)*
                }
            )*

            #[allow(clippy::zero_prefixed_literal)]
            pub fn aoc(year: Year, day: Day) -> Result<(Box<dyn AocSolver>, &'static [&'static str])> {
                Ok(match (year, u8::from(day)) {
                    $(
                        $(
                            (Year::[<Year20 $year>], $day) => (
                                Box::new(self::[<year20 $year>]::[<day $day>]::solver),
                                &self::[<year20 $year>]::[<day $day>]::INPUTS,
                            ),
                        )*
                    )*
                    _ => bail!("No solver for {:?} {:?} yet!", year, day),
                })
            }
        }
    };
}

pub_mod! {
    15 => 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 /*22*/ /*23*/ /*24*/ /*25*/,
    // 16 => /*01*/ /*02*/ /*03*/ /*04*/ /*05*/ /*06*/ /*07*/ /*08*/ /*09*/ /*10*/ /*11*/ /*12*/ /*13*/ /*14*/ /*15*/ /*16*/ /*17*/ /*18*/ /*19*/ /*20*/ /*21*/ /*22*/ /*23*/ /*24*/ /*25*/,
    // 17 => /*01*/ /*02*/ /*03*/ /*04*/ /*05*/ /*06*/ /*07*/ /*08*/ /*09*/ /*10*/ /*11*/ /*12*/ /*13*/ /*14*/ /*15*/ /*16*/ /*17*/ /*18*/ /*19*/ /*20*/ /*21*/ /*22*/ /*23*/ /*24*/ /*25*/,
    // 18 => /*01*/ /*02*/ /*03*/ /*04*/ /*05*/ /*06*/ /*07*/ /*08*/ /*09*/ /*10*/ /*11*/ /*12*/ /*13*/ /*14*/ /*15*/ /*16*/ /*17*/ /*18*/ /*19*/ /*20*/ /*21*/ /*22*/ /*23*/ /*24*/ /*25*/,
    // 19 => /*01*/ /*02*/ /*03*/ /*04*/ /*05*/ /*06*/ /*07*/ /*08*/ /*09*/ /*10*/ /*11*/ /*12*/ /*13*/ /*14*/ /*15*/ /*16*/ /*17*/ /*18*/ /*19*/ /*20*/ /*21*/ /*22*/ /*23*/ /*24*/ /*25*/,
    20 => 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25,
    21 => 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25,
    22 => 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25,
    23 => 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25,
}

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
