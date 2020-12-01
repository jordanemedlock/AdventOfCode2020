// sort list
// go from both ends
// i = 0, j = n-1
// add list[i] and list[j] 
// if > 2020: j-- 
// if < 2020: i++

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut input_vec: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("/home/jordanemedlock/Projects/AdventOfCode/day_1/input.txt") {
        for line in lines {
            if let Ok(line_str) = line {
                if let Ok(n) = line_str.parse::<u32>() {
                    input_vec.push(n);
                }
            } else {
                println!("Failed to read file!");
                break;
            }
        }
    }
    input_vec.sort();
    if let Ok((i, j, k)) = find_three(&input_vec, 2020) {
        let x = input_vec[i];
        let y = input_vec[j];
        let z = input_vec[k];
        println!("Heres your answer {}", x * y * z);

    }
}


fn find_two(v: &Vec<u32>, n: u32) -> Result<(usize, usize), &'static str> {
    let mut i = 0;
    let mut j = v.len()-1;
    return loop {
        let sum = v[i] + v[j];
        println!("sum: {}, i: {}, j: {}", sum, i, j);
        if sum == n {
            break Ok((i, j));
        } else if sum > n {
            j -= 1;
        } else {
            i += 1;
        }
        if i == j {
            break Err("Nothing in list equals desired result");
        }
    }
}

fn find_three(v: &Vec<u32>, n: u32) -> Result<(usize, usize, usize), &'static str> {
    for (k, x) in v.iter().enumerate() {
        if let Ok((i, j)) = find_two(v, n - x) {
            if i != j && i != k && j !=k {
                return Ok((i, j, k));
            }
        }
    }
    return Err("Nothing in list equals desired result");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}