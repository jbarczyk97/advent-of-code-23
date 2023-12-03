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

    let max_red: usize = 12;
    let max_green: usize = 13;
    let max_blue: usize = 14;

    let sum: usize = input
        .lines()
        .map(|line| {
            let id = find_id(line);
            let (blue, _) = find_max_sum(line, "blue");
            let (red, _) = find_max_sum(line, "red");
            let (green, _) = find_max_sum(line, "green");

            return (id, red, green, blue);
        })
        .map(|(_, r,g,b)| r * g * b )
        .sum();

    println!("{}", sum);
}

fn find_max_sum(text: &str, color: &str) -> (usize, usize) {
    let pattern = format!("(\\d+) {}", color);
    let re = Regex::new(&pattern).unwrap();

    let max_value = re.captures_iter(text)
        .filter_map(|cap| cap[1].parse::<usize>().ok())
        .max()
        .unwrap_or(0);

    let sum_value = re.captures_iter(text)
        .filter_map(|cap| cap[1].parse::<usize>().ok())
        .sum();

    (max_value, sum_value)
}

fn find_id(line: &str) -> usize {
    if let Some(idx) = line.find(":") {
        line[5..idx].parse::<usize>().unwrap_or(0)
    } else {
        0
    }
}
