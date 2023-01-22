use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static FILE_PATH: &str = "./data/input";

fn main() {
    let mut total_score_part_1: i32 = 0;
    let mut total_score_part_2: i32 = 0;

    // iterate over data
    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(line_str) = line {
                total_score_part_1 += score_round_part_1(&line_str);
                total_score_part_2 += score_round_part_2(&line_str);
            }
        }
    }

    println!("Total score (part 1): {}", total_score_part_1);
    println!("Total score (part 2): {}", total_score_part_2);
}

/// Calculate the score for a single round of rock, paper, scissors
fn score_round_part_2(round_str: &str) -> i32 {
    let opponent_play: char = round_str.as_bytes()[0] as char;
    let my_play: char = round_str.as_bytes()[2] as char;

    match my_play {
        // player loses
        'X' => match opponent_play {
            // rock - scissor
            'A' => 3,
            // paper - rock
            'B' => 1,
            // scissors - paper
            'C' => 2,
            _ => panic!("Invalid input (opponent)"),
        },
        // player draws
        'Y' => match opponent_play {
            // rock - rock
            'A' => 4,
            // paper - paper
            'B' => 5,
            // scissors - scissors
            'C' => 6,
            _ => panic!("Invalid input (opponent)"),
        },
        // player wins
        'Z' => match opponent_play {
            // rock - paper
            'A' => 8,
            // paper - scissors
            'B' => 9,
            // scissors - rock
            'C' => 7,
            _ => panic!("Invalid input (opponent)"),
        },
        _ => panic!("Invalid input (player)"),
    }
}

/// Calculate the score for a single round of rock, paper, scissors
fn score_round_part_1(round_str: &str) -> i32 {
    let opponent_play: char = round_str.as_bytes()[0] as char;
    let my_play: char = round_str.as_bytes()[2] as char;

    match my_play {
        // rock - 1 point
        'X' => match opponent_play {
            // rock - draw
            'A' => 4,
            // paper - loss
            'B' => 1,
            // scissors - win
            'C' => 7,
            _ => panic!("Invalid input (opponent)"),
        },
        // paper - 2 points
        'Y' => match opponent_play {
            // rock - win
            'A' => 8,
            // paper - draw
            'B' => 5,
            // scissors - loss
            'C' => 2,
            _ => panic!("Invalid input (opponent)"),
        },
        //scissors - 3 points
        'Z' => match opponent_play {
            // rock - loss
            'A' => 3,
            // paper - win
            'B' => 9,
            // scissors - draw
            'C' => 6,
            _ => panic!("Invalid input (opponent)"),
        },
        _ => panic!("Invalid input (player)"),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
