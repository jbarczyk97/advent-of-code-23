use std::fs;

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading the input file.");
            return;
        }
    };

    let mut current : Vec<String> = Vec::new();
    let mut result : usize = 0;

    for line in input.lines() {
        if line.len() == 0 {
            result += calculate_result(&current);
            current.clear();

            println!("Temporary result: {}", result);
            continue;
        }

        current.push(line.to_string());
    }

    result += calculate_result(&current);
    println!("Final result: {}", result);
}

fn calculate_result(matrix : &Vec<String>) -> usize {
    let rows: Vec<usize> = matrix.iter().map(|row| parse_binary_string(row)).collect();

    let columns: Vec<usize> = (0..matrix[0].len())
        .map(|col| {
            matrix.iter()
                .map(|row| row.chars().nth(col).unwrap())
                .collect::<String>()
        })
        .map(|col_str| parse_binary_string(&col_str))
        .collect();

    let result = find_mirror(rows);
    if result.is_some() {
        return result.unwrap() * 100
    }

    return find_mirror(columns).unwrap()
}

fn parse_binary_string(input: &str) -> usize {
    usize::from_str_radix(&input.replace('.', "0").replace('#', "1"), 2).unwrap_or(0)
}

fn find_mirror(input: Vec<usize>) -> Option<usize> {
    let n = input.len();
    for i in 0..(n - 1) {
        let (mut left, mut right) = (i, i + 1);
        let mut difference_found : bool = false; 
    
        while input[left] == input[right] || has_one_digit_difference(input[left], input[right]) {
            if difference_found && has_one_digit_difference(input[left], input[right]) {
                break;
            }

            if !difference_found && has_one_digit_difference(input[left], input[right]) {
                difference_found = true;
            }

            if difference_found && (left == 0 || right == n - 1) {
                return Some(i + 1)
            }

            if left == 0 || right == n - 1 {
                break;
            }

            left -= 1;
            right += 1;
        }
    }

    None
}

fn has_one_digit_difference(a: usize, b: usize) -> bool {
    let xor_result = a ^ b;
    let binary_string = format!("{:b}", xor_result);
    binary_string.chars().filter(|&c| c == '1').count() == 1
}