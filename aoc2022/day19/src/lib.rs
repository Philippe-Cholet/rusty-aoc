// I wanted to try another solver: "GLPK"
// choco install -y glpk
// use good_lp::solvers::lp_solvers::{GlpkSolver, LpSolver};
// let glpk_solver = LpSolver(GlpkSolver::new().command_name("glpsol".to_owned()));
// The command line failed for some reason, maybe its version, I don't know.
use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel};

use common::{ensure, Error, Ok, Part, Part1, Part2, Result};
use utils::OkIterator;

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore_robot_cost: i32,             // Ore
    clay_robot_cost: i32,            // Ore
    obsidian_robot_cost: (i32, i32), // Ore, Clay
    geode_robot_cost: (i32, i32),    // Ore, Obsidian
}

impl std::str::FromStr for Blueprint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let v: Vec<_> = s
            .replace(':', "")
            .split_whitespace()
            .filter_map(|t| t.parse::<i32>().ok())
            .collect();
        ensure!(v.len() == 7, "A blueprint line should have 7 integers");
        Ok(Self {
            id: v[0],
            ore_robot_cost: v[1],
            clay_robot_cost: v[2],
            obsidian_robot_cost: (v[3], v[4]),
            geode_robot_cost: (v[5], v[6]),
        })
    }
}

impl Blueprint {
    #[allow(clippy::cast_possible_truncation)]
    fn maximise_geodes(&self, minutes: usize) -> Result<i32> {
        let mut problem_vars = variables!();
        let zero = Expression::default;
        // Minerals/robots: [ore, clay, obsidian, geodes]
        let mut minerals: [Expression; 4] = [zero(), zero(), zero(), zero()];
        let mut robots: [Expression; 4] = [zero() + 1, zero(), zero(), zero()];
        let mut constaints = vec![];
        for _minute in 1..=minutes {
            let b1 = problem_vars.add(variable().integer().min(0));
            let b2 = problem_vars.add(variable().integer().min(0));
            let b3 = problem_vars.add(variable().integer().min(0));
            let b4 = problem_vars.add(variable().integer().min(0));
            // The robot factory can only build one robot each minute.
            constaints.push((b1 + b2 + b3 + b4).leq(1));
            // Spend some minerals to build robots.
            minerals[0] -= b1 * self.ore_robot_cost
                + b2 * self.clay_robot_cost
                + b3 * self.obsidian_robot_cost.0
                + b4 * self.geode_robot_cost.0;
            minerals[1] -= b3 * self.obsidian_robot_cost.1;
            minerals[2] -= b4 * self.geode_robot_cost.1;
            // Ensure we do not run out of minerals.
            constaints.extend(minerals.clone().map(|q| q.geq(0)));
            // Previously built robots have collected some minerals.
            for (mineral, collected) in minerals.iter_mut().zip(robots.iter()) {
                *mineral += collected;
            }
            // New robots are ready.
            for (robot, built) in robots.iter_mut().zip([b1, b2, b3, b4]) {
                *robot += built;
            }
        }
        let nb_geodes = &minerals[3];
        let mut problem = problem_vars.maximise(nb_geodes).using(default_solver);
        for quantity in constaints {
            problem.add_constraint(quantity);
        }
        let max_geodes = problem.solve()?.eval(nb_geodes);
        Ok(max_geodes as i32)
    }
}

/// Not Enough Minerals
pub fn solver(part: Part, input: &str) -> Result<String> {
    let data: Vec<Blueprint> = input.lines().map(str::parse).ok_collect()?;
    let result: i32 = match part {
        Part1 => data
            .iter()
            .map(|bp| Ok(bp.id * bp.maximise_geodes(24)?))
            .ok_sum()?,
        Part2 => data
            .iter()
            // .take(3)
            .filter(|bp| bp.id <= 3)
            .map(|bp| bp.maximise_geodes(32))
            .ok_product()?,
    };
    Ok(result.to_string())
}

pub const INPUTS: [&str; 2] = [
    "Blueprint 1: \
Each ore robot costs 4 ore. \
Each clay robot costs 2 ore. \
Each obsidian robot costs 3 ore and 14 clay. \
Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: \
Each ore robot costs 2 ore. \
Each clay robot costs 3 ore. \
Each obsidian robot costs 3 ore and 8 clay. \
Each geode robot costs 3 ore and 12 obsidian.
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_19() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "33");
    assert_eq!(solver(Part1, INPUTS[1])?, "1981");
    assert_eq!(solver(Part2, INPUTS[0])?, (56 * 62).to_string());
    assert_eq!(solver(Part2, INPUTS[1])?, "10962");
    Ok(())
}
