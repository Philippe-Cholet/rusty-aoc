use itertools::Itertools;

use common::prelude::*;

/// Cube Conundrum
pub fn solver(part: Part, input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| {
            let (game, line) = line.split_once(": ").context("colon delimiter")?;
            let id: u32 = game.strip_prefix("Game ").context("Game prefix")?.parse()?;
            line.split("; ")
                .map(|game_part| {
                    let mut rgb = [0; 3];
                    for n_color in game_part.split(", ") {
                        let (n, color) = n_color.split_once(' ').context("space before color")?;
                        let idx = match color {
                            "red" => 0,
                            "green" => 1,
                            "blue" => 2,
                            _ => bail!("Wrong color: {color}"),
                        };
                        ensure!(rgb[idx] == 0, "the same color twice in a game part");
                        rgb[idx] = n.parse()?;
                    }
                    Ok(rgb)
                })
                .fold_ok([0; 3], |[r0, g0, b0], [r, g, b]| {
                    [r0.max(r), g0.max(g), b0.max(b)]
                })
                .map(|rgb| (id, rgb))
        })
        .process_results(|it| match part {
            Part1 => it
                .filter_map(|(id, [r, g, b])| (r <= 12 && g <= 13 && b <= 14).then_some(id))
                .sum(),
            Part2 => it.map(|(_, [r, g, b])| r * g * b).sum(),
        })
}

test_solver! {
    "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
" => (8, 2286),
    include_input!(23 02) => (2685, 83707),
}
