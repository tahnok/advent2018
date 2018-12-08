mod utils;

fn main() {
    let contents = utils::load_input("inputs/day5.txt");
    let result = do_reaction(&contents);

    println!("{}", result.len() - 1);
}

fn reacts(unit_a: char, unit_b: char) -> bool {
    (unit_a.is_uppercase() && unit_b.is_lowercase() && unit_a == unit_b.to_ascii_uppercase()) ||
        (unit_a.is_lowercase() && unit_b.is_uppercase() && unit_a == unit_b.to_ascii_lowercase())
}

fn do_reaction(units: &str) -> String {
    let mut chars = units.chars().peekable();
    let mut result: Vec<char> = Vec::new();
    result.push(chars.next().unwrap());

    loop {
        if let Some(current) = chars.next() {
            if let Some(last) = result.pop() {
                if !reacts(current, last) {
                    result.push(last);
                    result.push(current);
                }
            } else {
               result.push(current);
            }
        } else {
            break;
        }
    }

    result.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposite_polarity_reacts() {
        assert!(reacts('a', 'A'));
        assert!(reacts('A', 'a'));
        assert!(reacts('b', 'B'));
    }

    #[test]
    fn everything_else_no_reacts() {
        assert!(!reacts('b', 'C'));
        assert!(!reacts('b', 'b'));
        assert!(!reacts('c', 'c'));
    }

    #[test]
    fn simple_reaction() {
        assert_eq!("", do_reaction("aA"));
    }

    #[test]
    fn less_simple_reaction() {
        assert_eq!("", do_reaction("abBA"));
    }

    #[test]
    fn dud_reaction() {
        assert_eq!("abAB", do_reaction("abAB"));
    }

    #[test]
    fn complex_reaction() {
        assert_eq!("dabCBAcaDA", do_reaction("dabAcCaCBAcCcaDA"))
    }

}