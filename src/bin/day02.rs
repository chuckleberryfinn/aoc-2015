use itertools::Itertools;

fn get_inputs() -> Vec<(usize, usize, usize)> {
    include_str!("../../input/day02.txt")
        .lines()
        .map(|l| l.split('x').map(|x| x.parse().unwrap()).sorted().collect_tuple().unwrap())
        .collect()
}

fn part1() -> usize {
    get_inputs().iter().fold(0, |acc, x|
        acc + (2 * ((x.0 * x.1) + (x.1 * x.2) + (x.0 * x.2)) + (x.0 * x.1))
    )
}

fn part2() -> usize {
    get_inputs().iter().fold(0, |acc, x|
        acc + (2 * (x.0 + x.1)) + (x.0 * x.1 * x.2)
    )
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 1_606_483);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 3_842_356);
    }
}
