fn get_inputs() -> Vec<char> {
    include_str!("../../input/day01.txt").chars().collect()
}

fn part1() -> i32 {
    get_inputs().iter().fold(0, |acc, x| {
        acc + match x {
            '(' => 1,
            _ => -1,
        }
    })
}

fn part2() -> usize {
    get_inputs()
        .iter()
        .enumerate()
        .scan(0, |state, (i, x)| {
            if *state < 0 {
                return None;
            }

            match x {
                '(' => *state += 1,
                _ => *state -= 1,
            };
            Some(i)
        })
        .last()
        .unwrap()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 232);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(), 1_782);
    }
}
