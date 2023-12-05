use std::fs;
use std::str::Lines;
use regex::Regex;

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading the input file.");
            return;
        }
    };

    let mut lines = input.lines();

    let seeds_line = lines.next().unwrap();
    let seeds = get_numbers(&seeds_line);
    lines.next();

    let mut mappings : Vec<(String, String, Vec<(usize, usize, usize)>)> = Vec::new();
    
    while let Some((from, to, numbers)) = get_mapping(&mut lines) {
        mappings.push((from, to, numbers));
    }

    let mut locations : Vec<usize> = Vec::new();
    for seed in seeds {
        let mut current : usize = seed;
        for (_, _, numbers) in &mappings {
            current = find_current(&numbers, current);

        }

        locations.push(current);
    }
    
    println!("{}", locations.iter().min().unwrap_or(&0));
}

fn find_current(numbers : &Vec<(usize, usize, usize)>, current : usize) -> usize {
    for &(source_start, destination_end, range) in numbers {
        if current >= destination_end && current < destination_end + range {
            return source_start + current - destination_end;
        }
    }

    current
}

fn get_mapping(lines : &mut Lines) -> Option<(String, String, Vec<(usize, usize, usize)>)> {
    let map_line = lines.next().unwrap_or("");

    if map_line == "" {
        return None
    }

    let (from, to) = get_map(map_line);
    let mut numbers: Vec<(usize, usize, usize)> = Vec::new();

    for (_, line) in lines.enumerate() {
        if line.trim().is_empty() {
            break; // Stop processing on an empty line
        }

        let current = get_numbers(line);
        numbers.push((current[0], current[1], current[2]));
    }

    Some((from, to, numbers))
}

fn get_numbers(line : &str) -> Vec<usize> {
    let re = Regex::new(r"\s*(\d+)\s*").unwrap();

    return re.captures_iter(line)
        .filter_map(|cap| cap[1].parse::<usize>().ok())
        .collect()
}

fn get_map(line: &str) -> (String, String) {
    let re = Regex::new(r"(\w+)-to-(\w+)").unwrap();

    let groups = re.captures_iter(line).next().unwrap();
    return (groups[1].trim().to_string(), groups[2].trim().to_string())
}