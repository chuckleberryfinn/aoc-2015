use std::collections::HashSet;

fn get_inputs() -> std::str::Chars<'static> {
    include_str!("../../input/day03.txt").chars()
}

fn part1() -> usize {
    get_inputs()
        .scan((0, 0), |state, d| {
            match d {
                '>' => state.0 += 1,
                '<' => state.0 -= 1,
                '^' => state.1 += 1,
                _ => state.1 -= 1,
            };
            Some(*state)
        })
        .chain(std::iter::once((0, 0)))
        .collect::<HashSet<(i32, i32)>>()
        .len()
}

fn part2() -> usize {
    get_inputs()
        .enumerate()
        .scan(vec![(0, 0), (0, 0)], |state, (i, d)| {
            match d {
                '>' => state[i % 2].0 += 1,
                '<' => state[i % 2].0 -= 1,
                '^' => state[i % 2].1 += 1,
                _ => state[i % 2].1 -= 1,
            };
            Some(state[i % 2])
        })
        .chain(std::iter::once((0, 0)))
        .collect::<HashSet<(i32, i32)>>()
        .len()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 2_081);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 2_341);
    }
}
