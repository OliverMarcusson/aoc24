use std::fs;

fn part_one() -> (Vec<u32>, Vec<u32>) {
    let input: Vec<String> = fs::read_to_string("src/input.txt")
        .expect("Failed to read file.")
        .split("\n")
        .map(|x| x.to_owned())
        .collect();

    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for line in input {
        let split: Vec<u32> = line
            .split_whitespace()
            .map(|value| value.parse::<u32>().unwrap())
            .collect();

        list1.push(split[0]);
        list2.push(split[1]);
    }

    list1.sort();
    list2.sort();

    let mut distance_sum: u32 = 0;

    for i in 0..list1.len() {
        distance_sum += list1[i].abs_diff(list2[i])
    }

    println!("Distance sum: {distance_sum}");
    (list1, list2)
}

fn part_two(lists: (Vec<u32>, Vec<u32>)) {
    let list1 = lists.0;
    let list2 = lists.1;
    let mut similarity_score = 0;

    for value in list1 {
        let appearances = list2.iter().filter(|x| x == &&value).count() as u32;
        similarity_score += value * appearances;
    }
    println!("Similarity score: {similarity_score}");
}

fn main() {
    let lists = part_one();
    part_two(lists);
}
