use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
Macro lazy_static!
- lazy_static! initializes a static variable only once at runtime.
- required for hash maps to be static

Description of below code:
- define
*/
lazy_static! {
    static ref DIGITS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("one", "1");
        m.insert("two", "2");
        m.insert("three", "3");
        m.insert("four", "4");
        m.insert("five", "5");
        m.insert("six", "6");
        m.insert("seven", "7");
        m.insert("eight", "8");
        m.insert("nine", "9");
        m
    };
}

/*
Function Signature:
- read_lines is declared, with generic type parameter "P"
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

fn string_has_number(input_string: &str) -> bool {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    numbers.contains(&input_string)
}

fn convert_to_number(input_string: &str) -> &str {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for n in numbers {
        if n.contains(&input_string) {
            println!("{}", DIGITS.get(&n).unwrap());
            return DIGITS.get(&n).unwrap();
        }
    }
    panic!("convert_to_number(input_string: &str) entered invalid state")
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
    - create empty string "cal", where the calibration value gets assembled
    - match each char to an integer value of 1-9
        - if no matches found, go to next char
        - if match found, save in str that is size 2, with the first char in the str
        - if all characters have been iterated through and the length string is still only 1, add that string to itself
    - cast the string of two numbers combined together to an int, and add that value to the sum
- return the final sum after all the calibration values are
*/
fn part_one(input: &Vec<String>) -> i32 {
    let mut calibrations: Vec<i32> = (0..input.len()).map(|_| 0).collect();

    // consume the initial input
    for (line_index, line) in input.iter().enumerate() {
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
        calibrations[line_index] = cal.parse::<i32>().unwrap_or(0);
    }
    calibrations.iter().sum()
}

/*
Problem:
- Some digits of the calibration values are actually spelled out, not numbers
- Need to find the sum of all calibration values from each line, like above

Solution:
- read in file line by line
- declare an integer "sum", initialize at 0
- loop over each line in file
    - create empty string "cal", where the calibration value gets assembled
    - create empty string "assemble_number_string", where the calibration value gets assembled
    - loop over each character in string
        - assemble a string for each consecutive character, until it spells out a number, e.g. "two"
            - if the string forms the spelling of a number, add the numerical equivalent to "cal"
        - if the character is a number, add it to "cal"
        - if the length of string "cal is two", keep replacing the second item with the next numberical value until the line ends
    - cast the discoved calibration value as an int
    - populate the vector of ints that will be summed together at the same index of the current line in the loop with the calibration int, like so:
        let cal_int = cal.parse::<i32>().unwrap();
        calibrations[line_index] = cal_int;
- add all ints in calibrations vector together
- return the sum as the answer
*/
fn part_two(input: &Vec<String>) -> i32 {
    let mut calibrations: Vec<i32> = (0..input.len()).map(|_| 0).collect();

    for (line_index, line) in input.iter().enumerate() {
        let mut cal = String::new();
        let mut find_value = String::new();

        for (char_index, character) in line.chars().enumerate() {
            // if character is number, follow logic from part one
            if character.is_numeric() {
                find_value.clear();
                if cal.len() == 2 {
                    cal.replace_range(1..2, &character.to_string());
                }
                if cal.len() <= 1 {
                    cal = cal + &character.to_string();
                }
            // if character is not numeric, implement new logic to handle the creation of a
            } else {
                find_value = find_value + &character.to_string();
                if string_has_number(&find_value) {
                    let dumb_find_value = find_value.clone();
                    let digit = convert_to_number(&dumb_find_value);
                    find_value.clear();
                    if cal.len() == 2 {
                        cal.replace_range(1..2, &digit.to_string());
                    }
                    if cal.len() <= 1 {
                        cal = cal + &digit.to_string();
                    }
                }
            }

            if (cal.len() == 1) && (char_index + 1 == line.len()) {
                let temp = cal.clone();
                cal = cal + &temp;
            }
        }
        println!("{}: {}", line_index + 1, cal);
        calibrations[line_index] = cal.parse::<i32>().unwrap_or(0);
    }
    calibrations.iter().sum()
}

pub fn run() {
    let file = "/Users/chandler/homelab/aoc23-rs/src/day1/part2_example.txt";
    let input_file = Path::new(file);
    let lines = read_lines(input_file).expect("failed to read input");

    let input = lines
        // anonymous function handling any errors during parsing, collects lines as a vector of strings
        .map(|line| line.expect("could not parse line"))
        .collect::<Vec<String>>();

    let result_one = part_one(&input);
    println!("Day 1 Part 1 Result: {}", result_one);

    let result_two = part_two(&input);
    println!("Day 1 Part 2 Result: {}", result_two);
}
