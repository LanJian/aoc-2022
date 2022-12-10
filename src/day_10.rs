use anyhow::{anyhow, bail, Error, Result};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Self::Noop);
        }

        let (a, b) = s
            .split_once(" ")
            .ok_or_else(|| anyhow!("Could not parse instruction: {}", s))?;

        match a {
            "addx" => Ok(Self::Addx(b.parse()?)),
            _ => bail!("Could not parse instruction: {}", s),
        }
    }
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Cpu {
    signal_strength: isize,
    register: isize,
    cycle: usize,
    interesting_cycle_index: usize,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            signal_strength: 0,
            register: 1,
            cycle: 0,
            interesting_cycle_index: 0,
        }
    }
}

impl Cpu {
    const INTERESTING_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

    fn run(&mut self, instruction: &Instruction) {
        if let Some(&interesting_cycle) = Self::INTERESTING_CYCLES.get(self.interesting_cycle_index)
        {
            if self.cycle + instruction.cycles() >= interesting_cycle {
                self.signal_strength += interesting_cycle as isize * self.register;
                self.interesting_cycle_index += 1;
            }
        }

        self.cycle += instruction.cycles();
        match instruction {
            Instruction::Addx(value) => self.register += value,
            Instruction::Noop => {}
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Crt {
    output: Vec<char>,
    cur_row: usize,
}

impl Crt {
    const WIDTH: usize = 40;

    fn draw(&mut self, cpu: &Cpu, instruction: &Instruction) {
        for i in 0..instruction.cycles() {
            self.draw_one_cycle(cpu.cycle + i, cpu.register);
        }
    }

    fn draw_one_cycle(&mut self, cycle: usize, register: isize) {
        let col = (cycle % Self::WIDTH) as isize;
        if cycle / Self::WIDTH > self.cur_row {
            self.cur_row += 1;
            self.output.push('\n');
        }

        if col >= register - 1 && col <= register + 1 {
            self.output.push('#');
        } else {
            self.output.push('.');
        }
    }
}

impl fmt::Display for Crt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.output.iter().collect::<String>())
    }
}

pub fn parse_input(lines: &[String]) -> Result<(Cpu, Crt)> {
    let mut cpu = Cpu::default();
    let mut crt = Crt::default();

    for line in lines {
        let instruction = Instruction::from_str(line)?;
        crt.draw(&cpu, &instruction);
        cpu.run(&instruction)
    }

    Ok((cpu, crt))
}

pub fn part_one(parsed: &(Cpu, Crt)) -> isize {
    parsed.0.signal_strength
}

pub fn part_two(parsed: &(Cpu, Crt)) -> String {
    parsed.1.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_10.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 13140);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_10.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        let expected = "\
        ##..##..##..##..##..##..##..##..##..##..\n\
        ###...###...###...###...###...###...###.\n\
        ####....####....####....####....####....\n\
        #####.....#####.....#####.....#####.....\n\
        ######......######......######......####\n\
        #######.......#######.......#######.....\
        ";
        assert_eq!(part_two(&parsed), expected);
    }
}
