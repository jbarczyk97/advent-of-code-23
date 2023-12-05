use std::fs;
use std::str::Lines;
use regex::Regex;
use std::ops::Range;

fn main() {
    let start = std::time::Instant::now();

    let input = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading the input file.");
            return;
        }
    };

    let mut lines = input.lines();

    let seeds_line = lines.next().unwrap();
    let seeds_pairs = get_numbers(&seeds_line);

    let seed_ranges : Vec<Range<u64>>  = seeds_pairs
        .chunks(2)
        .map(|chunk| Range {
            start: chunk[0],
            end: chunk[0] + chunk[1],
        })
        .collect();

    lines.next();


    let mut mappings : Vec<(String, String, Vec<(u64, u64, u64)>)> = Vec::new();
    
    while let Some((from, to, numbers)) = get_mapping(&mut lines) {
        mappings.push((from, to, numbers));
    }

    let mut locations : Vec<u64> = Vec::new();


    let mut current : Vec<Range<u64>> = Vec::new();
    current.extend(seed_ranges);
    for (from, to, numbers) in &mappings {
        current = find_current(&numbers, current);
        current.sort_by_key(|range| range.start);
        current = merge_ranges(current);

        println!("From {} to {} finished. Count of current ranges: {}", from, to, current.len());
        println!("");
    }
    
    locations.push(current.iter().map(|r| r.start).min().unwrap());

    println!("Response: {}", locations.iter().min().unwrap());

    let elapsed = start.elapsed();
    let seconds = elapsed.as_secs();
    let millis = elapsed.subsec_millis();

    println!("Time: {:02}s {:03}ms", seconds, millis);
}

fn merge_ranges(current : Vec<Range<u64>>) -> Vec<Range<u64>> {
    let mut result = Vec::with_capacity(current.len());
    let mut current_range = current[0].clone();

    for range in current.into_iter().skip(1) {
        if current_range.end >= range.start {
            current_range.end = current_range.end.max(range.end);
        } else {
            result.push(current_range.clone());
            current_range = range;
        }
    }

    result.push(current_range);

    result
}

fn find_current(numbers : &Vec<(u64, u64, u64)>, current : Vec<Range<u64>>) -> Vec<Range<u64>> {
    let mut result : Vec<Range<u64>> = Vec::with_capacity(current.len() * numbers.len());
    let mut used_ranges : Vec<Range<u64>> = Vec::with_capacity(numbers.len());

    for &(source_start, destination_end, r) in numbers {
        let start = destination_end;
        let end = destination_end + r;

        for range in &current {
            if start > range.end || end < range.start  {
                continue;
            }

            let internal_start = if start > range.start { start } else { range.start};
            let internal_end =  if end < range.end { end } else {range.end};

            used_ranges.push(Range {
                start: internal_start,
                end: internal_end
            });

            result.push(Range {
                start: internal_start - start + source_start,
                end: internal_end - start + source_start
            });
        }
    }



    used_ranges.sort_by_key(|range| range.start);
    let not_used_ranges = add_not_used_ranges(current.clone(), used_ranges.clone());

    result.extend(not_used_ranges.clone());

    result
}

fn add_not_used_ranges(main: Vec<Range<u64>>, used_ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    if used_ranges.is_empty() {
        return main;
    }

    let mut result = Vec::with_capacity(main.len());
    for range in main {
        let mut start = range.start;
        for used_range in &used_ranges {
            if start < used_range.start && used_range.start <= range.end {
                result.push(start..used_range.start);
            }
            start = used_range.end.max(start);
    
            if start > range.end {
                break;
            }
        }

        if start < range.end {
            result.push(start..range.end);
        }
    }

    result
}

fn get_mapping(lines : &mut Lines) -> Option<(String, String, Vec<(u64, u64, u64)>)> {
    let map_line = lines.next().unwrap_or("");

    if map_line == "" {
        return None
    }

    let (from, to) = get_map(map_line);
    let mut numbers: Vec<(u64, u64, u64)> = Vec::new();

    for (_, line) in lines.enumerate() {
        if line.trim().is_empty() {
            break; // Stop processing on an empty line
        }

        let current = get_numbers(line);
        numbers.push((current[0], current[1], current[2]));
    }

    Some((from, to, numbers))
}

fn get_numbers(line : &str) -> Vec<u64> {
    let re = Regex::new(r"\s*(\d+)\s*").unwrap();

    return re.captures_iter(line)
        .filter_map(|cap| cap[1].parse::<u64>().ok())
        .collect()
}

fn get_map(line: &str) -> (String, String) {
    let re = Regex::new(r"(\w+)-to-(\w+)").unwrap();

    let groups = re.captures_iter(line).next().unwrap();
    return (groups[1].trim().to_string(), groups[2].trim().to_string())
}
