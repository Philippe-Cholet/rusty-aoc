use common::prelude::*;

#[derive(Debug)]
struct Crt {
    x: i32,
    cycle: i32,
    strengths: [i32; 6],
    screen: [bool; 240],
}

impl Crt {
    const fn new() -> Self {
        Self {
            x: 1,
            cycle: 0,
            strengths: [0; 6],
            screen: [false; 240],
        }
    }

    #[allow(clippy::cast_sign_loss)] // cycle is always positive
    fn noop(&mut self) {
        self.screen[self.cycle as usize] = self.x.abs_diff(self.cycle % 40) <= 1;

        self.cycle += 1;

        if self.cycle % 40 == 20 {
            self.strengths[(self.cycle / 40) as usize] = self.cycle * self.x;
        }
    }

    fn addx(&mut self, value: i32) {
        self.noop();
        self.noop();
        self.x += value;
    }

    fn signal_strength_sum(&self) -> i32 {
        self.strengths.iter().sum()
    }

    const fn is_done(&self) -> bool {
        self.cycle == 240
    }
}

impl std::fmt::Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.screen.chunks_exact(40) {
            for visible in row {
                write!(f, "{}", if *visible { '█' } else { '░' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Cathode-Ray Tube
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut crt = Crt::new();
    for line in input.lines() {
        if line == "noop" {
            crt.noop();
        } else if let Some(addx) = line.strip_prefix("addx ") {
            crt.addx(addx.parse()?);
        } else {
            bail!("Wrong command: {}", line);
        }
    }
    ensure!(crt.is_done(), "Not 240 cycles");
    Ok(match part {
        Part1 => crt.signal_strength_sum().to_string(),
        Part2 => crt.to_string(),
    })
}

test_solver! {
    "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
" => ("13140", "\
██░░██░░██░░██░░██░░██░░██░░██░░██░░██░░
███░░░███░░░███░░░███░░░███░░░███░░░███░
████░░░░████░░░░████░░░░████░░░░████░░░░
█████░░░░░█████░░░░░█████░░░░░█████░░░░░
██████░░░░░░██████░░░░░░██████░░░░░░████
███████░░░░░░░███████░░░░░░░███████░░░░░
"),
    include_input!(22 10) => ("13220", "\
███░░█░░█░░██░░█░░█░█░░█░███░░████░█░░█░
█░░█░█░░█░█░░█░█░█░░█░░█░█░░█░█░░░░█░█░░
█░░█░█░░█░█░░█░██░░░████░███░░███░░██░░░
███░░█░░█░████░█░█░░█░░█░█░░█░█░░░░█░█░░
█░█░░█░░█░█░░█░█░█░░█░░█░█░░█░█░░░░█░█░░
█░░█░░██░░█░░█░█░░█░█░░█░███░░████░█░░█░
"), // RUAKHBEK
}
