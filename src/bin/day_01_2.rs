use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let filename = "inputs/day1.txt";
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let frequencies: Vec<isize> = contents.trim().lines().map(|line| line.parse::<isize>().unwrap()).collect();
    let mut seen = HashSet::new();
    let mut current_frequency = 0;
    for change in frequencies.iter().cycle() {
        current_frequency += change;
        if seen.contains(&current_frequency) {
            break;
        } else {
            seen.insert(current_frequency);
        }
    }
    println!("{}", current_frequency);
}