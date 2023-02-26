use regex::Regex;

fn get_inputs() -> Vec<(String, usize, usize, usize, usize)> {
    let re = Regex::new(r"(turn )?(off|on|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    re.captures_iter(include_str!("../../input/day06.txt"))
        .map(|caps| {
            (caps[2].to_string(), caps[3].parse().unwrap(), caps[4].parse().unwrap(), caps[5].parse().unwrap(), caps[6].parse().unwrap())
        })
        .collect()
}

fn part1() -> usize {
    let mut lights = vec![vec![false; 1_000]; 1_000];
    for input in get_inputs() {
        for r in &mut lights[input.1..=input.3] {
            for c in &mut r[input.2..=input.4] {
                match &input.0[..] {
                    "off" => *c = false,
                    "on" => *c = true,
                    "toggle" => *c ^= true,
                    _ => panic!("Unexpected input {}", input.0)
                }
            }
        };
    }
    lights.iter().flat_map(|r| r.iter()).filter(|&&l| l == true).count()
}

fn part2() -> usize {
    let mut lights = vec![vec![0usize; 1_000]; 1_000];
    for input in get_inputs() {
        for r in &mut lights[input.1..=input.3] {
            for c in &mut r[input.2..=input.4] {
                match &input.0[..] {
                    "off" => *c = c.saturating_sub(1),
                    "on" => *c += 1,
                    "toggle" => *c += 2,
                    _ => panic!("Unexpected input {}", input.0)
                }
            }
        };
    }
    lights.iter().flat_map(|r| r.iter()).sum()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 400_410);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 15_343_601);
    }
}
