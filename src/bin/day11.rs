fn is_valid(pass: &Vec<u8>) -> bool {
    if pass.contains(&('i' as u8)) | pass.contains(&('o' as u8)) | pass.contains(&('l' as u8)) {
        return false;
    }

    let mut valid = false;
    for w in pass.windows(3) {
        if (w[0] == w[1] - 1) & (w[1] == w[2] - 1) {
            valid = true;
            break;
        }
    }

    if !valid {
        return valid;
    }

    valid = false;
    let mut i = 0;
    let mut c = false;
    while i < 7 {
        if pass[i] == pass[i+1] {
            if c {
                valid = true;
                break;
            } else {
                c = true;
                i += 1;
            }
        }
        i += 1;
    }

    valid
}

fn part1(seed: &str) -> String {
    let mut seed = seed.bytes().collect();
    (0..)
        .scan(&mut seed, | pass: &mut &mut Vec<u8>, _ | {
            for x in (0..8).rev() {
                if pass[x] == 'z' as u8 {
                    pass[x] = 'a' as u8;
                } else {
                    pass[x] += 1;
                    break;
                }
            }

            match is_valid(&pass) {
                true => None,
                false => Some(1),
            }
        })
        .for_each(drop);
    std::str::from_utf8(&seed).unwrap().to_string()
}

fn part2() -> String {
    part1(&part1("hepxcrrq"))
}

fn main() {
    println!("{}", part1("hepxcrrq"));
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1("hepxcrrq"), "hepxxyzz".to_string());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), "heqaabcc".to_string());
    }
}
