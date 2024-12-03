use std::fs;

struct Day2 {
    input: Vec<String>,
}

impl Day2 {
    fn new(path: &str) -> Self {
        let input: Vec<String> = fs::read_to_string(path)
            .expect("Failed to read input.")
            .split("\n")
            .map(|x| x.to_owned())
            .collect();

        Self { input: input }
    }

    fn part_one(&self) -> u32 {
        let mut safe = 0;
        let mut notsafe = false;
        for report in &self.input {
            let mut increasing = false;
            let mut decreasing = false;
            let values: Vec<u32> = report
                .split(" ")
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            // For every value in the report
            for i in 0..values.len() - 1 {
                notsafe = false;
                if values[i] < values[i + 1] {
                    increasing = true;
                }

                if values[i] > values[i + 1] {
                    decreasing = true;
                }

                if increasing && decreasing {
                    notsafe = true;
                    break;
                }

                let diff = values[i].abs_diff(values[i + 1]);
                if diff > 3 || diff == 0 {
                    notsafe = true;
                    break;
                }
            }
            if !notsafe {
                safe += 1;
            }
        }
        safe
    }
}

fn main() {
    let day2 = Day2::new("src/input.txt");
    println!("Safe Reports: {}", day2.part_one())
}
