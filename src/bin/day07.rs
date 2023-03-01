use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Instruction {
    operation: Option<&'static str>,
    destination: &'static str,
    operand1: &'static str,
    operand2: Option<&'static str>,
}

fn get_inputs() -> Vec<Instruction> {
    let re = Regex::new(r"((?P<op2>[0-9a-z]*) )?((?P<op>NOT|AND|OR|RSHIFT|LSHIFT) )?(?P<op1>[0-9a-z]+) -> (?P<dest>[a-z]+)").unwrap();
    re.captures_iter(include_str!("../../input/day07.txt"))
        .map(|caps| Instruction {
            operation: match caps.name("op") {
                Some(m) => Some(m.as_str()),
                None => None,
            },
            destination: caps.name("dest").unwrap().as_str(),
            operand1: caps.name("op1").unwrap().as_str(),
            operand2: match caps.name("op2") {
                Some(m) => Some(m.as_str()),
                None => None,
            },
        })
        .collect()
}

fn calculate(mut wires: HashMap<&str, u16>) -> u16 {
    let inputs = get_inputs();
    loop {
        if inputs
            .iter()
            .find(|instruction| {
                match instruction.operation {
                    Some(i) => match i {
                        "AND" | "OR" => {
                            if (instruction.operand1.parse::<u16>().is_err()
                                & !wires.contains_key(instruction.operand1))
                                | (instruction.operand2.unwrap().parse::<u16>().is_err()
                                    & !wires.contains_key(&instruction.operand2.unwrap()))
                            {
                                return false;
                            }
                            let op = match i {
                                "AND" => |x: u16, y: u16| x & y,
                                _ => |x: u16, y: u16| x | y,
                            };
                            wires.insert(
                                instruction.destination,
                                op(
                                    instruction.operand1.parse::<u16>().unwrap_or_else(|_| {
                                        *wires.get(instruction.operand1).unwrap()
                                    }),
                                    instruction.operand2.unwrap().parse::<u16>().unwrap_or_else(
                                        |_| *wires.get(&instruction.operand2.unwrap()).unwrap(),
                                    ),
                                ),
                            );
                        }
                        "LSHIFT" | "RSHIFT" => {
                            if !wires.contains_key(&instruction.operand2.unwrap()) {
                                return false;
                            }
                            let op = match i {
                                "LSHIFT" => |x: u16, y: u16| x << y,
                                _ => |x: u16, y: u16| x >> y,
                            };
                            wires.insert(
                                instruction.destination,
                                op(
                                    *wires.get(&instruction.operand2.unwrap()).unwrap(),
                                    instruction.operand1.parse::<u16>().unwrap(),
                                ),
                            );
                        }
                        _ => {
                            if !wires.contains_key(&instruction.operand1) {
                                return false;
                            }
                            wires.insert(
                                instruction.destination,
                                !*wires.get(&instruction.operand1).unwrap(),
                            );
                        }
                    },
                    None => {
                        if wires.contains_key(instruction.destination)
                            | (instruction.operand1.parse::<u16>().is_err()
                                & !wires.contains_key(&instruction.operand1))
                        {
                            return false;
                        }
                        wires.insert(
                            instruction.destination,
                            instruction
                                .operand1
                                .parse::<u16>()
                                .unwrap_or_else(|_| *wires.get(&instruction.operand1).unwrap()),
                        );
                    }
                };
                wires.contains_key(&"a")
            })
            .is_some()
        {
            return *wires.get(&"a").unwrap();
        };
    }
}

fn part1() -> u16 {
    calculate(HashMap::<&str, u16>::new())
}

fn part2() -> u16 {
    calculate(HashMap::from([("b", part1())]))
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 956);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 40_149);
    }
}
