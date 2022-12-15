use anyhow::{anyhow, Error, Result};
use rustc_hash::FxHashMap;
use std::str::FromStr;

use crate::grid::Coordinate;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Sensor {
    position: Coordinate,
    beacon: Coordinate,
    radius: usize,
}

impl Sensor {
    /// Returns the intersection of 2 lines
    ///
    /// Each line is defined by 2 unique points on the line. If the lines are parallel, then `None`
    /// is returned.
    fn intersection(
        a: (Coordinate, Coordinate),
        b: (Coordinate, Coordinate),
    ) -> Option<Coordinate> {
        // I'm too lazy to do math, so just use the formula here:
        // https://en.wikipedia.org/wiki/Line-line_intersection
        let (p1, p2) = a;
        let (p3, p4) = b;

        // We might run into overflow, so just cast to i128. I know it's not the best.
        let (x1, y1) = (p1.col() as i128, p1.row() as i128);
        let (x2, y2) = (p2.col() as i128, p2.row() as i128);
        let (x3, y3) = (p3.col() as i128, p3.row() as i128);
        let (x4, y4) = (p4.col() as i128, p4.row() as i128);

        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denom == 0 {
            return None;
        }

        let x_nume = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let y_nume = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);
        Some(((y_nume / denom) as isize, (x_nume / denom) as isize).into())
    }

    fn scanned_interval(&self, row: isize) -> Option<(isize, isize)> {
        let diff = (self.position.row() - row).abs();
        let left = self.position.col() - self.radius as isize + diff;
        let right = self.position.col() + self.radius as isize - diff;

        if left >= right {
            return None;
        }

        Some((left, right))
    }

    fn perimeter(&self) -> [(Coordinate, Coordinate); 4] {
        let row = self.position.row();
        let col = self.position.col();
        let radius = self.radius as isize;
        let top = (row - radius, col).into();
        let bottom = (row + radius, col).into();
        let left = (row, col - radius).into();
        let right = (row, col + radius).into();

        [(top, right), (right, bottom), (bottom, left), (left, top)]
    }

    fn offset_intersections(&self, other: &Self) -> [Option<Coordinate>; 8] {
        let my_perimeter = self.perimeter();
        let their_perimeter = other.perimeter();

        [
            Sensor::intersection(my_perimeter[0], their_perimeter[1]).map(|c| c.east()),
            Sensor::intersection(my_perimeter[0], their_perimeter[3]).map(|c| c.north()),
            Sensor::intersection(my_perimeter[1], their_perimeter[0]).map(|c| c.east()),
            Sensor::intersection(my_perimeter[1], their_perimeter[2]).map(|c| c.south()),
            Sensor::intersection(my_perimeter[2], their_perimeter[1]).map(|c| c.south()),
            Sensor::intersection(my_perimeter[2], their_perimeter[3]).map(|c| c.west()),
            Sensor::intersection(my_perimeter[3], their_perimeter[0]).map(|c| c.north()),
            Sensor::intersection(my_perimeter[3], their_perimeter[2]).map(|c| c.west()),
        ]
    }
}

impl FromStr for Sensor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_whitespace().collect();
        let t1 = tokens.get(2).ok_or_else(|| anyhow!("Invalid input"))?;
        let sensor_x: isize = t1[2..t1.len() - 1].parse()?;
        let t2 = tokens.get(3).ok_or_else(|| anyhow!("Invalid input"))?;
        let sensor_y: isize = t2[2..t2.len() - 1].parse()?;
        let t3 = tokens.get(8).ok_or_else(|| anyhow!("Invalid input"))?;
        let beacon_x: isize = t3[2..t3.len() - 1].parse()?;
        let t4 = tokens.get(9).ok_or_else(|| anyhow!("Invalid input"))?;
        let beacon_y: isize = t4[2..t4.len()].parse()?;

        let position = Coordinate::from((sensor_y, sensor_x));
        let beacon = Coordinate::from((beacon_y, beacon_x));
        let radius = (beacon_x - sensor_x).abs() + (beacon_y - sensor_y).abs();
        Ok(Sensor {
            position,
            beacon,
            radius: radius as usize,
        })
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Sensors {
    sensors: Vec<Sensor>,
    beacons: FxHashMap<isize, Vec<isize>>,
}

impl Sensors {
    fn disjoint_intervals(&self, row: isize) -> Vec<(isize, isize)> {
        let mut disjoint = Vec::default();

        let mut sorted_intervals: Vec<_> = self
            .sensors
            .iter()
            .filter_map(|s| s.scanned_interval(row))
            .collect();
        sorted_intervals.sort_unstable();

        if sorted_intervals.is_empty() {
            return disjoint;
        }

        let mut cur_s = sorted_intervals[0].0;
        let mut cur_e = sorted_intervals[0].1;

        for i in 1..sorted_intervals.len() {
            let (s, e) = sorted_intervals[i];

            // interval lies completely inside the ongoing interval, so we do nothing
            if e <= cur_e {
                continue;
            }

            // start of interval is inside the ongoing interval, so we just extend it
            if s <= cur_e {
                cur_e = e;
                continue;
            }

            // interval lies completely outside the ongoing interval, so we start a new one
            disjoint.push((cur_s, cur_e));
            cur_s = s;
            cur_e = e;
        }

        // push the last ongoing interval
        disjoint.push((cur_s, cur_e));
        disjoint
    }

    fn cleared_positions(&self, row: isize) -> usize {
        let disjoint_intervals = self.disjoint_intervals(row);

        if disjoint_intervals.is_empty() {
            return 0;
        }

        let maybe_beacon_cols = self.beacons.get(&row);
        if maybe_beacon_cols.is_none() || maybe_beacon_cols.unwrap().is_empty() {
            return disjoint_intervals
                .iter()
                .map(|(s, e)| *e - *s + 1)
                .sum::<isize>() as usize;
        }

        // safe to unwrap because we checked for None above
        let beacon_cols = maybe_beacon_cols.unwrap();
        let mut i = 0;
        let mut sum = 0;
        for (s, e) in disjoint_intervals {
            sum += e - s + 1;

            // if beacon is to be left of start of interval, it would have been processed already,
            // so we just ignore
            while i < beacon_cols.len() && beacon_cols[i] < s {
                i += 1;
            }

            // next, for every beacon inside the interval, subtract 1 from the sum
            while i < beacon_cols.len() && beacon_cols[i] <= e {
                sum -= 1;
                i += 1;
            }
        }

        sum as usize
    }

    fn find_distress_signal(&self, upper_bound: isize) -> Option<Coordinate> {
        for i in 0..self.sensors.len() {
            for j in i + 1..self.sensors.len() {
                for candidate in self.sensors[i].offset_intersections(&self.sensors[j]) {
                    if candidate.is_none() {
                        continue;
                    }

                    // safe to unwrap because we just checked
                    let coord = candidate.unwrap();

                    // we don't need to look at out of bounds points
                    if coord.row() < 0
                        || coord.row() > upper_bound
                        || coord.col() < 0
                        || coord.col() > upper_bound
                    {
                        continue;
                    }

                    // if the point is outside of the range of all the sensors, then we found it
                    if self
                        .sensors
                        .iter()
                        .all(|s| s.position.manhattan_distance(&coord) > s.radius)
                    {
                        return Some(coord);
                    }
                }
            }
        }

        None
    }

    fn find_tuning_frequency(&self, upper_bound: isize) -> usize {
        match self.find_distress_signal(upper_bound) {
            Some(coord) => (coord.col() * 4000000 + coord.row()) as usize,
            None => unreachable!(), // problem description says there is exactly 1 possible position
        }
    }
}

impl TryFrom<&[String]> for Sensors {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let sensors = lines
            .iter()
            .map(|l| Sensor::from_str(l))
            .collect::<Result<Vec<_>>>()?;

        let mut beacons: Vec<_> = sensors.iter().map(|s| s.beacon).collect();
        beacons.sort_unstable_by_key(|c| c.col());

        let mut beacons_map: FxHashMap<isize, Vec<isize>> = FxHashMap::default();
        for beacon in beacons {
            beacons_map
                .entry(beacon.row())
                .and_modify(|v| {
                    if let Some(col) = v.last() {
                        if *col != beacon.col() {
                            v.push(beacon.col());
                        }
                    }
                })
                .or_insert(vec![beacon.col()]);
        }

        Ok(Sensors {
            sensors,
            beacons: beacons_map,
        })
    }
}

pub fn parse_input(lines: &[String]) -> Result<Sensors> {
    Sensors::try_from(lines)
}

pub fn part_one(parsed: &Sensors) -> usize {
    parsed.cleared_positions(2000000)
}

pub fn part_two(parsed: &Sensors) -> usize {
    parsed.find_tuning_frequency(4000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_15.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");

        // example is different from actual answer, so just call the underlying method directly
        assert_eq!(parsed.cleared_positions(10), 26);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_15.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");

        // example is different from actual answer, so just call the underlying method directly
        assert_eq!(parsed.find_tuning_frequency(20), 56000011);
    }
}
