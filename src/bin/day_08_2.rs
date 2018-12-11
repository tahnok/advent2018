mod utils;

fn main() {
    let contents = utils::load_input("inputs/day8.txt");
    let checksum = calculate_checksum(&contents);

    println!("{}", checksum);
}

fn calculate_checksum(input: &str) -> usize {
    let mut nodes: Vec<usize> = input.trim().split(" ").map(|x| x.parse().unwrap()).collect();
    nodes.reverse();

    recurse(&mut nodes)
}

fn recurse(nodes: &mut Vec<usize>) -> usize {
    let mut checksum = 0;
    let mut childs = nodes.pop().unwrap();
    let mut metadatas = nodes.pop().unwrap();
    if childs == 0 { //leaf node
        while metadatas > 0 {
            metadatas -= 1;
            checksum += nodes.pop().unwrap();
        }
        return checksum;
    } else {
        let mut child_checksums = Vec::new();
        while childs > 0 {
            childs -= 1;
            child_checksums.push(recurse(nodes));
        }
        
        while metadatas > 0 {
            metadatas -= 1;
            let index = nodes.pop().unwrap();
            checksum += child_checksums.get(index - 1).unwrap_or(&0);
        }
        return checksum;
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(66, calculate_checksum("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"));
    }

}