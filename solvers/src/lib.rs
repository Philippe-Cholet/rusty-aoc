use common::{bail, Day, Result, Year};

use crate::traits::AocSolver;

#[cfg(feature = "trace_alloc")]
#[global_allocator]
pub static ALLOCATOR: allocator::CounterAllocator = allocator::CounterAllocator::new();

#[cfg(feature = "trace_alloc")]
mod allocator;
#[macro_use]
mod macros;
mod traits;
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
