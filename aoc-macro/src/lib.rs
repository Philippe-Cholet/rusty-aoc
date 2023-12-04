use std::path::PathBuf;

use cargo_toml::Manifest;
use itertools::{Either, Itertools};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Nothing, parse_macro_input};

mod aoc_tests;

/// Create a function to get the `solver` and `INPUTS` for a given `year` and `day`.
///
/// Requires `common` and `aoc**-**` dependencies.
///
/// ## Example: `make_aoc!()`
/// If the project has the following dependencies:
/// - aoc21-01 ... aoc21-25
/// - aoc22-01 ... aoc22-05
///
/// then it creates a function named `aoc` like:
/// ```
/// fn aoc(year: Year, day: Day) -> Result<(Box<dyn AocSolver>, &'static [&'static str])> {
///     Ok(match (year, day) {
///         (Year2021, Day1)  => (Box::new(aoc21_01::solver), &aoc21_01::INPUTS),
///         ...
///         (Year2021, Day25) => (Box::new(aoc21_25::solver), &aoc21_25::INPUTS),
///         (Year2022, Day1)  => (Box::new(aoc22_01::solver), &aoc22_01::INPUTS),
///         ...
///         (Year2022, Day5)  => (Box::new(aoc22_05::solver), &aoc22_05::INPUTS),
///         _ => bail!("You have not solved AoC {:?} {:?}[...]", year, day),
///     })
/// }
/// ```
#[proc_macro]
pub fn make_aoc(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as Nothing);
    let matched_lines: Vec<_> = all_years_and_days()
        .iter()
        .map(|(y, d)| {
            let year = format_ident!("Year20{}", y);
            let day = format_ident!("Day{}", d);
            let dependency = format_ident!("aoc{}_{:0>2}", y, d);
            quote! {
                (::common::#year, ::common::#day) => (::std::boxed::Box::new(::#dependency::solver), &::#dependency::INPUTS),
            }
        })
        .collect();
    quote! {
        fn aoc(year: ::common::Year, day: ::common::Day) -> ::common::Result<(::std::boxed::Box<dyn ::common::AocSolver>, &'static [&'static str])> {
            ::common::Result::Ok(match (year, day) {
                #(#matched_lines)*
                _ => ::common::bail!("You have not solved AoC {:?} {:?}, what are you waiting for?!", year, day),
            })
        }
    }.into()
}

#[allow(clippy::expect_used)]
fn all_years_and_days() -> Vec<(u8, u8)> {
    let path = std::env::var("CARGO_MANIFEST_DIR")
        .expect("Environment variable CARGO_MANIFEST_DIR is missing");
    let cargo_path = PathBuf::from(path).join("Cargo.toml");
    Manifest::from_path(cargo_path)
        .expect("The path to the manifest file is wrong, sorry")
        .dependencies
        .into_keys()
        .filter_map(|dep| {
            // if dep == "aocYY-DD" then Some((y, d)) else None.
            let (year, day) = dep.strip_prefix("aoc")?.split_once('-')?;
            Some((year.parse().ok()?, day.parse().ok()?))
        })
        .collect()
}

#[allow(clippy::cast_possible_truncation)] // "d > u8::MAX" will not happen.
/// Create tests on other inputs, by year.
///
/// By prefixing _one answer_ OR _one day_ with `ignore`, the associated assertion
/// will be in a separated ignored test function.
///
/// ## Example:
/// ```
/// make_aoc_tests!(
///     username,
///     21 => (
///         ("Day1 Part1 answer", "Day1 Part2 answer"),
///         _, // Missing answers for Day2.
///         (_, "Day3 Part2 answer"), // Missing answer to Part1.
///         _, // Missing input file for Day4.
///         // ...
///         "Day25 Part1 answer",
///     ),
///     22 => (
///         ("Day1 Part1 answer", ignore "Day1 Part2 answer"),
///         (2, 3),
///         (4, 5),
///         (6, 7),
///         (8, "AB"),
///         (_, "CD"),
///         9,
///         _,
///         (10, _),
///         // It may stop before Day25.
///     ),
/// )
/// ```
/// will expand (for the available dependencies) to:
/// ```
/// #[allow(non_snake_case)]
/// #[test]
/// fn username_21() -> Result<()> {
///     let input = include_str!("username/2021/01.txt");
///     assert_eq!(aoc21_01::solver(Part1, input)?.to_string(), "Day1 Part1 answer", "username: year 2021 day 1 part 1");
///     // day1 part2
///     // days 2 to 25...
///     Ok(())
/// }
///
/// #[allow(non_snake_case)]
/// #[test]
/// fn username_22() -> Result<()> {
///     let input = include_str!("username/2022/01.txt");
///     assert_eq!(aoc22_01::solver(Part1, input)?.to_string(), "Day1 Part1 answer", "username: year 2022 day 1 part 1");
///     // ...
///     Ok(())
/// }
///
/// #[allow(non_snake_case)]
/// #[ignore]
/// #[test]
/// fn username_22_ignored() -> Result<()> {
///     let input = include_str!("username/2022/01.txt");
///     assert_eq!(aoc22_01::solver(Part2, input)?.to_string(), "Day1 Part2 answer", "username: year 2022 day 1 part 2");
///     // ...
///     Ok(())
/// }
/// ```
#[proc_macro]
pub fn make_aoc_tests(input: TokenStream) -> TokenStream {
    let aoc_tests::AocTests { name, year_tests } = parse_macro_input!(input);
    let years_days = all_years_and_days();
    let parts = [quote! { ::common::Part1 }, quote! { ::common::Part2 }];
    let funcs = year_tests.into_iter().flat_map(|(year, year_answers)| {
        let (all_tests, all_ignored_tests): (Vec<_>, Vec<_>) = year_answers
            .into_iter()
            .enumerate()
            .flat_map(|(d, answers)| {
                vec![false, true].into_iter().filter_map(|ignore| {
                    if answers[0].skip(ignore) && answers[1].skip(ignore) {
                        return None;
                    }
                    let day = d as u8 + 1;
                    years_days.contains(&(year, day)).then(|| {
                        let file = format!("{name}/20{year}/{day:0>2}.txt");
                        let assertions = answers.iter().enumerate().filter_map(|(idx, answer)| {
                            if answer.skip(ignore) {
                                return None;
                            }
                            answer.data.as_ref().map(|answer| {
                                let dep = format_ident!("aoc{}_{:0>2}", year, day);
                                let part = &parts[idx];
                                let expl = format!("{name}: year 20{year} day {day} part {}", idx + 1);
                                quote! {
                                    assert_eq!(::#dep::solver(#part, input)?.to_string(), #answer, #expl);
                                }
                            })
                        });
                        let tokens = quote! {
                            let input = include_str!(#file);
                            #(#assertions)*
                        };
                        (ignore, tokens)
                    })
                }).collect_vec()
            })
            .partition_map(|(ignore, tokens)| {
                if ignore { Either::Right(tokens) } else { Either::Left(tokens) }
            });
        let mut year_funcs = vec![];
        if !all_tests.is_empty() {
            let test_name = format_ident!("{}_{}", name, year);
            year_funcs.push(quote! {
                #[allow(non_snake_case)]
                #[test]
                fn #test_name() -> ::common::Result<()> {
                    #(#all_tests)*
                    ::common::Result::Ok(())
                }
            });
        }
        if !all_ignored_tests.is_empty() {
            let test_name = format_ident!("{}_{}_ignored", name, year);
            year_funcs.push(quote! {
                #[allow(non_snake_case)]
                #[ignore]
                #[test]
                fn #test_name() -> ::common::Result<()> {
                    #(#all_ignored_tests)*
                    ::common::Result::Ok(())
                }
            });
        }
        year_funcs
    });
    quote! {
        #(#funcs)*
    }
    .into()
}
