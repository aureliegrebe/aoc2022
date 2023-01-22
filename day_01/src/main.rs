use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static FILE_PATH: &str = "./data/input";

fn main() {
    // initialize counters
    let mut current_sum: i64 = 0;
    let mut current_reindeer_number: i64 = 1;
    let mut max_reindeers: [i64; 3] = [0, 0, 0];
    let mut max_sums: [i64; 3] = [0, 0, 0];

    // load data
    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                // if line is blank:
                if ip.is_empty() {
                    //check if bigger than min
                    let min_max = max_sums.iter().min().unwrap();
                    if current_sum > *min_max {
                        let min_max_idx = max_sums
                            .iter()
                            .enumerate()
                            .min_by(|(_, a), (_, b)| a.cmp(b))
                            .map(|(idx, _)| idx)
                            .unwrap();
                        max_sums[min_max_idx] = current_sum;
                        max_reindeers[min_max_idx] = current_reindeer_number;
                    }
                    
                    current_reindeer_number += 1;
                    current_sum = 0;
                } else if let Ok(num) = ip.parse::<i64>() {
                    // if line is number: add to sum
                    current_sum += num;
                } else {
                    panic!("Invalid input file!")
                }
            }
        }
    }

    let top_three_sum: i64 = max_sums.iter().sum();
    let max_idx = max_sums
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(idx, _)| idx)
        .unwrap();
    println!("Reindeer number {} is the most loaded with {} calories", max_reindeers[max_idx], max_sums[max_idx]);
    println!("Reindeer numbers {}, {}, and {} are the most loaded with a total of {} calories", max_reindeers[0], max_reindeers[1], max_reindeers[2], top_three_sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file= File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
