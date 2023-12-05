use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading the input file.");
            return;
        }
    };
    
    let mut dic: HashMap<usize, usize> = HashMap::new();
    let _: usize = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            
            let mut line_split = line.split('|');
            let mut winning_numbers = find_numbers(line_split.next().unwrap().trim());
            winning_numbers.remove(0); // remove first number which is Card Index
            let my_numbers = find_numbers(line_split.next().unwrap().trim());

            let mut repeat = 1;
            let count = dic.get(&idx).copied().unwrap_or(0);
            repeat += count;
            let mut current = idx;
            for num in &my_numbers {
                if winning_numbers.contains(&num) {
                    current += 1;
                    let counter = dic.entry(current).or_insert(0);
                    *counter += repeat;
                }
            }

            let counter = dic.entry(idx).or_insert(0);
            *counter += 1;
            0
        })
        .sum();

    let result : usize = dic
        .iter()
        .filter(|&(key, _)| *key < input.lines().count())
        .map(|(_, &value)|  value)
        .sum();

    println!("{}", result);
}

fn find_numbers(line: &str) -> Vec<usize> {
    let re = Regex::new(r"\s*(\d+)\s*").unwrap();

    return re.captures_iter(line)
        .filter_map(|cap| cap[1].parse::<usize>().ok())
        .collect()
}