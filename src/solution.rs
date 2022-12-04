use std::{env, fmt::Display};

use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct Solution<T, G>
where
    T: Display + Serialize + PartialEq,
    G: Display + Serialize + PartialEq,
{
    pub part_one: T,
    pub part_two: G,
}

impl<T, G> Display for Solution<T, G>
where
    T: Display + Serialize + PartialEq,
    G: Display + Serialize + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if env::var("AOC_OUTPUT_JSON").is_ok() {
            write!(
                f,
                "{}",
                serde_json::to_string(&self).expect("Unable to convert self to json")
            )
        } else {
            write!(f, "part 1: {}\npart 2: {}", self.part_one, self.part_two)
        }
    }
}
