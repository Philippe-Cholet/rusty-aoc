use itertools::Itertools;

use common::{prelude::*, Ok};
use crate::utils::{char16, OkIterator};

#[derive(Debug)]
enum Packet {
    LiteralValue {
        version: usize,
        value: usize,
    },
    Operation {
        version: usize,
        type_id: usize,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    fn version_sum(&self) -> usize {
        match self {
            Self::LiteralValue { version, .. } => *version,
            Self::Operation {
                version,
                sub_packets,
                ..
            } => *version + sub_packets.iter().map(Self::version_sum).sum::<usize>(),
        }
    }

    #[allow(clippy::expect_used)]
    fn value(&self) -> Result<usize> {
        Ok(match self {
            Self::LiteralValue { value, .. } => *value,
            Self::Operation {
                type_id,
                sub_packets,
                ..
            } => {
                let values: Vec<_> = sub_packets.iter().map(Self::value).try_collect()?;
                match (type_id, values.len()) {
                    (0, _) => values.into_iter().sum(),
                    (1, _) => values.into_iter().product(),
                    (2, 1..) => values
                        .into_iter()
                        .min()
                        .expect("A non empty iterable has a min"),
                    (3, 1..) => values
                        .into_iter()
                        .max()
                        .expect("A non empty iterable has a max"),
                    (5, 2) => (values[0] > values[1]).into(),
                    (6, 2) => (values[0] < values[1]).into(),
                    (7, 2) => (values[0] == values[1]).into(),
                    (op, n) => bail!("The operator {} does not accept {} arguments!", op, n),
                }
            }
        })
    }
}

#[derive(Debug)]
struct Stream(Vec<char>);

impl Stream {
    fn read(&mut self, size: usize) -> Result<String> {
        (0..size)
            .map(|_| self.0.pop().context("too soon to be empty"))
            .try_collect()
    }

    fn read_packet(&mut self) -> Result<Packet> {
        let s2n = |s: &str| usize::from_str_radix(s, 2);
        let (version, type_id) = (s2n(&self.read(3)?)?, s2n(&self.read(3)?)?);
        if type_id == 4 {
            let mut value_bits = String::new();
            let value = loop {
                let chunk = self.read(5)?;
                let (start, four_bits) = chunk.split_at(1);
                value_bits.push_str(four_bits);
                if start == "0" {
                    break s2n(&value_bits)?;
                }
            };
            Ok(Packet::LiteralValue { version, value })
        } else {
            let mut sub_packets = vec![];
            if s2n(&self.read(1)?)? == 0 {
                let total_bits_length = s2n(&self.read(15)?)?;
                let to_read = self.0.len() - total_bits_length;
                while self.0.len() != to_read {
                    sub_packets.push(self.read_packet()?);
                }
            } else {
                for _ in 0..s2n(&self.read(11)?)? {
                    sub_packets.push(self.read_packet()?);
                }
            };
            Ok(Packet::Operation {
                version,
                type_id,
                sub_packets,
            })
        }
    }
}

/// Packet Decoder
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let bin_line = input
        .trim_end()
        .chars()
        .map(|ch| Ok(format!("{:04b}", char16::<u32>(ch)?)))
        .ok_collect_str()?;
    let main_packet = Stream(bin_line.chars().rev().collect()).read_packet()?;
    // println!("{:#?}", main_packet);
    match part {
        Part1 => Ok(main_packet.version_sum()),
        Part2 => main_packet.value(),
    }
}

test_solver! {
    "8A004A801A8002F478" => 16,
    "620080001611562C8802118E34" => 12,
    "C0015000016115A2E0802F182340" => 23,
    "A0016C880162017C3686B18A3D4780" => 31,
    "C200B40A82" => ((), 3),                 // 1 + 2 => 3
    "04005AC33890" => ((), 54),              // 6 * 9 => 54
    "880086C3E88112" => ((), 7),             // min(7, 8, 9) => 7
    "CE00C43D881120" => ((), 9),             // max(7, 8, 9) => 9
    "D8005AC2A8F0" => ((), 1),               // "5 < 15" is true => 1
    "F600BC2D8F" => ((), 0),                 // "5 > 15" is false => 0
    "9C005AC2F8F0" => ((), 0),               // "5 is equal to 15" is false => 0
    "9C0141080250320F1802104A08" => ((), 1), // "1 + 3 = 2 * 2" is true => 1
    include_input!(21 16) => (943, 167737115857),
}
