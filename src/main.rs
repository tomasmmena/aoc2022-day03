use std::fs;
use std::env;
use std::io::{self, BufRead};

use std::collections::HashSet;


fn find_error(value: &str) -> usize {
    if value.len() == 0 {
        return 0
    }
    let (
        first_compartment, 
        second_compartment
    ) = value.trim().split_at(value.trim().len() / 2);
    let mut first_set: HashSet<char> = HashSet::new();
    for c in first_compartment.chars() {
        first_set.insert(c);
    }
    let mut second_set: HashSet<char> = HashSet::new();
    for c in second_compartment.chars() {
        second_set.insert(c);
    }
    let overlap = first_set.intersection(&second_set);

    let mut accumulator: usize = 0;

    let priorities = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for c in overlap.into_iter() {
        accumulator += priorities.chars().position(|p| p == *c).unwrap();
    }
    accumulator
}


fn main() {
    let path = env::args().nth(1).expect("No route was provided.");

    let data = io::BufReader::new(
        fs::File::open(path)
            .expect("Could not read file"))
        .lines()
        .map(|rucksack| {
            let value = rucksack.unwrap_or("".to_string());
            find_error(value.as_str())
        });

    println!("Sum of errors: {}", data.sum::<usize>());
}


#[test]
fn test_find_error(){
    assert!(find_error("aaBBCAAbbC") == 29);
}

#[test]
fn test_find_error_empty(){
    assert!(find_error("") == 0);
}