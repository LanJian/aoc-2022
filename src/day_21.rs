use anyhow::{bail, Error, Result};
use rustc_hash::FxHashMap;
use std::str::FromStr;

fn id_to_usize(s: &str) -> usize {
    let mut ret = 0;

    for c in s.chars() {
        let value = c as usize - 'a' as usize;
        ret = (ret + value) * 100;
    }

    ret
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Invert,
}

impl Operator {
    fn apply_to(&self, a: &Number, b: &Number) -> Number {
        let value = match self {
            Operator::Add => (a.nume * b.denom + b.nume * a.denom, a.denom * b.denom),
            Operator::Sub => (a.nume * b.denom - b.nume * a.denom, a.denom * b.denom),
            Operator::Mult => (a.nume * b.nume, a.denom * b.denom),
            Operator::Div => (a.nume * b.denom, a.denom * b.nume),
            Operator::Invert => (a.denom, a.nume),
        };

        Number::new(value.0, value.1)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Operation {
    operator: Operator,
    number: Number,
}

impl Operation {
    fn new(operator: Operator, number: Number) -> Self {
        Self { operator, number }
    }

    fn apply_to(&self, value: Number) -> Number {
        self.operator.apply_to(&value, &self.number)
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Number {
    nume: i64,
    denom: i64,
}

impl Number {
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            return a;
        }

        Self::gcd(b, a % b)
    }

    pub fn new(n: i64, d: i64) -> Self {
        let gcd = Self::gcd(n, d);
        Self {
            nume: n / gcd,
            denom: d / gcd,
        }
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::new(value, 1)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum EvalResult {
    Number(Number),
    Operations(Vec<Operation>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Expression {
    Number(i64),
    Add(usize, usize),
    Sub(usize, usize),
    Mult(usize, usize),
    Div(usize, usize),
}

impl FromStr for Expression {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_whitespace().collect();

        if tokens.len() == 1 {
            return Ok(Self::Number(tokens[0].parse()?));
        }

        if tokens.len() != 3 {
            bail!("Invalid expression: {}", s);
        }

        let a = id_to_usize(tokens[0]);
        let b = id_to_usize(tokens[2]);

        let expr = match tokens[1] {
            "+" => Self::Add(a, b),
            "-" => Self::Sub(a, b),
            "*" => Self::Mult(a, b),
            "/" => Self::Div(a, b),
            _ => bail!("Invalid operator: {}", tokens[1]),
        };

        Ok(expr)
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Riddle {
    bindings: FxHashMap<usize, Expression>,
}

impl Riddle {
    const ROOT: usize = 1714141900;
    const HUMN: usize = 720121300;

    fn evaluate(&self) -> Result<i64> {
        self.evaluate_helper(&Self::ROOT)
    }

    fn evaluate_helper(&self, var: &usize) -> Result<i64> {
        if self.bindings.get(var).is_none() {
            bail!("Could not find binding for var: {}", var);
        }

        let value = match self.bindings.get(var).unwrap() {
            Expression::Number(n) => *n,
            Expression::Add(a, b) => self.evaluate_helper(a)? + self.evaluate_helper(b)?,
            Expression::Sub(a, b) => self.evaluate_helper(a)? - self.evaluate_helper(b)?,
            Expression::Mult(a, b) => self.evaluate_helper(a)? * self.evaluate_helper(b)?,
            Expression::Div(a, b) => self.evaluate_helper(a)? / self.evaluate_helper(b)?,
        };

        Ok(value)
    }

    fn solve(&self) -> Result<i64> {
        let var = &Self::ROOT;

        if self.bindings.get(var).is_none() {
            bail!("Could not find binding for var: {}", var);
        }

        let value = match self.bindings.get(var).unwrap() {
            Expression::Number(_) => bail!("Expected root to not be a number"),
            Expression::Add(a, b)
            | Expression::Sub(a, b)
            | Expression::Mult(a, b)
            | Expression::Div(a, b) => {
                let left = self.solve_helper(a)?;
                let right = self.solve_helper(b)?;

                match (left, right) {
                    (EvalResult::Number(value), EvalResult::Operations(ops))
                    | (EvalResult::Operations(ops), EvalResult::Number(value)) => {
                        ops.iter().rfold(value, |acc, op| op.apply_to(acc))
                    }
                    _ => bail!("Unexpected input"),
                }
            }
        };

        Ok(value.nume / value.denom)
    }

    fn solve_helper(&self, var: &usize) -> Result<EvalResult> {
        if self.bindings.get(var).is_none() {
            bail!("Could not find binding for var: {}", var);
        }

        if var == &Self::HUMN {
            return Ok(EvalResult::Operations(Vec::with_capacity(
                self.bindings.len(),
            )));
        }

        let eval_result = match self.bindings.get(var).unwrap() {
            Expression::Number(n) => EvalResult::Number((*n).into()),
            Expression::Add(a, b) => {
                self.combine(self.solve_helper(a)?, self.solve_helper(b)?, Operator::Add)
            }
            Expression::Sub(a, b) => {
                self.combine(self.solve_helper(a)?, self.solve_helper(b)?, Operator::Sub)
            }
            Expression::Mult(a, b) => {
                self.combine(self.solve_helper(a)?, self.solve_helper(b)?, Operator::Mult)
            }
            Expression::Div(a, b) => {
                self.combine(self.solve_helper(a)?, self.solve_helper(b)?, Operator::Div)
            }
        };

        Ok(eval_result)
    }

    fn combine(&self, left: EvalResult, right: EvalResult, op: Operator) -> EvalResult {
        match (left, right) {
            (EvalResult::Number(a), EvalResult::Number(b)) => {
                EvalResult::Number(op.apply_to(&a, &b))
            }
            (EvalResult::Operations(mut ops), EvalResult::Number(value)) => {
                match op {
                    Operator::Add => ops.push(Operation::new(Operator::Sub, value)),
                    Operator::Sub => ops.push(Operation::new(Operator::Add, value)),
                    Operator::Mult => ops.push(Operation::new(Operator::Div, value)),
                    Operator::Div => ops.push(Operation::new(Operator::Mult, value)),
                    Operator::Invert => unreachable!(),
                }

                EvalResult::Operations(ops)
            }
            (EvalResult::Number(value), EvalResult::Operations(mut ops)) => {
                match op {
                    Operator::Add => ops.push(Operation::new(Operator::Sub, value)),
                    Operator::Sub => {
                        ops.push(Operation::new(Operator::Mult, (-1).into()));
                        ops.push(Operation::new(Operator::Sub, value));
                    }
                    Operator::Mult => ops.push(Operation::new(Operator::Div, value)),
                    Operator::Div => {
                        ops.push(Operation::new(Operator::Mult, value));
                        ops.push(Operation::new(Operator::Invert, 1.into()));
                    }
                    Operator::Invert => unreachable!(),
                }

                EvalResult::Operations(ops)
            }
            _ => unreachable!(),
        }
    }
}

impl TryFrom<&[String]> for Riddle {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let mut bindings = FxHashMap::default();

        for l in lines {
            let tokens: Vec<_> = l.split(": ").collect();
            bindings.insert(id_to_usize(tokens[0]), Expression::from_str(tokens[1])?);
        }

        Ok(Riddle { bindings })
    }
}

pub fn parse_input(lines: &[String]) -> Result<Riddle> {
    Riddle::try_from(lines)
}

pub fn part_one(parsed: &Riddle) -> i64 {
    parsed.evaluate().expect("Could not evaluate")
}

pub fn part_two(parsed: &Riddle) -> i64 {
    parsed.solve().expect("Could not solve")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_21.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 152);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_21.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 301);
    }

    #[test]
    fn part_two_div_test() {
        let lines = utils::load_input("inputs/day_21.test").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 3);
    }

    #[test]
    fn part_two_sub_test() {
        let lines = utils::load_input("inputs/day_21.test.2").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 8);
    }
}
