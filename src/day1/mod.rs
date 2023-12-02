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
    - A type that implements "AsRef<Path>" can be converted to a reference to a "Path" object. This is useful because it allows the function to accept any type that can be converted to a "Path" reference, such as &str  or a "PathBuf"

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

/*
Problem:
- each line has a calibration value
- value can be found by combining the FIRST digit and the LAST digit (in that order)
    - If the string is "1abc2def3" the calibration value would be "13"
    - If the string is "ghi4jkl" the calibration value would be "44" (a list of 1 would mean index 0 is both first and last)
- Need to find the sum of all calibration values from each line

Solution:
- read in file line by line
- declare an integer "sum", initialized at 0
- loop over each line read in from the file
    - loop over each charater in string
    - match each char to an integer value of 0-9
        - if no matches found, go to next char
        - if match found, save in str that is size 2, with the first char in the str
    - cast the string of two numbers combined together to an int, and add that value to the sum
- return the final sum after all the calibration values are
*/
fn part_one(input: Vec<String>) -> i32 {
    input.len() as i32
}

// reads in file line by line, passes input as Vec<String> and
pub fn run(file: &str) {
    let input_file = Path::new(file);
    let lines = read_lines(input_file).expect("failed to read input");

    let input = lines
        .map(|line| line.expect("could not parse line"))
        .collect::<Vec<String>>();

    let result = part_one(input);

    println!("Part 1 Result: {}", result);
}
