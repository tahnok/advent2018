extern crate regex;
#[macro_use] extern crate lazy_static;

use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

mod utils;

fn main() {
    let contents = utils::load_input("inputs/day3.txt");
    let result = no_overlap(&contents);
    println!("{}", result);
}

fn parse(raw_claim: &str) -> (usize, Vec<(usize, usize)>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }
    let captures = RE.captures_iter(raw_claim).next().unwrap();
    let id: usize = captures[1].parse().unwrap();
    let start_x: usize = captures[2].parse().unwrap();
    let start_y: usize = captures[3].parse().unwrap();
    let width: usize = captures[4].parse().unwrap();
    let height: usize = captures[5].parse().unwrap();

    let mut positions = Vec::with_capacity(width * height);

    for x in start_x..start_x + width {
        for y in start_y..start_y + height {
            positions.push((x,y));
        }
    }
    (id, positions)
}

fn no_overlap(raw_claims: &str) -> usize {
    let mut claims = HashMap::new();
    let mut ids = HashSet::new();
    for raw_claim in raw_claims.lines() {
        let (id, positions) = parse(raw_claim);
        ids.insert(id);
        for position in positions {
            let claim_at = claims.entry(position).or_insert(Vec::new());
            claim_at.push(id);
        }
    }
    for (_, claims_at) in claims {
        if claims_at.len() > 1 {
            for id in claims_at {
                ids.remove(&id);
            }
        }
    }
    if ids.len() == 1 {
        for id in &ids {
            return *id;
        }
        panic!("unreachable?");
    } else {
        panic!("Not found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_claim() {
        let raw_claim = "#1 @ 1,3: 4x4";
        let claim = parse(&raw_claim);
        let expected = vec!(
            (1,3), (2,3), (3,3), (4,3),
            (1,4), (2,4), (3,4), (4,4),
            (1,5), (2,5), (3,5), (4,5),
            (1,6), (2,6), (3,6), (4,6),
        );
        for pair in expected {
            assert!(claim.1.contains(&pair));
        }
        assert_eq!(1, claim.0)
    }

    #[test]
    fn finds_no_overlap() {
        let claims = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"].join("\n");
        assert_eq!(3, no_overlap(&claims));
    }
}