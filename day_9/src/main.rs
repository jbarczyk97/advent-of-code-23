use std::fs;
use regex::Regex;

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading the input file.");
            return;
        }
    };

    let result : i64= input
        .lines()
        .map(|line| next_value(line))
        .sum();

    println!("Result: {}", result);
}

fn next_value(line: &str) -> i64 {
    let mut current = get_numbers(line);
    let mut new_line : Vec<i64> = current.clone();
    let mut pyramid : Vec<Vec<i64>> = Vec::new();
    pyramid.push(current.clone());

    while !new_line.iter().all(|val| *val == 0i64) {
        new_line = Vec::new();
        for (idx, number) in current.iter().enumerate() {
            if idx == current.len() - 1 {
                break;
            }
    
            new_line.push(current[idx + 1] - number);
        }

        current = new_line.clone();
        pyramid.push(new_line.clone());
    }

    let result = get_result_2(pyramid.clone());
    for row in &pyramid {
        println!("{:?}", row);
    }
    println!("Result intermidiate: {}", result);
    result
}

fn get_result_1(pyramid : Vec<Vec<i64>>) -> i64 {
    pyramid
        .iter()
        .map(|row| row[row.len() - 1])
        .sum()
}

fn get_result_2(pyramid : Vec<Vec<i64>>) -> i64 {
    pyramid
        .iter()
        .enumerate()
        .map(|(idx, row)| if idx % 2 == 0 {row[0]} else {row[0] * (-1)})
        .sum()
}

fn get_numbers(line : &str) -> Vec<i64> {
    let re = Regex::new(r"\s*(-?\d+)\s*").unwrap();

    return re.captures_iter(line)
        .filter_map(|cap| cap[1].parse::<i64>().ok())
        .collect()
}
