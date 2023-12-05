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
    let mut game_ids: Vec<i32> = Vec::new();

    for line in input.iter() {
        println!("{line}");
        game_ids.push(1);
    }

    game_ids.iter().sum()
}

// fn part_two(input: &Vec<String>) -> i32 {
//     0 as i32
// }

pub fn run(file: &str) {
    let mut filepath = env::current_dir().unwrap();
    filepath.push(file);
    let lines = read_lines(filepath.to_str().unwrap()).expect("failed to read input");

    let input = lines
        .map(|line| line.expect("could not parse line"))
        .collect::<Vec<String>>();

    let result_one = part_one(&input);
    println!("Day 2 Part 1 Result: {result_one}");
    // let result_two = part_two(&input);
    // println!("Day 2 Part 2 Result: {result_two}");
}
