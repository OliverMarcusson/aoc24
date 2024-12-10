use std::{fs};
use regex::Regex;

fn main() {
    let input: String = fs::read_to_string("src/input.txt")
        .expect("Failed to read input.")
        .replace("\n", "");
    let mul_pattern = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let num_pattern = Regex::new(r"[0-9]+,[0-9]+").unwrap();
    let mut mul_matches: Vec<String> = Vec::new();
    let mut num_matches: Vec<String> = Vec::new();

    for cap in mul_pattern.captures_iter(&input) {
        mul_matches.push(cap[0].to_owned());
    }

    let mut mul_str = String::new(); 
    for val in mul_matches {
        mul_str += &val;
    }

    for cap in num_pattern.captures_iter(&mul_str) {
        num_matches.push(cap[0].to_owned());
    }

    let mut sum: u32 = 0;
    for nums in num_matches {
        let split: Vec<u32> = nums.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
        sum += split[0] * split[1]
    }
    println!("{}", sum);
}
