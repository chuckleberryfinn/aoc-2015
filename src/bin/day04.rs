use md5;

fn get_inputs() -> &'static str {
    "yzbqklnj"
}

fn mine(zeros: usize) -> usize {
    (0..)
        .find(|i| {
            format!("{:x}", md5::compute(format!("{}{}", get_inputs(), i)))
                .starts_with(&format!("{:0zeros$}", 0))
        })
        .unwrap()
}

fn part1() -> usize {
    mine(5)
}

fn part2() -> usize {
    mine(6)
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 282_749);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 9_962_624);
    }
}
