use anyhow::{bail, Error, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
enum PacketData {
    Integer(u64),
    List(Vec<PacketData>),
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Integer(_), Self::List(_)) => self.to_list().cmp(other),
            (Self::List(_), Self::Integer(_)) => self.cmp(&other.to_list()),
            (Self::Integer(a), Self::Integer(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


impl From<u64> for PacketData {
    fn from(value: u64) -> Self {
        Self::Integer(value)
    }
}

impl PacketData {
    fn to_list(&self) -> Self {
        match self {
            Self::Integer(x) => Self::List(vec![Self::Integer(*x)]),
            Self::List(_) => self.clone(),
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize, PartialOrd, Ord)]
struct Packet(Vec<PacketData>);

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct PacketPair {
    left: Packet,
    right: Packet,
}

impl TryFrom<&[String]> for PacketPair {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        if lines.len() != 2 {
            bail!("Packet pair input should be exactly 2 lines");
        }

        let left = serde_json::from_str(&lines[0])?;
        let right = serde_json::from_str(&lines[1])?;
        Ok(Self { left, right })
    }
}

pub fn parse_input(lines: &[String]) -> Result<Vec<PacketPair>> {
    lines
        .split(|l| l.is_empty())
        .map(|chunk| chunk.try_into())
        .collect()
}

pub fn part_one(parsed: &Vec<PacketPair>) -> usize {
    parsed.iter()
        .enumerate()
        .filter(|(_, pair)| pair.left < pair.right)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part_two(parsed: &Vec<PacketPair>) -> usize {
    let div1 = Packet(vec![PacketData::List(vec![PacketData::Integer(2)])]);
    let div2 = Packet(vec![PacketData::List(vec![PacketData::Integer(6)])]);
    let mut div1_index = 1;
    let mut div2_index = 2;

    for pair in parsed {
        if pair.left < div1 {
            div1_index += 1;
        }

        if pair.right < div1 {
            div1_index += 1;
        }

        if pair.left < div2 {
            div2_index += 1;
        }

        if pair.right < div2 {
            div2_index += 1;
        }
    }

    div1_index * div2_index
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn packet_data_ord_test() {
        assert!(PacketData::Integer(1) == PacketData::Integer(1));
        assert!(PacketData::Integer(1) < PacketData::Integer(2));
        assert!(PacketData::Integer(1) > PacketData::Integer(0));
        assert!(
            PacketData::List(vec![1.into(), 2.into()])
                == PacketData::List(vec![1.into(), 2.into()])
        );
        assert!(
            PacketData::List(vec![1.into(), 2.into()]) < PacketData::List(vec![1.into(), 3.into()])
        );
        assert!(
            PacketData::List(vec![1.into(), 2.into()]) > PacketData::List(vec![1.into(), 0.into()])
        );
        assert!(
            PacketData::List(vec![0.into(), 2.into()]) < PacketData::List(vec![1.into(), 0.into()])
        );
        assert!(PacketData::List(vec![2.into()]) > PacketData::List(vec![1.into(), 0.into()]));
        assert!(PacketData::List(vec![1.into()]) < PacketData::List(vec![1.into(), 0.into()]));
        assert!(PacketData::List(vec![1.into(), 2.into()]) > PacketData::List(vec![1.into()]));
        assert!(PacketData::List(vec![1.into(), 2.into()]) < PacketData::List(vec![2.into()]));
        assert!(PacketData::Integer(1) < PacketData::List(vec![1.into(), 2.into()]));
        assert!(PacketData::Integer(2) > PacketData::List(vec![1.into(), 2.into()]));
        assert!(PacketData::List(vec![1.into(), 2.into()]) < PacketData::Integer(2));
        assert!(PacketData::List(vec![1.into(), 2.into()]) > PacketData::Integer(0));
    }

    #[test]
    fn packet_ord_test() {
        assert!(
            serde_json::from_str::<Packet>("[1,1,3,1,1]").unwrap()
                < serde_json::from_str::<Packet>("[1,1,5,1,1]").unwrap()
        );
        assert!(
            serde_json::from_str::<Packet>("[[1],[2,3,4]]").unwrap()
                < serde_json::from_str::<Packet>("[[1],4]").unwrap()
        );
        assert!(
            serde_json::from_str::<Packet>("[9]").unwrap()
                > serde_json::from_str::<Packet>("[[8,7,6]]").unwrap()
        );
        assert!(
            serde_json::from_str::<Packet>("[[4,4],4,4]").unwrap()
                < serde_json::from_str::<Packet>("[[4,4],4,4,4]").unwrap()
        );
        assert!(
            serde_json::from_str::<Packet>("[7,7,7,7]").unwrap()
                > serde_json::from_str::<Packet>("[7,7,7]").unwrap()
        );
        assert!(
            serde_json::from_str::<Packet>("[]").unwrap()
                < serde_json::from_str::<Packet>("[3]").unwrap()
        );
        assert!(
            serde_json::from_str::<Packet>("[[[]]]").unwrap()
                > serde_json::from_str::<Packet>("[[]]").unwrap()
        );
        assert!(
            serde_json::from_str::<Packet>("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap()
                > serde_json::from_str::<Packet>("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap()
        );
    }

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_13.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 13);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_13.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 140);
    }
}
