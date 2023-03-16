fn run(reps: usize) -> usize {
    (0..reps)
        .scan("1321131112".bytes().collect(), |state: &mut Vec<u8>, _| {
            let mut l = 0;
            let mut r = 0;
            let mut ns: Vec<u8> = vec![];

            while r < state.len() {
                match state[l] == state[r] {
                    true => r += 1,
                    false => {
                        ns.extend((r - l).to_string().bytes());
                        ns.push(state[l]);
                        l = r;
                    }
                };
            }
            ns.extend((r - l).to_string().bytes());
            ns.push(state[l]);
            *state = ns;
            Some(state.len())
        })
        .last()
        .unwrap()
}

fn part1() -> usize {
    run(40)
}

fn part2() -> usize {
    run(50)
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 492_982);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 6_989_950);
    }
}
