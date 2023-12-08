use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading the input file.");
            return;
        }
    };

    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next(); // skip empty line

    let mut directions = HashMap::<String, (String, String)>::new();

    for line in lines {
        let dir = read_line(line);
        directions.insert(dir.from, (dir.next_direction.left, dir.next_direction.right));
    }

    let mut current = "AAA";
    let mut result = 0;
    while current != "ZZZ"{
        for instruction in instructions.chars() {
            if current == "ZZZ" {
                break;
            }
    
            result += 1;
            let (left, right) = directions.get(current).unwrap();
            if instruction == 'L' {
                current = left;
            } else {
                current = right;
            }
        }
    }

    println!("Result: {}", result);
}

fn read_line(line : &str) -> Direction {
    let re = Regex::new(r"([\d\w]+) = \(([\d\w]+), ([\d\w]+)\)").unwrap();

    let dir = re.captures_iter(line)
        .map(|cap| Direction {
             from: cap[1].to_string(),
             next_direction: Next {
                left: cap[2].to_string(),
                right: cap[3].to_string()
             }
        })
        .next()
        .unwrap();

    dir
}

#[derive(Debug)]
struct Direction {
    from : String,
    next_direction : Next
}

#[derive(Debug)]
struct Next {
    left : String,
    right : String
}
