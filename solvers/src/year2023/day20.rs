use std::collections::VecDeque;

use itertools::{Either, Itertools};

use common::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Off,
    On,
}

#[derive(Debug, Clone)]
enum Module<T> {
    Broadcast,
    // T is the id, a &str or an index.
    FlipFlop(T, Mode),
    // Remember previous pulses. None for unconnected modules.
    Conjunction(T, Vec<Option<Pulse>>),
    Output(T),
}

// None for the broadcaster, some otherwise.
type Names<'a> = Vec<Option<&'a str>>;
// The modules and the indexes of the modules where they send each pulse.
type Modules<T> = Vec<(Module<T>, Vec<T>)>;
// 4 conjunction modules feed another that send the low pulse to the output.
// Those 4 modules are feed periodically, we want `vec![(index, Some(period))]`.
type FinalPeriods<T> = Vec<(usize, Option<T>)>;

/// Pulse Propagation
pub fn solver(part: Part, input: &str) -> Result<u64> {
    let (names, mut modules) = parse(input)?;
    // Get the position of the unique Broadcast module.
    let broadcaster_idx = names
        .iter()
        .positions(Option::is_none)
        .exactly_one()
        .map_err(|it| format_err!("{} broadcaster(s)", it.count()))?;
    let mut params = match part {
        Part1 => Either::Left([0, 0]),
        Part2 => Either::Right(final_conjunctions(&names, &modules)?),
    };
    let mut queue = VecDeque::new();
    for step in 1..=part.value(1000, u16::MAX /*Arbitrary big value*/) {
        debug_assert!(queue.is_empty());
        // The button we push does not have an index so the broadcaster send it to itself.
        queue.push_back((Pulse::Low, broadcaster_idx, broadcaster_idx));
        while let Some((pulse, src_idx, idx)) = queue.pop_front() {
            match (pulse, &mut params) {
                (Pulse::Low, Either::Left([low, _])) => *low += 1,
                (Pulse::High, Either::Left([_, high])) => *high += 1,
                (Pulse::Low, Either::Right((_, periods))) => {
                    for (ref i, p) in periods.iter_mut() {
                        if *i == idx {
                            if p.is_none() {
                                *p = Some(step);
                            }
                            break;
                        }
                    }
                    // TODO: The product is right for my input, but the lcm should be used instead.
                    if let Some(res) = periods.iter().map(|(_, p)| p.map(u64::from)).product() {
                        #[cfg(debug_assertions)]
                        println!("Periods: {periods:?}");
                        return Ok(res);
                    }
                }
                (Pulse::High, Either::Right(_)) => {}
            }
            let (module, ref dsts) = &mut modules[idx];
            let new_pulse = match module {
                Module::Broadcast => {
                    debug_assert!(pulse == Pulse::Low);
                    pulse
                }
                Module::FlipFlop(_, mode) => {
                    if pulse == Pulse::High {
                        continue; // Ignore high pulses.
                    }
                    // Flip
                    match mode {
                        Mode::Off => {
                            *mode = Mode::On;
                            Pulse::High
                        }
                        Mode::On => {
                            *mode = Mode::Off;
                            Pulse::Low
                        }
                    }
                }
                Module::Conjunction(_, input_pulses) => {
                    // Remember pulse from inputs.
                    ensure!(input_pulses[src_idx].is_some(), "{}", src_idx);
                    input_pulses[src_idx] = Some(pulse);
                    if input_pulses.iter().all(|p| p != &Some(Pulse::Low)) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    }
                }
                Module::Output(ref i) => {
                    // Theorically, it could stop there but it won't!
                    if pulse == Pulse::Low && matches!(params, Either::Right((rx, _)) if rx == *i) {
                        return Ok(step.into());
                    }
                    continue; // Outputs only receive pulses!
                }
            };
            for &dst in dsts {
                queue.push_back((new_pulse, idx, dst));
            }
        }
    }
    match params {
        Either::Left([count_low_pulses, count_high_pulses]) => {
            #[cfg(debug_assertions)]
            println!("{count_low_pulses} low * {count_high_pulses} high");
            Ok(count_low_pulses * count_high_pulses)
        }
        Either::Right((_, periods)) => bail!("Undetected periods: {:?}", periods),
    }
}

fn parse(input: &str) -> Result<(Names, Modules<usize>)> {
    // Parse modules and destinations
    let mut modules: Modules<&str> = input
        .lines()
        .map(|line| {
            let (src, dsts) = line.split_once(" -> ").context("No -> delimiter")?;
            let src = if src == "broadcaster" {
                Module::Broadcast
            } else if let Some(s) = src.strip_prefix('%') {
                Module::FlipFlop(s, Mode::Off)
            } else if let Some(s) = src.strip_prefix('&') {
                Module::Conjunction(s, vec![])
            } else {
                bail!("Wrong module: {}", src);
            };
            let dsts = dsts.split(", ").collect();
            Ok((src, dsts))
        })
        .try_collect()?;
    // Replace names by indexes.
    let mut names = modules
        .iter()
        .map(|(module, _)| match module {
            Module::Broadcast => None,
            Module::FlipFlop(s, _) | Module::Conjunction(s, _) => Some(*s),
            Module::Output(_) => unreachable!(),
        })
        .collect_vec();
    let other_names = modules
        .iter()
        .flat_map(|(_, dsts)| dsts)
        .filter(|s| !names.contains(&Some(**s)))
        .copied()
        .sorted()
        .dedup()
        .collect_vec();
    for &new_name in &other_names {
        names.push(Some(new_name));
        modules.push((Module::Output(new_name), vec![]));
    }
    let modules: Modules<usize> = modules
        .iter()
        .enumerate()
        .map(|(i, (m, dsts))| {
            let m = match m {
                Module::Broadcast => Module::Broadcast,
                Module::FlipFlop(_, mode) => Module::FlipFlop(i, *mode),
                Module::Conjunction(_, _) => Module::Conjunction(i, vec![]),
                Module::Output(_) => Module::Output(i),
            };
            dsts.iter()
                .map(|dst| {
                    names
                        .iter()
                        .position(|d| d.as_ref() == Some(dst))
                        .with_context(|| format_err!("No position for {}", dst))
                })
                .try_collect()
                .map(|dsts| (m, dsts))
        })
        .try_collect()?;
    // Update input pulses for Conjunction modules.
    let modules = modules
        .iter()
        .map(|(m, dsts)| {
            let m = if let Module::Conjunction(i, _) = *m {
                // With low pulses by default.
                let input_pulses = modules
                    .iter()
                    .map(|(_, inputs)| inputs.contains(&i).then_some(Pulse::Low))
                    .collect();
                Module::Conjunction(i, input_pulses)
            } else {
                m.clone()
            };
            (m, dsts.clone())
        })
        .collect();
    Ok((names, modules))
}

fn final_conjunctions<T>(
    names: &Names,
    modules: &Modules<usize>,
) -> Result<(usize, FinalPeriods<T>)> {
    let rx_idx = names
        .iter()
        .position(|opt_name| opt_name == &Some("rx"))
        .context("No rx")?;
    let before_rx = modules
        .iter()
        .filter_map(|(module, dsts)| dsts.contains(&rx_idx).then_some(module))
        .collect_vec();
    ensure!(before_rx.len() == 1);
    let conjunction_before_rx_idx = match before_rx[..] {
        [Module::Conjunction(i, _)] => *i,
        _ => bail!("Unexpected (1)"),
    };
    let periods = modules
        .iter()
        .filter(|(_, dsts)| dsts.contains(&conjunction_before_rx_idx))
        .map(|(module, _)| match module {
            Module::Conjunction(i, _) => Ok(*i),
            _ => bail!("Unexpected (2)"),
        })
        .map_ok(|i| (i, None))
        .try_collect()?;
    Ok((rx_idx, periods))
}

pub const INPUTS: [&str; 3] = [
    "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
",
    "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
",
    include_input!(23 20),
];

#[test]
fn solver_23_20() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 32000000); // 8000 low * 4000 high
    assert_eq!(solver(Part1, INPUTS[1])?, 11687500); // 4250 low * 2750 high
    assert_eq!(solver(Part1, INPUTS[2])?, 731517480); // 17708 low * 41310 high
    assert_eq!(solver(Part2, INPUTS[2])?, 244178746156661); // 4049 * 3761 * 3931 * 4079
    Ok(())
}
