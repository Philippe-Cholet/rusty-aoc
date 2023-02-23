use common::{Context, Part, Part1, Part2, Result};

const fn sum_divisors(n: u32, upper_bound: u32) -> u32 {
    let (mut sum, mut d) = (0, 1);
    let mut k;
    // `d * d <= n` so `d <= n / d == k`. Therefore, if `d > upper_bound`
    // then `k > upper_bound` too, in which case we can break the loop.
    while d * d <= n && d <= upper_bound {
        if n % d == 0 {
            k = n / d;
            if k <= upper_bound {
                sum += d;
            }
            if k != d {
                sum += k;
            }
        }
        d += 1;
    }
    sum
}

/// Infinite Elves and Infinite Houses
pub fn solver(part: Part, input: &str) -> Result<String> {
    let n: u32 = input.trim_end().parse()?;
    // Commented out code works just fine but is a bit slow.
    // Ok((0..=u32::MAX)
    //     .find(|&k| match part {
    //         Part1 => 10 * sum_divisors(k, k),
    //         Part2 => 11 * sum_divisors(k, 50),
    //     } >= n)
    //     .context("No u32 solution")?
    // .to_string())
    let (divisor_upper_bound, sum_lower_bound) = match part {
        Part1 => (None, n / 10),
        Part2 => (Some(50), n / 11),
    };
    // Compute `solution_lower_bound` is fast and it cuts off
    // the section `0..solution_lower_bound` of the search below.
    let solution_lower_bound = find_robin_lower_bound(sum_lower_bound);
    #[cfg(debug_assertions)]
    println!("solution's lower bound = {solution_lower_bound:?}");
    // Part 1: 736_811 (for a solution below of 831_600).
    // Part 2: 671_426 (for a solution below of 884_520).
    // So respectively 88% and 75% of the search is cut off.
    Ok((solution_lower_bound..=u32::MAX)
        .find(|&k| sum_divisors(k, divisor_upper_bound.unwrap_or(k)) >= sum_lower_bound)
        .context("No u32 solution")?
        .to_string())
}

/// See [wikipedia](https://en.wikipedia.org/wiki/Euler's_constant).
const EULER_MASCHERONI: f64 = 0.577_215_664_901_532_9;

/// The "sum of divisors of n" is less than `robin(n)`.
/// See [wikipedia](https://en.wikipedia.org/wiki/Divisor_function#Growth_rate).
///
/// An upper-bound on divisors only make the sum smaller so we have:
/// `sum_divisors(n, 50) <= sum_divisors(n, n) <= robin(n)`.
///
/// So to have `x <= sum_divisors(n, ...)`, we necessarily must have `x <= robin(n)`.
fn robin(n: u32) -> f64 {
    let x = f64::from(n);
    let llx = x.ln().ln();
    (EULER_MASCHERONI.exp() * x).mul_add(llx, 0.6483 * x / llx)
}

/// Find the smallest integer `n` such as `x <= robin(n)`.
///
/// Because it is a monotonically increasing function for n >= 4, we can use a binary search.
fn find_robin_lower_bound(x: u32) -> u32 {
    let x = f64::from(x);
    let (mut mini, mut maxi) = (4u32, u32::MAX);
    let mut mid;
    while maxi - mini > 1 {
        debug_assert!(robin(mini) < x && x <= robin(maxi));
        mid = mini + (maxi - mini) / 2;
        if robin(mid) < x {
            mini = mid;
        } else {
            maxi = mid;
        }
    }
    maxi
}

pub const INPUTS: [&str; 1] = [include_str!("input.txt")];

#[test]
fn solver_15_20() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "831600");
    assert_eq!(solver(Part2, INPUTS[0])?, "884520");
    Ok(())
}
