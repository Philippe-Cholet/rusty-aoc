use common::prelude::*;
use utils::OkIterator;

#[derive(Debug)]
struct SensorData {
    sensor: (i64, i64),
    beacon: (i64, i64),
}

fn split_after_prefix<'a>(s: &'a str, prefix: &str, sep: &str) -> Result<(&'a str, &'a str)> {
    s.strip_prefix(prefix)
        .context("wrong prefix")?
        .split_once(sep)
        .context("wrong separator")
}

impl std::str::FromStr for SensorData {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let (sensor, beacon) = split_after_prefix(line, "Sensor at ", ": closest beacon is at ")?;
        let (xs, ys) = split_after_prefix(sensor, "x=", ", y=")?;
        let (xb, yb) = split_after_prefix(beacon, "x=", ", y=")?;
        Ok(Self {
            sensor: (xs.parse()?, ys.parse()?),
            beacon: (xb.parse()?, yb.parse()?),
        })
    }
}

impl SensorData {
    const fn manhattan(&self) -> i64 {
        (self.sensor.0 - self.beacon.0).abs() + (self.sensor.1 - self.beacon.1).abs()
    }

    //     .
    //    . .
    //   .   .
    //  X-----X   <-- y, "X------X is the "x interval"
    // .   S   .
    //  .     .
    //   .   B
    //    . .
    //     .
    fn x_interval(&self, y: i64) -> Option<(i64, i64)> {
        // Remaining distance left for the x-axis:
        let xd = self.manhattan() - (self.sensor.1 - y).abs();
        (xd >= 0).then_some((self.sensor.0 - xd, self.sensor.0 + xd))
    }

    fn x_intervals(datas: &[Self], y: i64, maxi: Option<i64>) -> Vec<(i64, i64)> {
        let mut intervals = Vec::with_capacity(datas.len());
        intervals.extend(datas.iter().filter_map(|data| {
            let x_int = data.x_interval(y)?;
            match maxi {
                // No restriction to "0..=maxi".
                None => Some(x_int),
                // Outside "0..=maxi".
                Some(m) if x_int.1 < 0 || x_int.0 > m => None,
                // Restrain it to "0..=maxi".
                Some(m) => Some((x_int.0.max(0), x_int.1.min(m))),
            }
        }));
        intervals.sort_unstable();
        let mut cur_x = i64::MIN;
        intervals.retain(|x_int| {
            if x_int.1 <= cur_x {
                // This interval is contained by the previous one.
                false
            } else {
                cur_x = x_int.1;
                true
            }
        });
        intervals
    }
}

/// Beacon Exclusion Zone
pub fn solver(part: Part, input: &str) -> Result<String> {
    let datas: Vec<SensorData> = input.lines().map(str::parse).ok_collect()?;
    // NOTE: tricky, but since some parameters are not provided in inputs...
    let small = datas
        .iter()
        .flat_map(|data| [data.sensor.0, data.sensor.1, data.beacon.0, data.beacon.1])
        .all(|x| x < 1000);
    Ok(match part {
        Part1 => {
            let y = if small { 10 } else { 2_000_000 };
            SensorData::x_intervals(&datas, y, None)
                .into_iter()
                .fold((0, i64::MIN), |(total, cur_x), (x0, x1)| {
                    let interval_length = (x0.max(cur_x) - x1).abs();
                    (total + interval_length, x1)
                })
                .0
        }
        Part2 => {
            let maxi = if small { 20 } else { 4_000_000 };
            (0..=maxi)
                .rev() // faster than without, but that's just fortunate!
                .find_map(|y| {
                    let mut cur_x = i64::MIN;
                    let x = SensorData::x_intervals(&datas, y, Some(maxi))
                        .into_iter()
                        .find_map(|(x0, x1)| {
                            if cur_x + 1 < x0 && cur_x != i64::MIN {
                                // `cur_x + 1` is in the hole and we were told it is the only one.
                                Some(cur_x + 1)
                            } else {
                                cur_x = x1;
                                None
                            }
                        })?;
                    #[cfg(debug_assertions)]
                    println!("{:?}", (x, y));
                    // NOTE: The type i64 is required for this multiplication:
                    Some(4_000_000 * x + y)
                })
                .context("no solution")?
        }
    }
    .to_string())
}

pub const INPUTS: [&str; 2] = [
    "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_15() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "26");
    assert_eq!(solver(Part1, INPUTS[1])?, "5083287");
    assert_eq!(solver(Part2, INPUTS[0])?, "56000011");
    assert_eq!(solver(Part2, INPUTS[1])?, "13134039205729");
    Ok(())
}
