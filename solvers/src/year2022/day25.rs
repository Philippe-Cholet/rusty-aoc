use common::prelude::*;
use crate::utils::OkIterator;

/// Full of Hot Air
pub fn solver(part: Part, input: &str) -> Result<String> {
    Ok(match part {
        Part1 => int_to_snafu(input.lines().map(snafu_to_int).ok_sum()?),
        Part2 => SUCCESS.to_owned(),
    })
}

fn snafu_to_int(snafu: &str) -> Result<i64> {
    snafu
        .chars()
        .map(|ch| {
            Ok(match ch {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => bail!("Wrong char for a SNAFU number: {}", ch),
            })
        })
        .ok_fold(0, |res, ch| res * 5 + ch)
}

fn int_to_snafu(mut n: i64) -> String {
    let mut snafu = String::new();
    while n != 0 {
        let rem = n.rem_euclid(5);
        n = n.div_euclid(5) + i64::from(rem > 2);
        snafu.insert(
            0,
            match rem {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '=', // true * 5 + -2 == 3
                4 => '-', // true * 5 + -1 == 4
                _ => unreachable!("0 <= rem < 5"),
            },
        );
    }
    snafu
}

const SUCCESS: &str = "It's time for the reindeer to get its star-smoothie.";

test_solver! {
    "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
" => "2=-1=0",
    include_input!(22 25) => "20=02=120-=-2110-0=1",
}
