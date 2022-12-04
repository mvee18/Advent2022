use std::fs;
use std::collections::HashSet;

fn read_file(fp: &str) -> Vec<HashSet<i32>> {
    let contents = fs::read_to_string(fp)
        .expect("Something went wrong reading the file");
    
    // We need to expand the range (ex. 1-3) to a list of numbers (ex. [1, 2, 3]).
    let mut result: Vec<HashSet<i32>> = vec![];
    for line in contents.lines() {
        let ranges: Vec<&str> = line.split(",").collect();
        for r in ranges {
            let range: Vec<&str> = r.split("-").collect();
            let start = range[0].parse::<i32>().unwrap();
            let end = range[1].parse::<i32>().unwrap();
            let range: HashSet<i32> = (start..=end).collect();
            result.push(range);
        }
    }

    result
}

fn determine_overlap(ranges: &Vec<HashSet<i32>>) -> (i32, i32) {
    let mut fully_overlap_count: i32 = 0;
    let mut any_overlap_count: i32 = 0;
    
    ranges.chunks(2).for_each(|chunk| {
        let overlap: HashSet<i32> = chunk[0].intersection(&chunk[1]).cloned().collect();

        // If the overlap length is equal to the length of either of the ranges, 
        // then one set is fully contained within the other.
        if overlap.len() == chunk[0].len() || overlap.len() == chunk[1].len() {
            fully_overlap_count += 1;
        }

        // Not mutually exclusive with the above condition.
        if overlap.len() > 0  {
            any_overlap_count += 1;
        }
    });

    (fully_overlap_count, any_overlap_count)
}

fn main() {
    let (fully_overlap_count, any_overlap_count) = determine_overlap(&read_file("input.txt"));

    println!("The fully overlap count is: {}", fully_overlap_count);
    println!("The any overlap count is: {}", any_overlap_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_overlap() {
        let overlap_count = determine_overlap(&read_file("sample.txt"));

        assert_eq!(overlap_count.0, 2);
        assert_eq!(overlap_count.1, 4);
    }
}