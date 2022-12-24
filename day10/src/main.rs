use std::fs;
use std::fmt;

#[allow(dead_code)]
struct Register {
    name: String,
    value: i32,
}

impl Register {
    fn new(name: String) -> Register {
        Register{name, value: 1}
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

struct Instruction {
    op: String,
    arg: i32,
}

impl Instruction {
    fn new(op: String, arg: i32) -> Instruction {
        Instruction{op, arg}
    }
}

fn read_file(fp: &str) -> Vec<Instruction> {
    let mut instructions = vec![];

    let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");
    let lines = contents.lines();
    for line in lines {
        // The first four characters are the operation
        let op = line[0..4].to_string();
        if op == "noop" {
            instructions.push(Instruction::new(op, 0));
            continue
        }

        // The rest of the line is the argument
        let arg = line[5..].parse::<i32>().unwrap();

        instructions.push(Instruction::new(op, arg));
    }

    instructions
}

fn parse_instruction(i: &Instruction, register: &mut Register) {
    let op = &i.op;
    let arg = i.arg;

    match op.as_str() {
        "addx" => register.value += arg,
        "noop" => (),
        _ => panic!("Unknown operation: {}", op),
    }
}

fn check_clock(clock: i32, reg: &Register) -> Option<i32> {
    match clock {
        20 => {println!("Value at 20: {}", reg.get_value()); return Some(reg.get_value())},
        60 => {println!("Value at 60: {}", reg.get_value()); return Some(reg.get_value())},
        100 => {println!("Value at 100: {}", reg.get_value()); return Some(reg.get_value())},
        140 => {println!("Value at 140: {}", reg.get_value()); return Some(reg.get_value())},
        180 => {println!("Value at 180: {}", reg.get_value()); return Some(reg.get_value())},
        220 => {println!("Value at 220: {}", reg.get_value()); return Some(reg.get_value())},
        _ => None,
    }
}

fn part1(fp: &str) -> f32 {
    let mut clock = 1;
    let mut x_register = Register::new("x".to_string());

    let mut result = 0.0;

    let instructions = read_file(fp);
    for instruction in instructions {
        if instruction.op == "noop" {
            clock += 1;
            if let Some(n) = check_clock(clock, &x_register) {
                result += clock as f32 * n as f32;
            }
            continue

        } else if instruction.op == "addx" {
            clock += 1;
            if let Some(n) = check_clock(clock, &x_register) {
                result += clock as f32 * n as f32;
            }
            clock += 1;
            parse_instruction(&instruction, &mut x_register);
            if let Some(n) = check_clock(clock, &x_register) {
                result += clock as f32 * n as f32;
            }
            continue

        } else {
            panic!("Unknown operation: {}", instruction.op);
        }
    }

    result
}


// The pixel drawing is 6 rows of 40 characters. Let's make a struct to hold the data.
// For ease, we will just make it a 1D array of 240 characters.
struct Pixel {
    data: [char; 240], // 6 rows of 40 characters
}

impl Pixel {
    fn new() -> Pixel {
        Pixel{data: ['.'; 240]}
    }

    fn set_pixel(&mut self, i: usize) {
        self.data[i] = '#';
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..6 {
            for j in 0..40 {
                write!(f, "{}", self.data[i*40 + j])?;
            }
            writeln!(f)?;
        }
        Ok(())
}}

fn draw_pixel(p: &mut Pixel, reg: &Register, clk: i32) {
    // CLock is 1-indexed, but we want to use 0-indexing for the pixel array.
    let adj_clk = clk - 1;

    // Adjust for row in a 1D array
    let clk_mod = adj_clk % 40;
    let i = reg.get_value();
    if (clk_mod - i).abs() <= 1 {
        p.set_pixel(adj_clk as usize);
    } 
}

fn part2(fp: &str){
    let mut clock = 1;
    let mut y_register = Register::new("y".to_string());
    let mut pix = Pixel::new();
    
    let instructions = read_file(fp);
    for instruction in instructions {
        if instruction.op == "noop" {
                draw_pixel(&mut pix, &y_register, clock);
                clock += 1;
                continue

            } else if instruction.op == "addx" {
                draw_pixel(&mut pix, &y_register, clock);
                clock += 1;
                draw_pixel(&mut pix, &y_register, clock);
                parse_instruction(&instruction, &mut y_register);
                clock += 1;
                continue

            } else {
                panic!("Unknown operation: {}", instruction.op);
            }
    }
    println!("{}", pix);
}

fn main() {
    let p1 = part1("input.txt");
    println!("Part 1: {}", p1);

    part2("input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("sample.txt"), 13140.0);
    }
}
