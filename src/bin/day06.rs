use regex::Regex;

fn get_inputs() -> Vec<(String, usize, usize, usize, usize)> {
    let re = Regex::new(r"(turn )?(off|on|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    re.captures_iter(include_str!("../../input/day06.txt"))
        .map(|caps| {
            (caps[2].to_string(), caps[3].parse().unwrap(), caps[4].parse().unwrap(), caps[5].parse().unwrap(), caps[6].parse().unwrap())
        })
        .collect()
}

fn part1() -> i32 {
    get_inputs()
        .iter()
        .scan(vec![vec![false; 1_000]; 1_000], |lights, input| {
            Some((input.1..=input.3)
                .map(|r|
                    (input.2..=input.4)
                        .map(|c|
                            match &input.0[..] {
                                "off" => {
                                    match lights[r][c] {
                                        true => {
                                            lights[r][c] = false;
                                            -1
                                        },
                                        _ => 0
                                    }
                                },
                                "on" => {
                                    match lights[r][c] {
                                        false => {
                                            lights[r][c] = true;
                                            1
                                        },
                                        _ => 0
                                    }
                                },
                                _ => {
                                    lights[r][c] ^= true;
                                    match lights[r][c] {
                                        true => {
                                            1
                                        },
                                        _ => -1
                                    }
                                }
                            }
                        )
                        .sum::<i32>()
                    )
                .sum::<i32>()
            )
        })
        .sum()
}

fn part2() -> i32 {
    get_inputs()
        .iter()
        .scan(vec![vec![0usize; 1_000]; 1_000], |lights, input| {
            Some((input.1..=input.3)
                .map(|r|
                    (input.2..=input.4)
                        .map(|c|
                            match &input.0[..] {
                                "off" => {
                                    match lights[r][c] {
                                        0 => 0,
                                        _ => {
                                            lights[r][c] -= 1;
                                            -1
                                        }
                                    }
                                },
                                "on" => {
                                    lights[r][c] += 1;
                                    1
                                },
                                _ => {
                                    lights[r][c] += 2;
                                    2
                                }
                            }
                    ).sum::<i32>()
                ).sum::<i32>()
            )
        })
     .sum()
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
