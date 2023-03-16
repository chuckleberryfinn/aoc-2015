use serde_json::Value;
use serde_json::Value::{Array, Object, Number};

fn get_inputs() -> Vec<Value> {
    serde_json::from_str::<Vec<Value>>(
        include_str!("../../input/day12.txt")
    )
    .unwrap()
}

fn parse(item: &Value, to_filter: &'static str) -> i64 {
    match item {
        Number(n) => n.as_i64().unwrap(),
        Array(a) => {
            a.iter().fold(0, |acc, i| acc + parse(i, to_filter))
        },
        Object(o) => {
            match o.values().any(|v| v == to_filter) {
                true => 0,
                false => o.values().fold(0, |acc, i| acc + parse(i, to_filter)),
            }
        },
        _ => 0,
    }
}

fn part1() -> i64 {
    get_inputs()
        .iter()
        .fold(0, |acc, i| acc + parse(i, ""))
}

fn part2() -> i64 {
    get_inputs()
        .iter()
        .fold(0, |acc, i| acc + parse(i, "red"))
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 191_164);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 87_842);
    }
}
