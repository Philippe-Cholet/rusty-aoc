use itertools::Itertools;

use common::{prelude::*, Ok};
use crate::utils::OkIterator;

/// Ticket Translation
pub fn solver(part: Part, input: &str) -> Result<u64> {
    let (rules, my_ticket, nearby_tickets) = input
        .split("\n\n")
        .collect_tuple()
        .context("Not 3 blocks")?;
    let parse_range = |s: &str| {
        let (start, end) = s.split_once('-').context("No dash")?;
        Ok(start.parse::<u32>()?..=end.parse::<u32>()?)
    };
    let rules = rules
        .lines()
        .map(|line| {
            let (name, rs) = line.split_once(": ").context("No colon in rule line")?;
            let (r0, r1) = rs.split_once(" or ").context("No \"or\"")?;
            Ok((name, parse_range(r0)?, parse_range(r1)?))
        })
        .ok_collect_vec()?;
    let (header, my_ticket) = my_ticket.split_once('\n').context("Single line")?;
    ensure!(header == "your ticket:", "Wrong header 1");
    let my_ticket: Vec<u32> = my_ticket.split(',').map(str::parse).ok_collect()?;
    let mut lines = nearby_tickets.lines();
    ensure!(lines.next() == Some("nearby tickets:"), "Wrong header 2");
    let mut nearby_tickets: Vec<Vec<u32>> = lines
        .map(|line| line.split(',').map(str::parse).collect())
        .ok_collect()?;
    let mut invalid_sum = 0;
    nearby_tickets.retain(|ticket| {
        ticket.iter().all(|n| {
            let valid = rules
                .iter()
                .any(|(_, r0, r1)| r0.contains(n) || r1.contains(n));
            if !valid {
                invalid_sum += n;
            }
            valid
        })
    });
    Ok(match part {
        Part1 => invalid_sum.into(),
        Part2 => {
            nearby_tickets.push(my_ticket.clone());
            let nb_rules = rules.len();
            let mut all_values = (0..nb_rules)
                .map(|idx| {
                    let values = nearby_tickets.iter().map(|t| t[idx]).collect_vec();
                    let names = rules
                        .iter()
                        .filter_map(|(name, r0, r1)| {
                            values
                                .iter()
                                .all(|n| r0.contains(n) || r1.contains(n))
                                .then_some(name)
                        })
                        .collect_vec();
                    (idx, names, values) // NOTE: I thought `values` would be useful later but no.
                })
                .collect_vec();
            let mut name2idx = HashMap::new();
            let mut in_progress = true;
            while in_progress {
                in_progress = false;
                all_values.retain_mut(|(idx, names, _)| {
                    names.retain(|name| !name2idx.contains_key(name));
                    if let [name] = names[..] {
                        name2idx.insert(name, *idx);
                        in_progress = true;
                        false
                    } else {
                        true
                    }
                });
            }
            if cfg!(debug_assertions) {
                if !all_values.is_empty() {
                    println!("Left unsolved:");
                    for (idx, names, _) in all_values {
                        println!("{idx}: {names:?}");
                    }
                }
                println!("{name2idx:#?}");
            }
            ensure!(name2idx.len() == nb_rules, "Unsolved!");
            name2idx
                .into_iter()
                .filter_map(|(name, idx)| {
                    name.starts_with("departure")
                        .then_some(u64::from(my_ticket[idx]))
                })
                .product()
        }
    })
}

pub const INPUTS: [&str; 2] = [
    "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
",
    include_input!(20 16),
];

#[test]
fn solver_20_16() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 71);
    assert_eq!(solver(Part1, INPUTS[1])?, 18142);
    assert_eq!(solver(Part2, INPUTS[1])?, 1069784384303);
    Ok(())
}
