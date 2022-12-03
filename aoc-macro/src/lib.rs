// TODO: The ideal would be to parse `aocYY-DD` dependencies instead of asking for them?!
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse, parse_macro_input, LitInt, Token};

/// Create a function to get the `solver` and `INPUTS` for a given `year` and `day`.
///
/// Requires `use common::*;` and `aoc**-**` dependencies.
///
/// ## Example:
/// `make_aoc!(21 25, 22 5)` if you have a solver for the 25 days of aoc2021 and the first 5 of aoc2022.
///
/// It creates a function named `aoc` like:
/// ```
/// fn aoc(year: Year, day: Day) -> Result<(AocSolver, &'static [&'static str])> {
///     Ok(match (year, day) {
///         (Year2021, Day1)  => (aoc21_01::solver, &aoc21_01::INPUTS),
///         ...
///         (Year2021, Day25) => (aoc21_25::solver, &aoc21_25::INPUTS),
///         (Year2022, Day1)  => (aoc22_01::solver, &aoc22_01::INPUTS),
///         ...
///         (Year2022, Day5)  => (aoc22_05::solver, &aoc22_05::INPUTS),
///         _ => bail!("You have not solved AoC {:?} {:?}[...]", year, day),
///     })
/// }
/// ```
#[proc_macro]
pub fn make_aoc(input: TokenStream) -> TokenStream {
    let matched_lines: Vec<_> = parse_macro_input!(input as YearsDays)
        .all_years_and_days()
        .iter()
        .map(|(y, d)| {
            let year = format_ident!("Year20{}", y);
            let day = format_ident!("Day{}", d);
            let dependency = format_ident!("aoc{}_{:0>2}", y, d);
            quote! {
                (#year, #day) => (#dependency::solver, &#dependency::INPUTS),
            }
        })
        .collect();
    quote! {
        fn aoc(year: Year, day: Day) -> Result<(AocSolver, &'static [&'static str])> {
            Ok(match (year, day) {
                #(#matched_lines)*
                _ => bail!("You have not solved AoC {:?} {:?}, what are you waiting for?!", year, day),
            })
        }
    }.into()
}

// Parse the (year, day_max) integers given as arguments to the macro and get all (year, day).
struct YearsDays(Vec<(u8, u8)>);

impl YearsDays {
    pub fn all_years_and_days(&self) -> Vec<(u8, u8)> {
        self.0
            .iter()
            .flat_map(|(year, day_max)| (1..=*day_max).map(|day| (*year, day)))
            .collect()
    }
}

impl parse::Parse for YearsDays {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let mut year_days = vec![];
        while !input.is_empty() {
            let year = input.parse::<LitInt>()?.base10_parse()?;
            let days = input.parse::<LitInt>()?.base10_parse()?;
            year_days.push((year, days));
            // Can end without a comma after a pair of integers...
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            // ...or with a comma, as you wish!
        }
        Ok(Self(year_days))
    }
}
