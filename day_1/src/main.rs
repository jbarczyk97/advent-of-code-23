use std::fs;

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading the input file.");
            return;
        }
    };

    let sum: usize = input
    .lines()
    .map(|line| {
        let first_digit = find_first_digit(line);
        let last_digit = find_last_digit(line);

        match (first_digit, last_digit) {
            (Some(first), Some(last)) => {
                first * 10 + last
            }
            _ => 0,
        }
    })
    .sum();

    println!("Sum is: {}", sum);
}

fn find_first_digit(line: &str) -> Option<usize> {
    for (index, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            return Some(c.to_digit(10).unwrap() as usize);
        }

        if let Some(text_digit) = try_text_digit(line, index) {
            return Some(text_digit);
        }
    }

    None
}

fn find_last_digit(line: &str) -> Option<usize> {
    let mut lastDigit : usize = 10 as usize;
    for (index, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            lastDigit = c.to_digit(10).unwrap() as usize;
        }

        if let Some(text_digit) = try_text_digit(line, index) {
            lastDigit = text_digit;
        }
    }

    if lastDigit == 10 {
        return None
    }

    Some(lastDigit)
}

fn try_text_digit(line: &str, index: usize) -> Option<usize> {
    let dictionary: std::collections::HashMap<&str, usize> = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
        .iter()
        .cloned()
        .collect();

    for (key, &value) in &dictionary {
        let trial = &line[index..].chars().take(key.len()).collect::<String>();
        if trial == *key {
            return Some(value);
        }
    }

    None
}