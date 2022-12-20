use std::fmt;
use std::fs;

struct Grid {
    g: Vec<Vec<u32>>,
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: u32,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.g {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Grid {
        let mut grid = Grid{g: vec![vec![0; cols]; rows], x: rows-1, y: 0};
        let (rows, _) = grid.shape();
        grid.update_cell(rows-1, 0, 1);
        return grid
    }

    fn shape(&self) -> (usize, usize) {
        (self.g.len(), self.g[0].len())
    }

    fn erase_cell(&mut self, row: usize, col: usize) {
        self.g[row][col] = 0;
    }

    fn update_cell(&mut self, row: usize, col: usize, value: u32) {
        self.g[row][col] = value;
    }

    fn get_xy_coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn update_xy(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn count_ones(&self) {
        let mut count = 0;
        for row in &self.g {
            for cell in row {
                if *cell == 1 {
                    count += 1;
                }
            }
        }
        println!("Number of 1s: {}", count);
    }
}

fn parse_file(fp: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    
    let contents = fs::read_to_string(fp)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();
    for line in lines {
        let i = Instruction {
            direction: match line.chars().nth(0).unwrap() {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            },
            steps: line.chars().nth(2).unwrap().to_digit(10).unwrap() as u32,
        };
        instructions.push(i);
    }

    instructions
}

fn direction_update(g: &mut Grid, t_g: &mut Grid, steps: u32, x_mod: usize, y_mod: usize, neg: bool) {
    println!("Total steps: {}", steps);
    for c in 0..steps {
        println!("Step: {}", c);
        let (x, y) = g.get_xy_coords();
        g.erase_cell(x, y);    
        if neg {
            let (new_x, new_y) = (x-x_mod, y-y_mod);
            g.update_cell(new_x, new_y, 1);
            g.update_xy(new_x, new_y);
        } else {
            let (new_x, new_y) = (x+x_mod, y+y_mod);
            g.update_cell(new_x, new_y, 1);
            g.update_xy(new_x, new_y);
        }
        let adj_bool = determine_if_adjacent(g, t_g);
        // println!("h_g:\n{}", g);
        // println!("t_g:\n{}", t_g);
        // println!("adjacent: {}", adj_bool);
        if !adj_bool {
            tail_follow(g, t_g);
        }
    }
}

fn execute_instruction(g: &mut Grid, t_g: &mut Grid, i: Instruction) {
    match i.direction {
        Direction::Up => {
            direction_update(g, t_g, i.steps, 1, 0, true);
        },
        Direction::Down => {
            direction_update(g, t_g, i.steps, 1, 0, false);
        },
        Direction::Left => {
            direction_update(g, t_g, i.steps, 0, 1, true);
        },
        Direction::Right => {
            direction_update(g, t_g, i.steps, 0, 1, false);
        },
    }
}

fn determine_if_adjacent(h_grid: &Grid, t_grid: &Grid) -> bool {
    let (h_x, h_y) = h_grid.get_xy_coords();
    let (t_x, t_y) = t_grid.get_xy_coords();

    let x_diff = (h_x as i32 - t_x as i32).abs();
    let y_diff = (h_y as i32 - t_y as i32).abs();

    if x_diff <= 1 && y_diff <= 1 {
        return true
    } else {
        return false
    }
}

fn tail_follow(h_grid: &Grid, t_grid: &mut Grid) {
    let (h_x, h_y) = h_grid.get_xy_coords();
    let (t_x, t_y) = t_grid.get_xy_coords();
   
    let x_diff = h_x as i32 - t_x as i32;
    let y_diff = h_y as i32 - t_y as i32;

    // These are in the same row.
    if x_diff == 0 && y_diff != 0 {
        if y_diff > 0 {
            t_grid.update_cell(t_x, t_y+1, 1);
            t_grid.update_xy(t_x, t_y+1);
        } else {
            t_grid.update_cell(t_x, t_y-1, 1);
            t_grid.update_xy(t_x, t_y-1);
        }
    } 

    // These are in the same column.
    if y_diff == 0 && x_diff != 0 {
        if x_diff > 0 {
            t_grid.update_cell(t_x+1, t_y, 1);
            t_grid.update_xy(t_x+1, t_y);
        } else {
            t_grid.update_cell(t_x-1, t_y, 1);
            t_grid.update_xy(t_x-1, t_y);
        }
    }

    // Final case where not in same row or column.
    if x_diff != 0 && y_diff != 0 {
        if x_diff > 0 && y_diff > 0 {
            t_grid.update_cell(t_x+1, t_y+1, 1);
            t_grid.update_xy(t_x+1, t_y+1);
        } else if x_diff > 0 && y_diff < 0 {
            t_grid.update_cell(t_x+1, t_y-1, 1);
            t_grid.update_xy(t_x+1, t_y-1);
        } else if x_diff < 0 && y_diff > 0 {
            t_grid.update_cell(t_x-1, t_y+1, 1);
            t_grid.update_xy(t_x-1, t_y+1);
        } else if x_diff < 0 && y_diff < 0 {
            t_grid.update_cell(t_x-1, t_y-1, 1);
            t_grid.update_xy(t_x-1, t_y-1);
        }
    } 

    // println!("t_grid_new:\n{}", t_grid);
}

fn main() {
    let i = parse_file("input.txt");
    // println!("{:?}", i);

    let row = 500;
    let col = 500;

    let mut t_grid = Grid::new(row, col);
    let mut h_grid = Grid::new(row, col);

    // We can start the simulation at the center.
    t_grid.update_cell(row / 2, col / 2, 1);
    t_grid.update_xy(row / 2, col / 2);
    t_grid.erase_cell(row-1, 0);

    h_grid.update_cell(row / 2, col / 2, 1);
    h_grid.update_xy(row / 2, col / 2);
    h_grid.erase_cell(row-1, 0);

    // println!("{}", t_grid);
    // println!("{:?}", t_grid.get_xy_coords());
    // println!("{}", h_grid);

    for instruction in i {
        println!("Executing instruction: {:?}", instruction);
        execute_instruction(&mut h_grid, &mut t_grid, instruction);
    }

    t_grid.count_ones();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_instruction_right() {
        let mut g = Grid::new(5, 5);
        let mut t_g = Grid::new(5, 5);
        let i = Instruction {
            direction: Direction::Right,
            steps: 4,
        };
        execute_instruction(&mut g, &mut t_g, i);
        assert_eq!(g.get_xy_coords(), (4, 4));
    }

    #[test]
    fn test_execute_instruction_up() {
        let mut g2 = Grid::new(5, 5);
        let mut t_g = Grid::new(5, 5);
        let i = Instruction {
            direction: Direction::Up,
            steps: 4,
        };
        g2.update_cell(4, 0, 0);
        g2.update_cell(4, 4, 1);
        g2.update_xy(4, 4);

        execute_instruction(&mut g2, &mut t_g, i);
        assert_eq!(g2.get_xy_coords(), (0, 4));
    }

    #[test]
    fn test_execute_instruction_left() {
        let mut g2 = Grid::new(5, 5);
        let mut t_g = Grid::new(5, 5);
        let i = Instruction {
            direction: Direction::Left,
            steps: 3,
        };
        g2.update_cell(4, 0, 0);
        g2.update_cell(0, 4, 1);
        g2.update_xy(0, 4);

        execute_instruction(&mut g2, &mut t_g, i);
        assert_eq!(g2.get_xy_coords(), (0, 1));
    }

    #[test]
    fn test_execute_instruction_down() {
        let mut g2 = Grid::new(5, 5);
        let mut t_g = Grid::new(5, 5);
        let i = Instruction {
            direction: Direction::Down,
            steps: 1,
        };
        g2.update_cell(4, 0, 0);
        g2.update_cell(0, 1, 1);
        g2.update_xy(0, 1);

        execute_instruction(&mut g2, &mut t_g, i);
        assert_eq!(g2.get_xy_coords(), (1, 1));
    }

    #[test]
    fn test_if_adjacent_true() {
        let h_g = Grid::new(5, 5);
        let t_g = Grid::new(5, 5);

        assert_eq!(determine_if_adjacent(&h_g, &t_g), true);
    }

    #[test]
    fn test_if_adjacent_false() {
        let mut h_g = Grid::new(5, 5);

        h_g.update_cell(4, 0, 0);
        h_g.update_cell(0, 4, 1);
        h_g.update_xy(0, 4);

        let t_g = Grid::new(5, 5);

        assert_eq!(determine_if_adjacent(&h_g, &t_g), false);
    }

    #[test]
    fn test_tail_follow_adjacent() {
        let mut h_g = Grid::new(5, 5);
        let mut t_g = Grid::new(5, 5);

        h_g.update_cell(4, 0, 0);
        h_g.update_cell(2, 1, 1);
        h_g.update_xy(2, 1);

        println!("h_g:\n{}", h_g);
        println!("t_g:\n{}", t_g);

        tail_follow(&h_g, &mut t_g);

        assert_eq!(t_g.get_xy_coords(), (3, 1));
    }
}