use std::path::PathBuf;

use criterion::{BenchmarkId, Criterion};

use common::{Day, Day25, Part1, Part2, Year};
use solvers::aoc;

macro_rules! bench_input {
    ($group:ident, $day:ident, $solver:ident, $id:expr, $input:expr) => {
        $group.bench_with_input(BenchmarkId::new("1", $id), $input, |b, input| {
            b.iter(|| $solver.solve(Part1, input).unwrap())
        });
        if $day != Day25 {
            $group.bench_with_input(BenchmarkId::new("2", $id), $input, |b, input| {
                b.iter(|| $solver.solve(Part2, input).unwrap())
            });
        }
    };
}

/// Usage example: `cargo bench --bench solvers 22-16/2`
/// or with the "speed" alias: `cargo speed 22-16/2`.
///
/// Full ids are `YY-DD/P/` and `YY-DD/P/OTHER_NAME` (`Y D P` standing for year, day and part).
///
/// Since it's regexes, use `$` to only bench my inputs `22-16/2/$`.
fn main() {
    let other_dir = PathBuf::from("../inputs/other");
    let other_names = std::fs::read_dir(&other_dir)
        .map(|read_dir| {
            read_dir
                .filter_map(|dir_entry| {
                    let dir_entry = dir_entry.ok()?;
                    dir_entry
                        .file_type()
                        .ok()
                        .and_then(|file_type| file_type.is_dir().then(|| dir_entry.file_name()))
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let mut criterion = Criterion::default().configure_from_args();
    for year in Year::ALL {
        for day in Day::ALL {
            let Ok((solver, inputs)) = aoc(year, day) else {
                continue;
            };
            let group_name = format!("{}-{:0>2}", u8::from(year), u8::from(day));
            let txt_day = format!("{}/{:0>2}.txt", i32::from(year), u8::from(day));
            let mut group = criterion.benchmark_group(group_name);
            if let Some(input) = inputs.last() {
                bench_input!(group, day, solver, "", input);
            };
            for name in &other_names {
                let txt_path = other_dir.join(name).join(&txt_day);
                if let Ok(other_input) = std::fs::read_to_string(txt_path) {
                    bench_input!(group, day, solver, name.to_string_lossy(), &other_input);
                }
            }
            group.finish();
        }
    }
    criterion.final_summary();
}
