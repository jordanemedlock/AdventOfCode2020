// Parse file into struct
// apply rule
// profit

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("/home/jordanemedlock/Projects/AdventOfCode/day_2/input.txt") {
        for line in lines {
            if let Ok(line_str) = line {
                let v: Vec<&str> = line_str.split(' ').collect();
                if let [range, target, password] = v[..] {
                    if let [start_str, stop_str] = range.split('-').collect::<Vec<&str>>()[..] {

                        let start = start_str.parse::<u32>().unwrap();
                        let stop = stop_str.parse::<u32>().unwrap();
    
                        let target_char = target.chars().nth(0).unwrap();
                        
                        let valid = valid_part_2(start, stop, target_char, password);

                        println!("{} - {} {} {} => {}", start, stop, target_char, password, valid);

                        if valid {
                            sum += 1;
                        }
                    }
                }
            } else {
                println!("Failed to read file!");
                break;
            }
        }
    }

    println!("{} valid passwords", sum);
}

fn valid_password(start: u32, stop: u32, target: char, password: &str) -> bool {
    let mut sum = 0;
    for c in password.chars() {
        if c == target {
            sum += 1;
            if sum > stop {
                return false;
            }
        }
    }

    return sum >= start;
}

fn valid_part_2(start: u32, stop: u32, target: char, password: &str) -> bool {
    if let Some(first) = password.chars().nth((start-1) as usize) {
        if let Some(second) = password.chars().nth((stop-1) as usize) {
            return (first == target) ^ (second == target);
        }
    }
    return false;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}