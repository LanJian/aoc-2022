use anyhow::{anyhow, Error, Result};
use rustc_hash::{FxHashMap, FxHashSet};
use std::str::FromStr;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Valve {
    id: String,
    rate: usize,
    connections: Vec<String>,
}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_whitespace().collect();
        let id = tokens.get(1).ok_or_else(|| anyhow!("Invalid input"))?;
        let token = tokens.get(4).ok_or_else(|| anyhow!("Invalid input"))?;
        let rate: usize = token[5..token.len() - 1].parse()?;
        let connections: Vec<String> = tokens[9..].iter().map(|t| t[0..2].to_owned()).collect();

        Ok(Valve {
            id: (*id).to_owned(),
            rate,
            connections,
        })
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Memo {
    opened: FxHashMap<String, [usize; 31]>,
    unopened: FxHashMap<String, [usize; 31]>,
}

impl Memo {
    fn new(keys: &Vec<&String>) -> Self {
        let opened = FxHashMap::from_iter(keys.iter().map(|&k| (k.to_owned(), [0; 31])));
        let unopened = opened.clone();

        Self {
            opened,
            unopened
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Cave {
    valves: FxHashMap<String, Valve>,
}

impl Cave {
    fn foo(&self) -> usize {
        let keys: Vec<&String> = self.valves.keys().collect();
        let mut dp: FxHashMap<String, Memo> =
            FxHashMap::from_iter(keys.iter().map(|&k| (k.to_owned(), Memo::new(&keys))));

        for i in 1..=30 {
            for k in self.valves.keys() {
                let valve = &self.valves[k];

                // calculate the max if the valve has been opened
                {
                    let memo = &dp[k];
                    let mut max = 0;

                    // option 1: open the valve right now
                    max = max.max(memo.unopened[k][i-1] + valve.rate * (i - 1));

                    // option 2: don't open the valve right now, get the max neighbour where the
                    // valve has been opened
                    for con in &valve.connections {
                        max = max.max(memo.opened[con][i-1]);
                    }

                    for (a, b) in dp.iter_mut() {
                        let cur_opened = b.opened[k][i];
                        let cur_unopened = b.unopened[k][i];
                        if max > cur_opened {
                            b.opened.get_mut(k).unwrap()[i] = max;
                        }
                        if max > cur_unopened && a != k {
                            b.unopened.get_mut(k).unwrap()[i] = max;
                        }
                    }
                }

                // calculate the max if the valve has not been opened
                {
                    let memo = &dp[k];
                    let mut max = 0;

                    // get the max neighbour where the valve has not been opened
                    for con in &valve.connections {
                        max = max.max(memo.unopened[con][i-1]);
                    }

                    //dp.get_mut(k).unwrap().unopened.get_mut(k).unwrap()[i] = max;
                    for (a, b) in dp.iter_mut() {
                        let cur_opened = b.opened[k][i];
                        let cur_unopened = b.unopened[k][i];
                        if max > cur_unopened {
                            b.unopened.get_mut(k).unwrap()[i] = max;
                        }
                        if max > cur_opened && a != k {
                            b.opened.get_mut(k).unwrap()[i] = max;
                        }
                    }
                }


                //// if valve is open, we have to go somewhere else
                //for con in &valve.connections {
                    //let neighbour = &dp[con];
                    //if k == "AA" && i == 2 {
                        //dbg!(&con);
                        //dbg!(&neighbour.open[i - 1]);
                        //dbg!(&neighbour.closed[i - 1]);
                    //}
                    //if neighbour.open[i - 1].0 + neighbour.open[i - 1].1 > open_max {
                        //open_max = neighbour.open[i - 1].0 + neighbour.open[i - 1].1;
                        //open_rate = neighbour.open[i - 1].1;
                    //}
                    //if neighbour.closed[i - 1].0 + neighbour.closed[i - 1].1 > open_max {
                        //open_max = neighbour.closed[i - 1].0 + neighbour.closed[i - 1].1;
                        //open_rate = neighbour.closed[i - 1].1;
                    //}
                //}

                //// if valve is closed, we can either stay and open it, or go somewhere else
                //// option 1: open the valve
                //if memo.open[i - 1].0 + memo.open[i - 1].1 > closed_max {
                    //closed_max = memo.open[i - 1].0 + memo.open[i - 1].1;
                    //closed_rate = memo.open[i - 1].1;
                //}

                //// option 2: don't open the valve, go somewhere else
                //for con in &valve.connections {
                    //let neighbour = &dp[con];
                    //if neighbour.open[i - 1].0 + neighbour.open[i - 1].1 > closed_max {
                        //closed_max = neighbour.open[i - 1].0 + neighbour.open[i - 1].1;
                        //closed_rate = neighbour.open[i - 1].1;
                    //}
                    //if neighbour.closed[i - 1].0 + neighbour.closed[i - 1].1 > closed_max {
                        //closed_max = neighbour.closed[i - 1].0 + neighbour.closed[i - 1].1;
                        //closed_rate = neighbour.closed[i - 1].1;
                    //}
                //}

                //// FIXME jackhxs 2022-12-16: msg
                //dp.get_mut(k).unwrap().open[i].0 = open_max;
                //dp.get_mut(k).unwrap().open[i].1 = open_rate;
                //dp.get_mut(k).unwrap().closed[i].0 = closed_max;
                //dp.get_mut(k).unwrap().closed[i].1 = closed_rate;
            }
        }

        //dbg!(&dp["AA"]);
        //dbg!(&dp["DD"]);
        //dbg!(&dp["II"]);
        //dbg!(&dp["BB"]);
        dbg!(&dp["AA"]);
        dp["AA"].unopened["AA"][30]

        //todo!()
    }
    fn dfs(&self, id: &str, remaining: usize, acc: usize, opened: &mut FxHashSet<String>) -> usize {
        //dbg!(&remaining);
        if remaining <= 10 {
            return acc;
        }

        let valve = &self.valves[id];

        let mut ret = acc;

        // open the valve

        opened.insert(id.to_owned());
        let result = self.dfs(
            id,
            remaining - 1,
            acc + (valve.rate * (remaining - 1)),
            opened,
        );
        opened.remove(id);
        if result > ret {
            ret = result;
        }

        // don't open the valve
        for con in &valve.connections {
            let result = self.dfs(con, remaining - 1, acc, opened);
            if result > ret {
                ret = result;
            }
        }

        ret
    }
}

impl TryFrom<&[String]> for Cave {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let mut valves = FxHashMap::default();

        for result in lines.iter().map(|l| Valve::from_str(l)) {
            let valve = result?;
            valves.insert(valve.id.clone(), valve);
        }

        Ok(Cave { valves })
    }
}

pub fn parse_input(lines: &[String]) -> Result<Cave> {
    Cave::try_from(lines)
}

pub fn part_one(parsed: &Cave) -> usize {
    //dbg!(&parsed);
    //parsed.dfs("AA", 30, 0, &mut FxHashSet::default())
    parsed.foo()
}

pub fn part_two(parsed: &Cave) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_16.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 1651);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_16.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 0);
    }
}
