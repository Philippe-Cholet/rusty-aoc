use std::fmt::Write;
use std::process::Stdio;

use itertools::Itertools;

use common::prelude::*;
use crate::utils::OkIterator;

#[derive(Debug)]
struct Hailstone {
    position: [i128; 3],
    velocity: [i128; 3],
}

/// Never Tell Me The Odds
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let hailstones: Vec<Hailstone> = input.lines().map(str::parse).try_collect()?;
    Ok(match part {
        Part1 => {
            let area = if input == INPUTS[0] {
                (7, 27)
            } else {
                (200_000_000_000_000, 400_000_000_000_000)
            };
            hailstones
                .iter()
                .tuple_combinations()
                .filter(|(a, b)| a.xy_cross(b, area))
                .count()
        }
        Part2 => {
            let mut z3text = "\
                (declare-const x0 Real) (declare-const vx0 Real)
                (declare-const y0 Real) (declare-const vy0 Real)
                (declare-const z0 Real) (declare-const vz0 Real)"
                .to_owned();
            hailstones.into_iter().enumerate().try_for_each(
                |(
                    i,
                    Hailstone {
                        position: [px, py, pz],
                        velocity: [vx, vy, vz],
                    },
                )| {
                    z3text.write_fmt(format_args!(
                        "(declare-const t{i} Real)
                        (assert (= (+ (to_real {px}) (* t{i} (to_real {vx}))) (+ x0 (* t{i} vx0))))
                        (assert (= (+ (to_real {py}) (* t{i} (to_real {vy}))) (+ y0 (* t{i} vy0))))
                        (assert (= (+ (to_real {pz}) (* t{i} (to_real {vz}))) (+ z0 (* t{i} vz0))))"
                    ))
                },
            )?;
            z3text.push_str("(check-sat) (get-model)");
            #[cfg(debug_assertions)]
            z3text.push_str(
                "(echo \"position:\") (eval x0) (eval y0) (eval z0)
                (echo \"velocity:\") (eval vx0) (eval vy0) (eval vz0)
                (echo \"answer:\")",
            );
            z3text.push_str("(eval (+ x0 y0 z0))");
            let mut child = std::process::Command::new("z3")
                .arg("-in")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;
            let mut stdin = child.stdin.take().context("failed to get stdin")?;
            #[allow(clippy::expect_used)]
            std::thread::spawn(move || {
                use std::io::Write;
                stdin
                    .write_all(z3text.as_bytes())
                    .expect("failed to write to stdin");
            });
            let output = child.wait_with_output()?;
            ensure!(output.status.success());
            let text = String::from_utf8(output.stdout)?;
            #[cfg(debug_assertions)]
            println!("{text}");
            let res: f64 = text.lines().last().context("No output")?.parse()?;
            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            {
                res.round() as usize
            }
        }
    })
}

impl Hailstone {
    #[allow(clippy::cast_precision_loss, clippy::similar_names)]
    fn xy_cross(&self, other: &Self, area: (i128, i128)) -> bool {
        let [px1, py1, _] = self.position;
        let [vx1, vy1, _] = self.velocity;
        let [px2, py2, _] = other.position;
        let [vx2, vy2, _] = other.velocity;
        let vv = vy2 * vx1 - vy1 * vx2;
        if vv == 0 {
            // ~~TODO~~: handle special case? Aligned!
            // Based on part2, such case can not happen!
            return false;
        }
        let x = (py1 - py2) * vx1 * vx2 - vy1 * vx2 * px1 + vy2 * vx1 * px2;
        let x = x as f64 / vv as f64;
        let future = if vx1 > 0 {
            x >= px1 as f64
        } else {
            x <= px1 as f64
        };
        if !future {
            return false;
        }
        let future = if vx2 > 0 {
            x >= px2 as f64
        } else {
            x <= px2 as f64
        };
        if !future {
            return false;
        }
        let y = (px1 - px2) * vy1 * vy2 - vx1 * vy2 * py1 + vx2 * vy1 * py2;
        let y = y as f64 / -vv as f64;
        let future = if vy1 > 0 {
            y >= py1 as f64
        } else {
            y <= py1 as f64
        };
        if !future {
            return false;
        }
        let future = if vy2 > 0 {
            y >= py2 as f64
        } else {
            y <= py2 as f64
        };
        if !future {
            return false;
        }
        let area = (area.0 as f64, area.1 as f64);
        area.0 <= x && x <= area.1 && area.0 <= y && y <= area.1
    }
}

impl std::str::FromStr for Hailstone {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.replace(' ', "");
        let (p, v) = s.split_once('@').context("No @")?;
        Ok(Self {
            position: p.split(',').map(str::parse).ok_collect_array()?,
            velocity: v.split(',').map(str::parse).ok_collect_array()?,
        })
    }
}

test_solver! {
    "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
" => (2, 47),
    include_input!(23 24) => (15593, 757031940316991),
}
