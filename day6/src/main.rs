use std::collections::{HashSet, VecDeque};

fn read_file(fp: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(fp).expect("Something went wrong reading the file");

    let results = contents.split_whitespace().map(|s| s.to_string()).collect();

    results
}

// Determine if the characters in the VecDeque are unique.
fn determine_if_unique(input: VecDeque<char>) -> bool {
    // Make the characters into a HashSet
    // println!("{:?}", input);
    let mut unique: HashSet<char> = HashSet::new();
    for c in &input {
        unique.insert(*c);
    }

    // If the length of the HashSet is equal to the length of the VecDeque, then
    // the characters are unique.
    if unique.len() == input.len() {
        return true;
    } else {
        return false;
    }
}

fn determine_unique_position(input: String, pos: usize) -> i32 {
    let mut seen: VecDeque<char> = VecDeque::new();

    for (i, c) in input.chars().enumerate() {
        if seen.len() == pos {
            if determine_if_unique(seen.clone()) {
                return i as i32;
            }
            seen.pop_front();
        }
        seen.push_back(c);
    }

    return 0;
}

fn main() {
    let input = read_file("input.txt");
    let result = determine_unique_position(input[0].clone(), 4);
    println!("Result: {}", result);

    let result2 = determine_unique_position(input[0].clone(), 14);
    println!("Result2: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_unique() {
        let input = read_file("sample.txt");
        let result = determine_unique_position(input[0].clone(), 4);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_fourteen_unique() {
        let input = read_file("sample.txt");
        let result = determine_unique_position(input[0].clone(), 14);
        assert_eq!(result, 19);
    }
}
