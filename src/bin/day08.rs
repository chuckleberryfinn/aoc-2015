use regex::Regex;

fn get_inputs() -> Vec<&'static str> {
    include_str!("../../input/day08.txt").lines().collect()
}

fn part1() -> usize {
    let re = Regex::new(r#"\\"|\\\\|\\x[0-9a-f]{2}"#).unwrap();
    get_inputs().iter().fold(0, |acc, s| {
        acc + (s.len() - re.replace_all(s, " ").len()) + 2
    })
}

fn part2() -> usize {
    let re = Regex::new(r#""|\\"#).unwrap();
    get_inputs().iter().fold(0, |acc, s| {
        acc + (re.replace_all(s, "  ").len() - s.len()) + 2
    })
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 1_333);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 2_046);
    }
}
