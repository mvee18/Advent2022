use std::collections::HashMap;
use std::fs;

fn read_file(fp: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");
    let mut rounds: Vec<Vec<char>> = vec![];
    for l in contents.lines() {
        let mut round: Vec<char> = vec![];
        for c in l.chars() {
            if c == ' ' {
                continue;
            }
            round.push(c);
        }
        rounds.push(round);
    }
    return rounds;
}

enum Outcome {
    Win,
    Loss,
    Tie,
}

fn calculate_winner(our_val: i32, their_val: i32) -> Outcome {
    let val = (3 + our_val - their_val) % 3;
    match val {
        0 => Outcome::Tie,
        1 => Outcome::Win,
        2 => Outcome::Loss,
        _ => panic!("Impossible value {}", val),
    }
}

fn part1(rounds: &Vec<Vec<char>>, values: HashMap<char, i32>) -> i32 {
    let mut round_scores: Vec<i32> = vec![];

    for round in rounds {
        let their_choice = &round[0];
        let our_choice = &round[1];

        let their_val = values.get(their_choice).unwrap();
        let our_val = values.get(our_choice).unwrap();

        let outcome = calculate_winner(*our_val, *their_val);

        match outcome {
            Outcome::Win => round_scores.push(our_val + 6),
            Outcome::Loss => round_scores.push(our_val + 0),
            Outcome::Tie => round_scores.push(our_val + 3),
        }
    }

    return round_scores.iter().sum();
}

fn calculate_choice(their_val: i32, wanted_val: i32) -> i32 {
    const VALUES: &'static [i32] = &[1, 2, 3];
    // Going to solve interatively because I am bad at modular arithmetic.
    let mut result: i32 = 0;
    for v in VALUES {
        let val = (3 + v - their_val) % 3;
        if val == wanted_val {
            result = *v;
        }
    }
    return result;
}

fn calculate_final_part2(our_choice: i32, wanted_val: i32) -> i32 {
    match wanted_val {
        0 => return our_choice + 3,
        1 => return our_choice + 6,
        2 => return our_choice + 0,
        _ => panic!("Impossible value {}", wanted_val),
    }
}

fn part2(rounds: &Vec<Vec<char>>, values: HashMap<char, i32>) -> i32 {
    let mut round_scores: Vec<i32> = vec![];

    for round in rounds {
        let their_choice = &round[0];
        let wanted_outcome = &round[1];

        let their_val = values.get(their_choice).unwrap();
        let wanted_val = values.get(wanted_outcome).unwrap();

        let our_choice = calculate_choice(*their_val, *wanted_val);

        round_scores.push(calculate_final_part2(our_choice, *wanted_val));
    }

    let result = round_scores.iter().sum();
    result
}

fn main() {
    let value_map: HashMap<char, i32> =
        HashMap::from([('A', 1), ('B', 2), ('C', 3), ('X', 1), ('Y', 2), ('Z', 3)]);

    println!("{}", part1(&read_file("input.txt"), value_map));

    let value_map_2: HashMap<char, i32> = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3),
        ('Z', 1), // We need to win
        ('X', 2), // We need to lose
        ('Y', 0), // We need to draw
    ]);

    println!("{}", part2(&read_file("input.txt"), value_map_2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let value_map: HashMap<char, i32> =
            HashMap::from([('A', 1), ('B', 2), ('C', 3), ('X', 1), ('Y', 2), ('Z', 3)]);
        assert_eq!(part1(&read_file("sample.txt"), value_map), 15);
    }

    #[test]
    fn test_sample_2() {
        let value_map: HashMap<char, i32> = HashMap::from([
            ('A', 1),
            ('B', 2),
            ('C', 3),
            ('Z', 1), // We need to win
            ('X', 2), // We need to lose
            ('Y', 0), // We need to draw
        ]);
        assert_eq!(part2(&read_file("sample.txt"), value_map), 12);
    }
}
