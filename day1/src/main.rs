use std::fs;

fn calculate_sublist_cals(v: Vec<Vec<f64>>) -> Vec<f64> {
    let mut sublist_totals: Vec<f64> = v.iter().map(|x| x.iter().sum::<f64>()).collect();
    sublist_totals.sort_by(|a, b| b.partial_cmp(a).unwrap());

    return sublist_totals;
}

fn read_contents(fp: &str) -> Vec<Vec<f64>> {
    let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");

    let mut calories: Vec<Vec<f64>> = vec![];

    // Read through contents. If a blank line is encountered, append the sublist to the list of lists.
    let mut sublist: Vec<f64> = vec![];
    for l in contents.lines() {
        if l == "" {
            calories.push(sublist);
            sublist = vec![];
        } else {
            sublist.push(l.parse::<f64>().unwrap());
        }
    }
    // One more push for the last sublist at the end of the file.
    calories.push(sublist);

    return calories;
}

fn main() {
    let calories = read_contents("input1.txt");
    let subtotals = calculate_sublist_cals(calories);

    println!("The most calories is: {}", subtotals[0]);

    let top_three = &subtotals[0..3];
    println!("Sum of top three is: {}", top_three.iter().sum::<f64>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sublist_cals() {
        let test_data = read_contents("sample.txt");
        let most_cals = calculate_sublist_cals(test_data);
        assert_eq!(most_cals, vec![24000.0, 11000.0, 10000.0, 6000.0, 4000.0]);
    }
}
