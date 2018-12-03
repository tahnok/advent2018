mod utils;

fn main() {
    let contents = utils::load_input("inputs/day2.txt");
    let mut box_ids = contents.lines();
    while let Some(box_id1) = box_ids.next() {
        for box_id2 in box_ids.clone() {
            if compare(&box_id1, &box_id2) {
                print_all_matching(box_id1, box_id2);
                return;
            }
        }
    }
}

fn print_all_matching(left: &str, right: &str) {
    for (x,y) in left.chars().zip(right.chars()) {
        if x == y {
            print!("{}", x)
        }
    }
    print!("\n");
}

fn compare(left: &str, right: &str) -> bool {
    let mut mismatch = false;
    for (x,y) in (left.chars()).zip(right.chars()) {
        if x != y {
            if mismatch {
                mismatch = false;
                break;
            } else {
                mismatch = true;
            }
        }
    }
    mismatch
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_match() {
        assert!(!compare(&"abcde", &"fghij"));
        assert!(!compare(&"abcde", &"abcij"));
    }

    #[test]
    fn find_match() {
        assert!(compare(&"fghij", &"fguij"));
    }
}