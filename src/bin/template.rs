mod utils;

fn main() {
    let contents = utils::load_input("inputs/dayN.txt");

    println!("{}", contents);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        assert!(true);
    }

}