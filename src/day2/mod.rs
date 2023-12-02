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

fn part_one(input: Vec<String>) -> i32 {
    input.len() as i32
}

pub fn run(file: &str) {
    let input_file = Path::new(file);
    let lines = read_lines(input_file).expect("failed to read input");

    let input = lines
        .map(|line| line.expect("could not parse line"))
        .collect::<Vec<String>>();

    let result = part_one(input);

    println!("Part 1 Result: {}", result);
}
