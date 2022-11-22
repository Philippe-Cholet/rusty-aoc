use common::{Context, Part, Part1, Part2, Result};

/// Arithmetic Logic Unit
pub fn solver(part: Part, input: &str) -> Result<String> {
    parser::check_input(input)?; // old code that I don't want to remove.
    deductions::get_possibilities();
    // TODO: use the deduced result to define digit ranges and digits... I did it manually while I would prefer it automatic.
    // d4 == d3 - 8 && d6 == d5 - 3 && d7 == d2 && d8 == d1 - 7 && d10 == d9 + 5 && d13 == d12 - 6 && d14 == d11 + 3
    let ds = [8..=9, 1..=9, 9..=9, 4..=9, 1..=4, 1..=6, 7..=9];
    println!(
        "There are {} model numbers accepted by MONAD.",
        2*9 /* *1 */ *6*4*6*3, // == 7776 (product of the range lengths in `ds`).
    );
    let [d1, d2, d3, d5, d9, d11, d12]: [_; 7] = ds
        .into_iter()
        .map(|range| {
            match part {
                Part1 => range.max(),
                Part2 => range.min(),
            }
            .context("empty range")
        })
        .collect::<Result<Vec<_>>>()?
        .try_into()
        .ok()
        .context("Not 7 long")?;
    let [d4, d6, d7, d8, d10, d13, d14] = [d3 - 8, d5 - 3, d2, d1 - 7, d9 + 5, d12 - 6, d11 + 3];
    let model_digits = [d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13, d14];
    let model_number = model_digits
        .into_iter()
        .fold(0u64, |res, digit| res * 10 + digit);
    Ok(model_number.to_string())
}

pub const INPUTS: [&str; 1] = [include_str!("input.txt")];

#[test]
fn solver_21_24() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "99919692496939");
    assert_eq!(solver(Part2, INPUTS[0])?, "81914111161714");
    Ok(())
}

const PATTERN: [(bool, i32, i32); 14] = [
    (false, 10, 5),
    (false, 13, 9),
    (false, 12, 4),
    (true, -12, 4),
    (false, 11, 10),
    (true, -13, 14),
    (true, -9, 14),
    (true, -12, 12),
    (false, 14, 14),
    (true, -9, 14),
    (false, 15, 5),
    (false, 11, 10),
    (true, -16, 8),
    (true, -2, 15),
];

mod parser {
    use std::str::FromStr;

    use itertools::Itertools;

    use common::{bail, ensure, Error, Result};

    use super::{INPUTS, PATTERN};

    use self::{
        Instruction::{Add, Div, Eql, Inp, Mod, Mul},
        Value::{Number, Var},
        Variable::{W, X, Y, Z},
    };

    #[derive(Debug, PartialEq)]
    enum Variable {
        W,
        X,
        Y,
        Z,
    }

    #[derive(Debug)]
    enum Value {
        Var(Variable),
        Number(i32),
    }

    #[derive(Debug)]
    enum Instruction {
        Inp(Variable),
        Add(Variable, Value),
        Mul(Variable, Value),
        Div(Variable, Value),
        Mod(Variable, Value),
        Eql(Variable, Value),
    }

    impl FromStr for Variable {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            Ok(match s {
                "w" => Self::W,
                "x" => Self::X,
                "y" => Self::Y,
                "z" => Self::Z,
                v => bail!("Wrong variable: {}", v),
            })
        }
    }

    impl FromStr for Value {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            Ok(match s.parse::<Variable>() {
                Ok(var) => Self::Var(var),
                Err(e) => Self::Number(s.parse().or(Err(e))?),
            })
        }
    }

    impl FromStr for Instruction {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            let data: Vec<_> = s.split_whitespace().collect();
            Ok(match data[..] {
                ["inp", var] => Self::Inp(var.parse()?),
                ["add", var, value] => Self::Add(var.parse()?, value.parse()?),
                ["mul", var, value] => Self::Mul(var.parse()?, value.parse()?),
                ["div", var, value] => Self::Div(var.parse()?, value.parse()?),
                ["mod", var, value] => Self::Mod(var.parse()?, value.parse()?),
                ["eql", var, value] => Self::Eql(var.parse()?, value.parse()?),
                _ => bail!("Wrong instruction"),
            })
        }
    }

    pub fn check_input(input: &str) -> Result<()> {
        let instructions: Vec<Instruction> = input.lines().map(str::parse).try_collect()?;
        let data: Vec<_> = instructions.chunks(18)
            .map(|chunk| {
                match chunk {
                    [
                        Inp(v0),
                        Mul(v1, Number(0)),
                        Add(v2, Var(w0)),
                        Mod(v3, Number(26)),
                        Div(v4, Number(a)),
                        Add(v5, Number(b)),
                        Eql(v6, Var(w1)),
                        Eql(v7, Number(0)),
                        Mul(v8, Number(0)),
                        Add(v9, Number(25)),
                        Mul(v10, Var(w2)),
                        Add(v11, Number(1)),
                        Mul(v12, Var(w3)),
                        Mul(v13, Number(0)),
                        Add(v14, Var(w4)),
                        Add(v15, Number(c)),
                        Mul(v16, Var(w5)),
                        Add(v17, Var(w6)),
                    ]
                    if [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17]
                        == [&W, &X, &X, &X, &Z, &X, &X, &X, &Y, &Y, &Y, &Y, &Z, &Y, &Y, &Y, &Y, &Z]
                        && [w0, w1, w2, w3, w4, w5, w6] == [&Z, &W, &X, &Y, &W, &X, &Y]
                        && [1, 26].contains(a)
                        && (-16..=15).contains(b)
                        && (4..=15).contains(c)
                    => Ok((*a == 26, *b, *c)),
                    _ => bail!("Does not match the blocks I saw"),
                }
            })
            .try_collect()?;
        ensure!(
            data == PATTERN && input == INPUTS[0],
            "Only one MONAD in the world!"
        );
        Ok(())
    }
}

/*
After mindless backtracking method, I saw a pattern in the MONAD input:
[
    Inp(W),
    Mul(X, Number(0)),
    Add(X, Var(Z)),
    Mod(X, Number(26)),
    Div(Z, Number(a)),
    Add(X, Number(b)),
    Eql(X, Var(W)),
    Eql(X, Number(0)),
    Mul(Y, Number(0)),
    Add(Y, Number(25)),
    Mul(Y, Var(X)),
    Add(Y, Number(1)),
    Mul(Z, Var(Y)),
    Mul(Y, Number(0)),
    Add(Y, Var(W)),
    Add(Y, Number(c)),
    Mul(Y, Var(X)),
    Add(Z, Var(Y)),
]
// (a == 1 || a == 26) && -16 <= b && b <= 15 && 4 <= c <= 15

Executing such block of instructions would give us:
[d, x, y, z]
[d, 0, y, z]
[d, z, y, z]
[d, z%26, y, z]
[d, z%26, y, z/a]
[d, z%26+b, y, z/a]
[d, (z%26+b)==d, y, z/a]
[d, e, y, z/a] // let e = (((z%26+b)==d).into()==0).into();
[d, e, 0, z/a]
[d, e, 25, z/a]
[d, e, 25*e, z/a]
[d, e, 25*e+1, z/a]
[d, e, 25*e+1, z/a*(25*e+1)]
[d, e, 0, z/a*(25*e+1)]
[d, e, d, z/a*(25*e+1)]
[d, e, d+c, z/a*(25*e+1)]
[d, e, (d+c)*e, z/a*(25*e+1)]
[d, e, (d+c)*e, z/a*(25*e+1)+(d+c)*e]

x and y are entirely discared, only the z resulting matters.
This block tranforms `z` into `z/a*(25*e+1)+(d+c)*e` with `let e = (z % 26 + b != d).into();`
So `z = if z % 26 + b != d { z / a * 26 + d + c } else { z / a };`

I'm gonna represent `z` in base 26 with a vector `vec![a, b, c, d, e]` for `a + 26 * b + 26**2 * c + 26**3 * d + 26**4 * e`.
And it appears that every number considered for those vector is a number in `0..26` so everything is fine.
Therefore:
- `z % 26` is the first element of the vector (or 0).
- `z / 26` is the vector without its first element (or vec![]).
- `z * 26` is the vector with 0 inserted at index 0.
*/
mod deductions {
    use std::{fmt, ops::Add};

    use itertools::Itertools;

    use super::PATTERN;

    // Basic symbolic calculus with digits as symbols.
    // I first tried to do this by hand (with Sublime Text and multiple cursors which is amazingly useful)
    // but I made a mistake(s?) doing so. So I finally chose to do this in rust, which was a bit scary,
    // being used to "sympy" with python but it is not that complicated here so it's fine.

    // e.g. d1
    #[derive(Debug, Clone, Copy)]
    struct Digit(i32);

    // e.g. d1 - 5
    #[derive(Debug, Clone)]
    enum Addition {
        Value(i32),
        Stuff(Digit, i32),
    }

    // e.g. Equality(d1 + 6, d4, true) to represent that "d1 + 6 == d4" is true.
    #[derive(Debug, Clone)]
    struct Equality(Addition, Digit, bool);

    #[derive(Debug, Default, Clone)]
    struct Possibility {
        base26: Vec<Addition>, // vec![a, b, c] represents a + 26 * b + 26**2 * c, vec![] represents 0.
        conditions: Vec<Equality>, // list of conditions in order for this to be possible.
    }

    impl Add<i32> for Digit {
        type Output = Addition;

        fn add(self, k: i32) -> Addition {
            Addition::Stuff(self, k)
        }
    }

    impl Add<i32> for Addition {
        type Output = Self;

        fn add(self, k: i32) -> Self {
            match self {
                Self::Value(n) => Self::Value(n + k),
                Self::Stuff(digit, n) => Self::Stuff(digit, n + k),
            }
        }
    }

    impl Addition {
        const fn can_be_equal_digit(&self) -> bool {
            match self {
                Self::Value(n) => 1 <= *n && *n <= 9,
                // The difference between two digits is necessarily between -8 and 8.
                Self::Stuff(_, diff) => diff.abs() <= 8,
            }
        }
    }

    impl fmt::Display for Digit {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "d{}", self.0)
        }
    }

    impl fmt::Display for Addition {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Value(n) => write!(f, "{n}"),
                Self::Stuff(digit, n) if n < &0 => write!(f, "{digit} - {}", -n),
                Self::Stuff(digit, n) if n == &0 => write!(f, "{digit}"),
                Self::Stuff(digit, n) => write!(f, "{digit} + {n}"),
            }
        }
    }

    impl fmt::Display for Equality {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{} {} {}",
                self.1,
                if self.2 { "==" } else { "!=" },
                self.0,
            )
        }
    }

    impl fmt::Display for Possibility {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let base26 = self.base26.iter().map(|add| format!("{add}")).join(", ");
            let eql = self
                .conditions
                .iter()
                .map(|eql| format!("{eql}"))
                .join(" && ");
            write!(f, "[{base26}] // {eql}")
        }
    }

    impl Possibility {
        fn is_zero(&self) -> bool {
            self.base26
                .iter()
                .all(|add| matches!(add, Addition::Value(0)))
            // I ignore Addition::Stuff without being absolute sure I should.
        }
    }

    pub fn get_possibilities() {
        let mut possibilities = vec![Possibility::default()];
        let digits: Vec<_> = (1..=14).map(Digit).collect();
        for (idx, item) in PATTERN.into_iter().enumerate() {
            let digit = digits[idx];
            possibilities = possibilities
                .iter()
                .flat_map(|z| {
                    let mut z1 = z.clone();
                    let first = match z.base26.first() {
                        None => Addition::Value(0),
                        Some(add) => {
                            if item.0 {
                                // Divise by 26 means removing the first element.
                                z1.base26.remove(0);
                            }
                            add.clone()
                        }
                    };
                    let new_add = first + item.1;
                    if new_add.can_be_equal_digit() {
                        let mut z2 = z1.clone();
                        z2.conditions.push(Equality(new_add.clone(), digit, true));
                        z1.conditions.push(Equality(new_add, digit, false));
                        // Multiply by 26 and add a number in 0..26 means inserting as first element.
                        z1.base26.insert(0, digit + item.2);
                        vec![z1, z2]
                    } else {
                        z1.base26.insert(0, digit + item.2);
                        vec![z1]
                    }
                })
                .collect();
        }
        possibilities.retain(Possibility::is_zero);
        for possibility in possibilities {
            println!("{possibility}");
        }
    }
}
