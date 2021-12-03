use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    //TODO: Avoid cloning here. Skipped now due ownership fuckery.
    let input: Vec<String> = read_input();
    let inputcopy = input.clone().to_vec();
    
    println!("Day 2");
    black_box(input, 1);
    black_box(inputcopy, 2);   
}

fn black_box(input: Vec<String>, exercise: i8) {
    let mut x = 0;
    let mut aim = 0;
    let mut y = 0;

    for line in input {
        // Last char is always number 1-9
        let value: u32 = line.chars()
                             .nth(line.len()-1)
                             .unwrap()
                             .to_digit(10)
                             .unwrap();
        
        // Only 3 possible commands, each starting with different char.
        match line.chars().nth(0).unwrap() {
            'f' => {
                x+=value;
                if exercise == 2 { y+=aim*value; }  
            },
            'u' => aim-=value,
            'd' => aim+=value,
             _  => println!("Parse error"),
        }
    }
    // Excuse the inconsistent naming, but it saves lots of lines.
    if exercise == 1 {
        println!("The sub has moved {} units forward and {} units deep", x, aim);
        println!("Answer 1: {}", x*aim);
    } else {
        println!("The sub has moved {} units forward and {} units deep", x, y);
        println!("Answer 2: {}", x*y);
    }
}
/**
 * Still reads files, Praise Jebus. Now returns Vec<String>.
 **/
fn read_input() -> Vec<String> {
    let filename = "./input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input: Vec<String> = Vec::new();
    
    for line in reader.lines().enumerate() {
        input.push(line.1.unwrap());
    }
    input
}