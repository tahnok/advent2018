mod utils;

fn main() {
    let contents = utils::load_input("inputs/day2.txt");

    let checksum = checksum(&contents);

    println!("{}", checksum);
}

fn count(box_id: &str) -> (bool, bool) {
    let mut found_2 = false;
    let mut found_3 = false;

    let mut chars: Vec<char> = box_id.chars().collect();
    chars.sort();
    let mut iter = chars.iter();
    let mut count = 1;
    let mut last_char = iter.next().unwrap();
    for character in iter {
        if last_char != character && count == 2 {
            found_2 = true;
            count = 1;
        } else if last_char != character && count == 3 {
            found_3 = true;
            count = 1;
        } else if last_char == character {
            count += 1;
        }
        last_char = character;
    }
    if count == 2 {
        found_2 = true;
    } else if count == 3 {
        found_3 = true;
    }
    (found_2,found_3)
}

fn checksum(box_ids: &str) -> usize {
    let mut twos = 0;
    let mut threes = 0;
    for box_id in box_ids.lines() {
        let (two, three) = count(box_id);
        if two {
            twos += 1;
        }
        if three {
            threes += 1;
        }
    }
    println!("twos: {}, threes: {}", twos, threes);
    twos * threes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_no_doubles() {
        assert_eq!((false,false), count("abcdef"));
    }

    #[test]
    fn counts_one_double() {
        assert_eq!((true,false), count("abbcde"));
        assert_eq!((true, false), count("abcdee"));
    }

    #[test]
    fn counts_one_triple() {
        assert_eq!((false,true), count("abcccd"));
        assert_eq!((false,true), count("abcddd"));
    }

    #[test]
    fn counts_one_double_and_one_triple() {
        assert_eq!((true, true), count("bababc"));
    }

    #[test]
    fn counts_two_doubles_once() {
        assert_eq!((true, false), count("aabcdd"));
    }

    #[test]
    fn correct_checksum() {
        let box_ids = vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"];
        assert_eq!(12, checksum(&box_ids));
    }
}