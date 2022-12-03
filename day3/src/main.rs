use std::collections::HashMap;
use std::fs;

fn read_file(fp: &str) -> Vec<(String, String)> {
    let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");
    let knapsacks: Vec<(&str, &str)> = contents.lines().map(|l| l.split_at(l.len() / 2)).collect();

    // Convert the &str to String to avoid lifetime issues.
    let knapsacks_str: Vec<(String, String)> = knapsacks
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();

    return knapsacks_str;
}

fn read_file_triplet(fp: &str) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");
    let mut knapsacks: Vec<Vec<String>> = vec![];

    let mut sublist: Vec<String> = vec![];
    for (c, l) in contents.lines().enumerate() {
        sublist.push(l.to_string());
        if c % 3 == 2 {
            knapsacks.push(sublist);
            sublist = vec![];
        }
    }

    return knapsacks;
}

fn determine_ascii_value(c: char) -> i32 {
    let ascii_value = c as u8;
    if ascii_value > 64 && ascii_value < 91 {
        return ascii_value as i32 - 64 + 26;
    } else if ascii_value > 96 && ascii_value < 123 {
        return ascii_value as i32 - 96;
    } else {
        panic!("Invalid character {}", c);
    }
}

fn determine_chars_in_common(t: Vec<(String, String)>) -> i32 {
    let mut shared_values: Vec<i32> = vec![];

    let mut first_hash: HashMap<char, i32> = HashMap::new();
    let mut second_hash: HashMap<char, i32> = HashMap::new();
    for (a, b) in t {
        first_hash.clear();
        second_hash.clear();

        for c in a.chars() {
            first_hash.entry(c).or_insert(determine_ascii_value(c));
        }
        for c in b.chars() {
            second_hash.entry(c).or_insert(determine_ascii_value(c));
        }

        let mut common_chars: Vec<(char, i32)> = vec![];
        for (k, v) in first_hash.iter() {
            if second_hash.contains_key(k) {
                common_chars.push((*k, *v));
                shared_values.push(*v);
            }
        }
        // println!("{:?}", common_chars);
    }
    // println!("{:?}", first_hash);

    return shared_values.iter().sum();
}

fn return_string_hash(s: String) -> HashMap<char, i32> {
    let mut hash: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        hash.entry(c).or_insert(determine_ascii_value(c));
    }
    return hash;
}

fn determine_chars_in_common_ncases(t: Vec<Vec<String>>) -> i32 {
    let mut shared_values: Vec<i32> = vec![];

    for groups in t {
        let mut hashes: Vec<HashMap<char, i32>> = vec![];
        for s in groups {
            hashes.push(return_string_hash(s.to_string()));
        }

        for (k, v) in hashes[0].iter() {
            if hashes.iter().all(|h| h.contains_key(k)) {
                shared_values.push(*v);
            }
        }
    }

    return shared_values.iter().sum();
}

fn main() {
    let data = read_file("input.txt");
    println!("{}", determine_chars_in_common(data));

    let data2 = read_file_triplet("input.txt");
    println!("{}", determine_chars_in_common_ncases(data2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_file("sample.txt");
        assert_eq!(determine_chars_in_common(data), 157);
    }

    #[test]
    fn test_part2() {
        let data = read_file_triplet("sample.txt");
        assert_eq!(determine_chars_in_common_ncases(data), 70);
    }
}
