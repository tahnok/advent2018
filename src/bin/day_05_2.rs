mod utils;

fn main() {
    let contents = utils::load_input("inputs/day5.txt");
    let result = shortest(&contents);

    println!("{}", result.len() - 1);
}

fn reacts(unit_a: char, unit_b: char) -> bool {
    (unit_a.is_uppercase() && unit_b.is_lowercase() && unit_a == unit_b.to_ascii_uppercase()) ||
        (unit_a.is_lowercase() && unit_b.is_uppercase() && unit_a == unit_b.to_ascii_lowercase())
}

fn shortest(units: &str) -> String {
    let mut shortest = std::usize::MAX;
    let mut result= String::new();
    for raw_to_drop in 97u8..123u8 {
        let to_drop = raw_to_drop as char;
        let x = do_reaction(units, to_drop);
        if x.len() < shortest {
            shortest = x.len();
            result = x;
        }
    }
    result
}


fn do_reaction(units: &str, to_drop: char) -> String {
    let mut chars = units.chars().peekable();
    let mut result: Vec<char> = Vec::new();
    while let Some(c) = chars.next() {
        if c.to_ascii_lowercase() != to_drop.to_ascii_lowercase() {
            result.push(c);
            break;
        }
    }

    loop {
        if let Some(current) = chars.next() {
            if current.to_ascii_lowercase() == to_drop.to_ascii_lowercase()  {
                continue;
            }
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
    fn complex_reaction() {
        assert_eq!("dbCBcD", do_reaction("dabAcCaCBAcCcaDA", 'a'))
    }

    #[test]
    fn finds_shortest() {
        assert_eq!("daDA", shortest("dabAcCaCBAcCcaDA"));
    }

}