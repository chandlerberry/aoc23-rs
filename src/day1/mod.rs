use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
Function Signature:
- read_lines is decleard, with generic type parameter "P"
- Takes one argument of filename, type "P"

Return Type "io::Result<io::Lines<io::BufReader<File>>>":
- Result type is an enum used for error handling, can can either be Ok(T) or Err(E). In this case, "T" is "io::Lines<io::BufReader<File>>"

"Where":
- "where P: AsRef<Path>" is a constraint on generic "P"
- "AsRef<Path>" is a trait in the Rust std library
    - A type that implements "AsRef<Path>" can be converted to a reference to a "Path" object. This is usefule because it allows the function to accept any type that can be converted to a "Path" reference, such as &str  or a "PathBuf"

Function Body:
- "File::open(filename)?" attempts to open a specicified file, and uses the "?" operator to throw an error if one occurs in the "open" function
- "io::BufReader::new(file).lines()" creates a "BufReader" wrapped around the file, and the "lines" method returns an iterator over the lines in the file
*/
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
