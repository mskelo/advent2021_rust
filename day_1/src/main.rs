use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input: Vec<i32> = read_input();
    println!("Day 1");
    one(input.as_mut_slice());
    two(input.as_mut_slice());
}

fn read_input() -> Vec<i32> {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input: Vec<i32> = Vec::new();
    
    for line in reader.lines().enumerate() {
        input.push(line.1.unwrap().parse::<i32>().unwrap());
    }
    input
}

/**
 * How many measurements are larger than the previous measurement?
 */
fn one(input: &mut [i32]) {
    let mut count = 0;
    let mut temp = 0;

    for line in input {
        count = if line > &mut temp {count+1} else {count};
        temp = *line;
    }
    // First measurement didn't count, hence count-1.
    println!("Answer 1: {}", count-1); 
}

/**
 * Consider sums of a three-measurement sliding window. 
 * How many sums are larger than the previous sum?
 */
fn two(input: &mut [i32]) {
    let mut count = 0;
    let mut tempA = 0;
    let mut tempB = 0;

    for i in 2..input.len() {
        if i > 1 {
            tempA = tempB;
            tempB = input[i] + input[i-1] + input[i-2];
            count = if tempB > tempA {count+1} else {count};
        }
    }
    // First measurement didn't count, hence count-1.
    println!("Answer 2: {}", count-1); 
}
