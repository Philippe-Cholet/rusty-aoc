use petgraph::{algo::toposort, prelude::DiGraphMap, visit::Dfs};

use common::{prelude::*, Ok};
use utils::OkIterator;

/// Handy Haversacks
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let data = input
        .lines()
        .map(|line| {
            let (owner, s) = line.split_once(" bags contain ").context("No contain")?;
            let content = if s == "no other bags." {
                vec![]
            } else {
                s.trim_end_matches('.')
                    .split(", ")
                    .map(|s| {
                        let (s, bags) = s.rsplit_once(' ').context("no whitespace")?;
                        Ok(match bags {
                            "bag" => (1, s.strip_prefix("1 ").context("not prefix 1")?),
                            "bags" => {
                                let (n, c) = s.split_once(' ').context("no 2nd whitespace")?;
                                (n.parse::<usize>()?, c)
                            }
                            _ => bail!("Not bag(s) but {}", bags),
                        })
                    })
                    .ok_collect_vec()?
            };
            Ok((owner, content))
        })
        .ok_collect_vec()?;
    // "u --> v" is an edge if "v-bags" contain some "u-bag(s)".
    let graph = DiGraphMap::<_, ()>::from_edges(
        data.iter()
            .flat_map(|(u, vs)| vs.iter().map(|(_, v)| (*v, *u))),
    );
    match part {
        Part1 => {
            let mut dfs = Dfs::new(&graph, "shiny gold");
            let mut nb_colors: usize = 0;
            while dfs.next(&graph).is_some() {
                nb_colors += 1;
            }
            nb_colors.checked_sub(1).context("shiny gold is missing")
        }
        Part2 => {
            let order =
                toposort(&graph, None).map_err(|_| format_err!("Cycle of bags detected"))?;
            // No need to continue after shiny gold bags.
            let idx = order
                .iter()
                .position(|x| x == &"shiny gold")
                .context("shiny gold is missing")?;
            #[cfg(debug_assertions)]
            println!("{order:?}");
            let contents: HashMap<_, _> = data.into_iter().collect();
            let mut nb_bags_per_color = HashMap::new();
            for owner in &order[..=idx] {
                nb_bags_per_color.insert(
                    owner,
                    contents
                        .get(owner)
                        .context("Missing bag owner")?
                        .iter()
                        .map(|(nb, color)| {
                            nb_bags_per_color
                                .get(color)
                                .map(|sub_bags| (sub_bags + 1) * nb)
                                .context("Missing sub-bag, wrong order of bags?!")
                        })
                        .ok_sum::<usize>()?,
                );
            }
            #[cfg(debug_assertions)]
            println!("{nb_bags_per_color:?}");
            nb_bags_per_color
                .get(&"shiny gold")
                .copied()
                .context("shiny gold is missing")
        }
    }
}

pub const INPUTS: [&str; 3] = [
    "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
",
    "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
",
    include_input!(20 07),
];

#[test]
fn solver_20_07() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 4);
    assert_eq!(solver(Part1, INPUTS[2])?, 274);
    assert_eq!(solver(Part2, INPUTS[0])?, 32);
    assert_eq!(solver(Part2, INPUTS[1])?, 126);
    assert_eq!(solver(Part2, INPUTS[2])?, 158730);
    Ok(())
}
