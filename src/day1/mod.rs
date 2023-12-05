use aho_corasick::{AhoCorasick, Match};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_one(input: &Vec<String>) -> i32 {
    let mut calibrations: Vec<i32> = Vec::new();

    for line in input.iter() {
        let mut cal = String::new();

        for (char_index, character) in line.chars().enumerate() {
            if cal.len() == 2 {
                if character.is_numeric() {
                    cal.replace_range(1..2, &character.to_string());
                }
            } else if character.is_numeric() {
                cal = cal + &character.to_string();
            }

            if (cal.len() == 1) && (char_index + 1 == line.len()) {
                let temp = cal.clone();
                cal = cal + &temp;
            }
        }
        calibrations.push(cal.parse::<i32>().unwrap_or(0));
    }
    calibrations.iter().sum()
}

fn part_two(input: &Vec<String>) -> i32 {
    let patterns = &[
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();
    let mut total = 0;

    for line in input.iter() {
        let matches: Vec<Match> = ac.find_overlapping_iter(line).collect::<Vec<_>>();
        let first = matches.iter().nth(0).unwrap().pattern().as_i32() / 2;
        let last = matches.iter().last().unwrap().pattern().as_i32() / 2;
        total += 10 * (first + 1) + (last + 1);
    }
    total
}

pub fn run(file: &str) {
    let mut filepath = env::current_dir().unwrap();
    filepath.push(file);
    let lines = read_lines(filepath.to_str().unwrap()).expect("failed to read input");

    let input = lines
        .map(|line| line.expect("could not parse line"))
        .collect::<Vec<String>>();

    let result_one = part_one(&input);
    println!("Day 1 Part 1 Result: {}", result_one);

    let result_two = part_two(&input);
    println!("Day 1 Part 2 Result: {}", result_two);
}
