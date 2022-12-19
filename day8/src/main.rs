use std::fs;

#[derive(Debug)]
struct Array(Vec<Vec<i32>>);

#[derive(Debug)]
struct ScenicScores {
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
}

impl ScenicScores {
    fn new() -> ScenicScores {
        ScenicScores {
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        }
    }

    fn score(&self) -> i32 {
        self.top * self.bottom * self.left * self.right
    }
}

impl Array {
    fn shape(&self) -> (usize, usize) {
        let rows = self.0.len();
        let cols = self.0[0].len();
        (rows, cols)
    }

    fn determine_edges(&self) -> i32 {
    let mut edge_count = 0;
    let (rows, cols) = self.shape();
    for i in 0..rows {
        for j in 0..cols {
            if i == 0 || i == rows-1 || j == 0 || j == cols-1 {
                edge_count += 1;
            }
        }
    }

    edge_count
    }
}

fn read_file(fp: &str) -> Array {
    let mut array: Array = Array(vec![]);
    
    let contents = fs::read_to_string(fp)
        .expect("Something went wrong reading the file");

    for line in contents.lines() {
        let mut row: Vec<i32> = vec![];
        for c in line.chars() {
            let num = c.to_digit(10).unwrap();
            row.push(num as i32);
        }
        array.0.push(row);
    }
    array
}

fn check_rows(a: &Array, i: usize, j: usize) -> bool {
    let mut visible_top: bool = true;
    let mut visible_bottom: bool = true;
    let mut visible_left: bool = true;
    let mut visible_right: bool = true;

    let (rows, cols) = a.shape();
    for r in 0..rows {
        if r < i {
            if a.0[r][j] >= a.0[i][j] {
                visible_top = false;
            }

            for c in 0..cols {
                if c < j {
                    if a.0[i][c] >= a.0[i][j] {
                        visible_left = false;
                    }
                } else if c > j {
                    if a.0[i][c] >= a.0[i][j] {
                        visible_right = false;
                    }
                }
            }
        } else if r > i {
            if a.0[r][j] >= a.0[i][j] {
                visible_bottom = false;
            }

            for c in 0..cols {
                if c < j {
                    if a.0[i][c] >= a.0[i][j] {
                        visible_left = false;
                    }
                } else if c > j {
                    if a.0[i][c] >= a.0[i][j] {
                        visible_right = false;
                    }
                }
            }
        }
    }


    return visible_top || visible_bottom || visible_left || visible_right;
}


fn scenic(a: &Array, i: usize, j: usize) -> i32 {
    let mut scenic_score = ScenicScores::new();

    let (rows, cols) = a.shape();

    for r in (0..i).rev() {
        // println!("value T: {}", a.0[r][j]);
        scenic_score.top += 1;
        if a.0[r][j] >= a.0[i][j] {
            break;
        }
    }

    for r in i+1..rows {
        // println!("value B: {}", a.0[r][j]);
        scenic_score.bottom += 1;
        if a.0[r][j] >= a.0[i][j] {
            break;
        }
    }

    for c in (0..j).rev() {
        // println!("value L: {}", a.0[i][c]);
        scenic_score.left += 1;
        if a.0[i][c] >= a.0[i][j] {
            break;
        }
    }

    for c in j+1..cols {
        // println!("value R: {}", a.0[i][c]);
        scenic_score.right += 1;
        if a.0[i][c] >= a.0[i][j] {
            break;
        }
    }

    // println!("Values of scores: {:?}", scenic_score);

    scenic_score.score()
}

fn determine_most_scenic(a: &Array) -> i32 {
    let mut best_scenic_score = 0;

    let (rows, cols) = a.shape();
    for i in 1..rows-1 {
        for j in 1..cols-1 {
            if scenic(&a, i, j) > best_scenic_score {
                best_scenic_score = scenic(&a, i, j);
                // println!("Value: {}", a.0[i][j]);
                // println!("The value at ({}, {}) is visible", i, j);
                // visible_count += 1;
            }
        }
    }

    best_scenic_score
}

fn determine_if_visible(a: &Array) -> i32 {
    let mut visible_count = 0;
    // Remove the first and last rows. These are always visible.
    let (rows, cols) = a.shape();
    for i in 1..rows-1 {
        for j in 1..cols-1 {
            if check_rows(&a, i, j) {
                // println!("Value: {}", a.0[i][j]);
                // println!("The value at ({}, {}) is visible", i, j);
                visible_count += 1;
            }
        }
    }

    visible_count
}



fn main() {
    let array = read_file("input.txt");

    let result = determine_if_visible(&array);
    println!("The result is: {}", result);

    let edges = array.determine_edges();
    println!("The edges are: {}", edges);

    let total_visible = result + edges;
    println!("The total visible is: {}", total_visible);

    let most_scenic = determine_most_scenic(&array);

    println!("The most scenic is: {}", most_scenic);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edges() {
        let array = read_file("sample.txt");
        let result = array.determine_edges();
        assert_eq!(result, 16);
    }
    

    #[test]
    fn test_determine_if_visible() {
        let array = read_file("sample.txt");
        let result = determine_if_visible(&array);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_check_rows() {
        let array = read_file("sample.txt");
        let result = check_rows(&array, 2, 3);
        assert_eq!(result, true);
    }

    #[test]
    fn test_scenic_score() {
        let array = read_file("sample.txt");
        let result = scenic(&array, 1, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_scenic_score_best_individual() {
        let array = read_file("sample.txt");
        let result = scenic(&array, 3, 2);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_scenic_score_best() {
        let array = read_file("sample.txt");
        let result = determine_most_scenic(&array);
        assert_eq!(result, 8);
    }
}