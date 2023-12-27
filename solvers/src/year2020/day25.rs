use common::prelude::*;
use crate::utils::OkIterator;

#[allow(clippy::inconsistent_digit_grouping)]
const MOD: u64 = 2020_12_27; // a prime number

/// Combo Breaker
pub fn solver(part: Part, input: &str) -> Result<String> {
    match part {
        Part1 => {
            let [card_public_key, door_public_key] =
                input.lines().map(str::parse).ok_collect_array()?;
            let mut v = 1;
            for card_loop_size in 1..MOD {
                v = (v * 7) % MOD;
                if v == card_public_key {
                    #[cfg(debug_assertions)]
                    println!("Card's secret loop size: {card_loop_size:?}");
                    // v = 1;
                    // for _ in 0..card_loop_size {
                    //     v = (v * door_public_key) % MOD;
                    // }
                    // The code above works just fine but is "slow". While this
                    // below only takes a small number of steps: the number of
                    // bits of the exponent `card_loop_size`, which is max 25
                    // because it's inferior to `MOD` which has 25 bits.
                    // So the only slow thing is to first find `card_loop_size`.
                    v = checked_mod_pow(door_public_key, card_loop_size, MOD)
                        .context("might overflow: choose a type with more bits")?;
                    return Ok(v.to_string());
                }
            }
            bail!("Did not find the encryption key");
        }
        Part2 => Ok(SUCCESS.to_owned()),
    }
}

/// `n.pow(exp) % modulus`
fn checked_mod_pow(mut n: u64, mut exp: u64, modulus: u64) -> Option<u64> {
    if modulus == 1 {
        return Some(0);
    }
    modulus.checked_sub(1)?.checked_pow(2)?;
    let mut res = 1;
    n %= modulus;
    while exp != 0 {
        if exp % 2 == 1 {
            res = res * n % modulus;
        }
        exp >>= 1;
        n = n * n % modulus;
    }
    Some(res)
}

pub const INPUTS: [&str; 2] = ["5764801\n17807724\n", include_input!(20 25)];

const SUCCESS: &str = "Time to pay the resort and take a ride in Santa' sleigh.";

#[test]
fn solver_20_25() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "14897079");
    assert_eq!(solver(Part1, INPUTS[1])?, "12285001");
    assert_eq!(solver(Part2, INPUTS[0])?, SUCCESS);
    assert_eq!(solver(Part2, INPUTS[1])?, SUCCESS);
    Ok(())
}
