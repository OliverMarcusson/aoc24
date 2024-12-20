use std::fs;

fn get_string(input: &Vec<String>, chars: &str) -> String {
    let indexes: Vec<String> = chars
        .split(",") // Splits coords
        // Makes every coord into a vec with two values.
        .map(|i| i.split(".").map(|j| j.parse::<usize>().unwrap()).collect())
        // Gets the character at the
        // coordinate.
        .map(|i: Vec<usize>| input[i[0]].chars().nth(i[1]).unwrap().to_string())
        .collect();

    return String::from(indexes.concat());
}

fn pattern(input: &Vec<String>, pat_type: &str, start_index: &str) -> String {
    let spl_start_index: Vec<usize> = start_index
        .split(".")
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    let i = spl_start_index[0];
    let j = spl_start_index[1];
    let not_enough_space_h = input[i].len() - j >= 4;
    let not_enough_space_v = i <= input[0].len() - 4;
    let not_enough_space_h1 = j >= 3;
    let xmas = i != 0 && i != input.len() - 1 && j != 0 && j != input[0].chars().count() - 1;

    match pat_type {
        "normal" => {
            if not_enough_space_h {
                let mut output = String::new();
                output += format!("{}.{},", i, j).as_str();
                output += format!("{}.{},", i, j + 1).as_str();
                output += format!("{}.{},", i, j + 2).as_str();
                output += format!("{}.{}", i, j + 3).as_str();
                return output;
            }
            return String::new();   
        },

        "vertical" => {
            if not_enough_space_v {
                let mut output = String::new();
                output += format!("{}.{},", i, j).as_str();
                output += format!("{}.{},", i + 1, j).as_str();
                output += format!("{}.{},", i + 2, j).as_str();
                output += format!("{}.{}", i + 3, j).as_str();
                return output;
            }
            return String::new();
        },

        "diagr" => {
            if not_enough_space_v && not_enough_space_h {
                let mut output = String::new();
                output += format!("{}.{},", i, j).as_str();
                output += format!("{}.{},", i + 1, j + 1).as_str();
                output += format!("{}.{},", i + 2, j + 2).as_str();
                output += format!("{}.{}", i + 3, j + 3).as_str();
                return output;
            }
            return String::new();
        }

        "diagl" => {
            if not_enough_space_v && not_enough_space_h1 {
                let mut output = String::new();
                output += format!("{}.{},", i, j).as_str();
                output += format!("{}.{},", i + 1, j - 1).as_str();
                output += format!("{}.{},", i + 2, j - 2).as_str();
                output += format!("{}.{}", i + 3, j - 3).as_str();
                return output;
            }
            return String::new();
        }

        "xmas" => {
            if xmas {
                let mut output = String::new();
                output += format!("{}.{},", i - 1, j - 1).as_str();
                output += format!("{}.{},", i, j).as_str();
                output += format!("{}.{}", i + 1, j + 1).as_str();
                output += "|";
                output += format!("{}.{},", i - 1, j + 1).as_str();
                output += format!("{}.{},", i, j).as_str();
                output += format!("{}.{}", i + 1, j - 1).as_str();
                return output;
            }
            return String::new();
        }

        _ => {
            return String::new();   
            }
    }
}

fn part_two() {
    let mut input: Vec<String> = fs::read_to_string("src/input.txt")
        .expect("Failed to read input.")
        .split("\n")
        .map(|line| line.to_string())
        .collect();
    input.pop();
    let words = ["MAS", "SAM"];
    let mut yes = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, _) in row.chars().enumerate() {
            let to_add: String = pattern(&input, "xmas", format!("{}.{}", i, j).as_str());
            if to_add.as_str() != "" {
                let mut to_add: Vec<String> = to_add.split("|").map(|coord| coord.to_string()).collect();
                to_add[0] = get_string(&input, to_add[0].as_str());
                to_add[1] = get_string(&input, to_add[1].as_str());
                if words.contains(&to_add[0].as_str()) && words.contains(&to_add[1].as_str()) {
                    yes += 1;
                }
            }
        }
    }
    println!("Part 2 Appearances: {yes}");
}

fn part_one() {
    let mut input: Vec<String> = fs::read_to_string("src/input.txt")
        .expect("Failed to read input.")
        .split("\n")
        .map(|line| line.to_string())
        .collect();
    
    input.pop();
    let words = ["XMAS", "SAMX"];
    let patterns = ["normal", "vertical", "diagr", "diagl"];

    let mut yes = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, _) in row.chars().enumerate() {
            let mut to_check: Vec<String> = Vec::new();
            for pat in patterns {
                let to_add = pattern(&input, pat, format!("{}.{}", i, j).as_str());
                    if to_add.as_str() != "" {
                        to_check.push(to_add);  
                    }
            }
            for coord in to_check {
                if words.contains(&get_string(&input, coord.as_str()).as_str()) {
                    yes += 1;
                } 
            }
        }
    }
    println!("Part 1 Appearances: {yes}");
}

fn main() {
    part_one();
    part_two();
}
