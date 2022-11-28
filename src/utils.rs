use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn load_input(default_path: &str) -> Result<Vec<String>, io::Error> {
    let path = env::var("AOC_INPUT").unwrap_or(default_path.to_string());
    load_lines(&path)
}

pub fn load_lines(path: &str) -> Result<Vec<String>, io::Error> {
    BufReader::new(File::open(path)?).lines().collect()
}

pub fn split_lines(s: &str) -> Vec<String> {
    s.split_terminator("\n")
        .map(|x| x.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_lines_test() {
        let s = "6,10
                 0,14

                 fold along y=7
                 fold along x=5";

        let actual = split_lines(s);
        let expected = vec![
            "6,10".to_string(),
            "0,14".to_string(),
            "".to_string(),
            "fold along y=7".to_string(),
            "fold along x=5".to_string(),
        ];

        assert_eq!(actual, expected);
    }
}
