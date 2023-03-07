use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use regex::Regex;

fn get_inputs() -> HashMap<&'static str, Vec<(&'static str, usize)>> {
    let tuples: Vec<(&'static str, (&'static str, usize))> = Regex::new(r"(.*) to (.*) = (\d+)")
        .unwrap()
        .captures_iter(include_str!("../../input/day09.txt"))
        .map(|caps| {
            (
                caps.get(1).unwrap().as_str(),
                (
                    caps.get(2).unwrap().as_str(),
                    caps[3].parse::<usize>().unwrap(),
                ),
            )
        })
        .collect();

    let mut m = HashMap::new();
    for (k, v) in tuples {
        m.entry(k).or_insert_with(Vec::new).push(v);
        m.entry(v.0).or_insert_with(Vec::new).push((k, v.1));
    }
    m
}

fn dfs<F>(
    graph: &HashMap<&'static str, Vec<(&'static str, usize)>>,
    start: &'static str,
    seen: &mut HashSet<&'static str>,
    mut comp: F,
) -> usize
where
    F: FnMut(&&(&str, usize), &&(&str, usize)) -> std::cmp::Ordering,
{
    seen.insert(start);
    if seen.len() == graph.keys().len() {
        return 0;
    }
    let m = graph
        .get(start)
        .unwrap()
        .iter()
        .filter(|e| !seen.contains(&e.0))
        .sorted_by(&mut comp)
        .next()
        .unwrap();
    m.1 + dfs(graph, m.0, seen, comp)
}

fn run<F, G>(mut comp: F, mut comp1: G) -> usize
where
    F: FnMut(&&(&str, usize), &&(&str, usize)) -> std::cmp::Ordering,
    G: FnMut(&usize, &usize) -> std::cmp::Ordering,
{
    let graph = get_inputs();
    graph
        .keys()
        .map(|k| dfs(&graph, k, &mut HashSet::<&'static str>::new(), &mut comp))
        .sorted_by(&mut comp1)
        .next()
        .unwrap()
}

fn part1() -> usize {
    run(
        |x: &&(&str, usize), y: &&(&str, usize)| x.1.cmp(&y.1),
        |x: &usize, y: &usize| x.cmp(y),
    )
}

fn part2() -> usize {
    run(
        |x: &&(&str, usize), y: &&(&str, usize)| y.1.cmp(&x.1),
        |x: &usize, y: &usize| y.cmp(x),
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
        assert_eq!(super::part1(), 117);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 909);
    }
}
