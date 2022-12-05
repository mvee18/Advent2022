use std::collections::VecDeque;
use std::fs;

fn parse_file(fp: &str) -> (Vec<VecDeque<char>>, Vec<i32>) {
    let mut instruction: bool = false;

    let mut cols: Vec<VecDeque<char>> = vec![];
    let mut instructions_vec: Vec<i32> = vec![];

    let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");
    let lines = contents.lines();
    for l in lines {
        if l == "" {
            instruction = true;
            continue;
        }
        for (idx, c) in l.chars().enumerate() {
            if c.is_alphanumeric() && !instruction {
                let col_idx = (idx - 1) / 4;

                // If the index is greater than the length, we need to add columns up to that index.
                if col_idx >= cols.len() {
                    for _ in 0..(col_idx - cols.len() + 1) {
                        cols.push(VecDeque::new());
                    }
                }
                cols[col_idx].push_back(c);
            }
        }

        let splitted = l.split_whitespace().collect::<Vec<&str>>();
        for s in splitted {
            if s.parse::<i32>().is_ok() && instruction {
                instructions_vec.push(s.parse::<i32>().unwrap());
            }
        }
    }

    // Remove the last element of all of the deques.
    for col in cols.iter_mut() {
        col.pop_back();
    }

    (cols, instructions_vec)
}

fn part1(mut array: Vec<VecDeque<char>>, instructions: Vec<i32>) -> Vec<char> {
    // println!("Array: {:?}", array);
    // println!("Instructions: {:?}", instructions);
    for i in instructions.chunks(3) {
        let (num, from, to) = (i[0], i[1] - 1, i[2] - 1);
        for _ in 0..num {
            // Pop the top element from the from deque and push it to the to deque on the front.
            let c = array[from as usize].pop_front().unwrap();
            array[to as usize].push_front(c);
        }
    }
    get_top_deques(&array)
}

fn part2(mut array: Vec<VecDeque<char>>, instructions: Vec<i32>) -> Vec<char> {
    // println!("Array: {:?}", array);
    // println!("Instructions: {:?}", instructions);
    for i in instructions.chunks(3) {
        let (num, from, to) = (i[0], i[1] - 1, i[2] - 1);
        // Pop num elements from the front of the from deque and push them to the front of the to deque.
        let mut temp: VecDeque<char> = VecDeque::new();
        for _ in 0..num {
            let c = array[from as usize].pop_front().unwrap();
            temp.push_back(c);
        }
        for _ in 0..num {
            let c = temp.pop_back().unwrap();
            array[to as usize].push_front(c);
        }
    }
    get_top_deques(&array)
}

fn get_top_deques(array: &Vec<VecDeque<char>>) -> Vec<char> {
    let mut result = vec![];
    for col in array {
        let c = col.front().unwrap();
        result.push(*c);
    }
    result
}

fn main() {
    let (array, instructions) = parse_file("input.txt");
    println!("{:?}", part1(array.clone(), instructions.clone()));
    println!("{:?}", part2(array.clone(), instructions.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let (array, instructions) = parse_file("sample.txt");
        let result = part1(array, instructions);
        assert_eq!(result, vec!['C', 'M', 'Z']);
    }

    #[test]
    fn test_sample_part2() {
        let (array, instructions) = parse_file("sample.txt");
        let result = part2(array, instructions);
        assert_eq!(result, vec!['M', 'C', 'D']);
    }
}
