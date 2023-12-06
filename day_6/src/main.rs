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

    let mut lines = input.lines();
    let times = get_numbers_2(lines.next().unwrap());
    let distances = get_numbers_2(lines.next().unwrap());

    let mut result = 1;
    for (idx, time) in times.iter().enumerate() {
        let distance = distances[idx];
        match find_range_of_quadratic_function(1, -time, distance) {
            Some((start, end)) => {
                let diff = end - start + 1;
                result *= diff;
            },
            None => {}
        }
    } 

    println!("Result: {}", result);
}

fn get_numbers(line : &str) -> Vec<i64> {
    let re = Regex::new(r"\s*(\d+)\s*").unwrap();

    return re.captures_iter(line)
        .filter_map(|cap| cap[1].parse::<i64>().ok())
        .collect()
}

fn get_numbers_2(line : &str) -> Vec<i64> {
    let text = line.replace(" ", "");
    let re = Regex::new(r"(\d+)").unwrap();

    return re.captures_iter(&text)
        .filter_map(|cap| cap[1].parse::<i64>().ok())
        .collect()
}

// a x^2 + b x + c
fn find_range_of_quadratic_function(a : i64, b: i64, c: i64) -> Option<(i64, i64)> {
    let delta = b * b - 4 * a * c;

    if delta < 0  {
        return None;
    }


    if delta == 0 {
        let single_root : f64 = (-(b as f64))  / (2.0 * a as f64);
        if single_root.fract() == 0.0 {
            return Some((single_root as i64, single_root as i64));
        }

        return None;
    }

    let root_a : f64 = ((-(b as f64)) - (delta as f64).sqrt()) / (2.0 * a as f64);
    let root_b : f64 = ((-(b as f64)) + (delta as f64).sqrt()) / (2.0 * a as f64);

    return Some(((root_a + 1.0).floor() as i64, (root_b - 1.0).ceil() as i64));
}