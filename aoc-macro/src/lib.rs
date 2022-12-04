// TODO: We could probably do it without "syn" as we only have to get a simple string argument.
use std::path::PathBuf;

use cargo_toml::Manifest;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, LitStr};

/// Create a function to get the `solver` and `INPUTS` for a given `year` and `day`.
///
/// Requires `use common::*;` and `aoc**-**` dependencies.
///
/// ## Example: `make_aoc!("project-path")`
/// If "project-path/Cargo.toml" have the following dependencies:
/// - aoc21-01 ... aoc21-25
/// - aoc22-01 ... aoc22-05
///
/// then it creates a function named `aoc` like:
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
    let path = parse_macro_input!(input as LitStr).value();
    let matched_lines: Vec<_> = all_years_and_days(&path)
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

#[allow(clippy::unwrap_used)] // The compiler already say the provided path is wrong.
fn all_years_and_days(path: &str) -> Vec<(u8, u8)> {
    let cargo_path = PathBuf::from(path).join("Cargo.toml");
    Manifest::from_path(&cargo_path)
        .unwrap()
        .dependencies
        .into_keys()
        .filter_map(|dep| {
            // if dep == "aocYY-DD" then Some((y, d)) else None.
            let (year, day) = dep.strip_prefix("aoc")?.split_once('-')?;
            Some((year.parse().ok()?, day.parse().ok()?))
        })
        .collect()
}
