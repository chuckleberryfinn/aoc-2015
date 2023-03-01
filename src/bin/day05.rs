use regex::Regex;

fn get_inputs() -> Vec<&'static str> {
    include_str!("../../input/day05.txt").lines().collect()
}

fn part1() -> usize {
    let re = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    get_inputs()
        .iter()
        .filter(|l| {
            !re.is_match(&l)
                & (l.chars().filter(|c| "aeiou".contains(*c)).count() >= 3)
                & (l.chars()
                    .collect::<Vec<char>>()
                    .windows(2)
                    .any(|w| w[0] == w[1]))
        })
        .count()
}

fn part2() -> usize {
    get_inputs()
        .iter()
        .filter(|l| {
            (l.chars()
                .collect::<Vec<char>>()
                .windows(3)
                .any(|w| w[0] == w[2]))
                & (l.chars()
                    .collect::<Vec<char>>()
                    .windows(2)
                    .enumerate()
                    .any(|(i, w)| l[i + 2..].contains(&w.into_iter().collect::<String>())))
        })
        .count()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 236);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 51);
    }
}
