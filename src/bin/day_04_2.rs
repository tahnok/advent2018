extern crate regex;
#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;

mod utils;

fn main() {
    lazy_static! {
        static ref guard: Regex = Regex::new(r"#(\d+)").unwrap();
    }

    let raw_contents = utils::load_input("inputs/day4.txt");
    let mut results: Vec<&str> = raw_contents.lines().collect();
    results.sort();

    let mut guards_and_times = HashMap::new();

    let mut iter = results.iter().peekable();
    while let Some(line) = iter.next() {
        if !guard.is_match(line) {
            panic!("line is unexpected: {}", line);
        }
        let id: usize = guard.captures(line).unwrap()[1].parse().unwrap();
        if !guards_and_times.contains_key(&id) {
            guards_and_times.insert(id, HashMap::new());
        }
        let mut times = guards_and_times.get_mut(&id).unwrap();


        loop {
            if iter.peek().is_none() || guard.is_match(iter.peek().unwrap()) {
                break;
            }

            let raw_sleep_time = *iter.next().unwrap();
            let sleep = extract_time(raw_sleep_time);
            let raw_wake_time = *iter.next().unwrap();
            let wake = extract_time(raw_wake_time);
            for time in sleep..wake {
                let count = times.entry(time).or_insert(0);
                *count += 1;
            }
        }
    }
    let mut sleepiest_guard = 0;
    let mut sleepiest_time = 0;
    let mut sleepiest_count = 0;

    for (&id, ref times) in &guards_and_times {
        for (&time, &count) in times.iter() {
            if count > sleepiest_count {
                sleepiest_count = count;
                sleepiest_time = time;
                sleepiest_guard = id;
            }
        }
    }
    println!("{}", sleepiest_guard * sleepiest_time);

}

fn extract_time(raw_sleep_time: &str) -> usize {
    lazy_static! {
        static ref time: Regex = Regex::new(r"(\d+):(\d+)\]").unwrap();
    }
    let capture = time.captures(raw_sleep_time).unwrap();
//    let hour: usize = capture[1].parse().unwrap();
    let minute: usize = capture[2].parse().unwrap();
//    (hour, minute)
    minute
}

