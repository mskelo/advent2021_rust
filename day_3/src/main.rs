use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input: Vec<String> = read_input();
    println!("Day 3");
    one(input);
}

/**
 * TODO: Shorten this
 */ 
fn one(input: Vec<String>) {
    let input_length = input.len();
    let line_length: usize = input[0].chars().count();
    let mut count = vec![0; line_length];

    for line in input {
        for (i, character) in line.chars().enumerate() {
            match character {
                '0' => count[i]+=1,
                 _  => continue,
            }
        }
    }
    let mut gammavec: Vec<char> = Vec::new();
    let mut epsilonvec: Vec<char> = Vec::new();
    for i in count {
        let g = if i > input_length/2 {'1'} else {'0'};
        let e = if i < input_length/2 {'1'} else {'0'};
        gammavec.push(g);
        epsilonvec.push(e);
    }

    let gamma_string: String = gammavec.into_iter().collect();
    let epsilon_string: String = epsilonvec.into_iter().collect();
    
    let gamma: i64 = i64::from_str_radix(gamma_string.as_str(), 2).unwrap();
    let epsilon: i64 = i64::from_str_radix(epsilon_string.as_str(), 2).unwrap();

    println!("Answer 1: {}", gamma*epsilon);
}

/**
 * Ol' Reliable. Removed the unnecessary enumerate().
 */ 
fn read_input() -> Vec<String> {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input: Vec<String> = Vec::new();
    
    for line in reader.lines() {
        input.push(line.unwrap());
    }
    input
}