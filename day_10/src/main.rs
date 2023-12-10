use std::fs;

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading the input file.");
            return;
        }
    };

    let map : Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut first_path = find_coordinates(&map, 'S');
    let mut second_path = first_path.clone();
    let mut result = 0;

    let mut last_first_path : (i32, i32) = first_path.clone();
    let mut last_second_path : (i32, i32) = first_path.clone();

    while first_path != second_path || result == 0 {
        result += 1;
        
        if is_move_possible(&map, (first_path.0, first_path.1), (first_path.0 + 1, first_path.1), last_first_path) {
            last_first_path = first_path.clone();
            first_path = (first_path.0 + 1, first_path.1);
        } else if is_move_possible(&map, (first_path.0, first_path.1), (first_path.0 - 1, first_path.1), last_first_path) {
            last_first_path = first_path.clone();
            first_path = (first_path.0 - 1, first_path.1);
        } else if is_move_possible(&map, (first_path.0, first_path.1), (first_path.0, first_path.1 + 1), last_first_path) {
            last_first_path = first_path.clone();
            first_path = (first_path.0, first_path.1 + 1);
        } else if is_move_possible(&map, (first_path.0, first_path.1), (first_path.0 , first_path.1 - 1), last_first_path) {
            last_first_path = first_path.clone();
            first_path = (first_path.0, first_path.1 - 1);
        }

        if first_path == second_path {
            result -= 1;
            break;
        }

        if is_move_possible(&map, (second_path.0, second_path.1), (second_path.0, second_path.1 - 1), last_second_path) {
            last_second_path = second_path.clone();
            second_path = (second_path.0 , second_path.1 - 1);
        } else if is_move_possible(&map, (second_path.0, second_path.1), (second_path.0, second_path.1 + 1), last_second_path) {
            last_second_path = second_path.clone();
            second_path = (second_path.0, second_path.1 + 1);
        } else if is_move_possible(&map, (second_path.0, second_path.1), (second_path.0 - 1, second_path.1), last_second_path) {
            last_second_path = second_path.clone();
            second_path = (second_path.0 - 1, second_path.1);
        } else if is_move_possible(&map, (second_path.0, second_path.1), (second_path.0 + 1 , second_path.1), last_second_path) {
            last_second_path = second_path.clone();
            second_path = (second_path.0 + 1, second_path.1);
        }
    }

    println!("Result: {}", result);
}

fn find_coordinates(map: &Vec<Vec<char>>, target: char) -> (i32, i32) {
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == target {
                return (row_idx as i32, col_idx as i32);
            }
        }
    }
    (0,0)
}

fn is_move_possible(map: &Vec<Vec<char>>, from: (i32, i32), to: (i32, i32), last_location: (i32, i32)) -> bool {
    if to == last_location {
        return false
    }

    let (from_x, from_y) = from;
    let (to_x, to_y) = to;

    if to_x < 0 || to_y < 0 {
        return false
    }

    if to_x >= map.len() as i32 {
        return false
    }

    if to_y >= map[to_x as usize].len() as i32 {
        return false
    }

    let direction_x = to_x - from_x;
    let direction_y = to_y - from_y;
    let last_pipe = map[from_x as usize][from_y as usize];
    let pipe = map[to_x as usize][to_y as usize];

    if direction_y == -1 && (pipe == '-' || pipe == 'L' || pipe == 'F')  && (last_pipe == '-' || last_pipe == 'J' || last_pipe == '7' || last_pipe == 'S') {
        return true
    }

    if direction_y == 1 && (pipe == '-' || pipe == 'J' || pipe == '7')  && (last_pipe == '-' || last_pipe == 'L' || last_pipe == 'F' || last_pipe == 'S') {
        return true
    }

    if direction_x == 1 && (pipe == '|' || pipe == 'J' || pipe == 'L')  && (last_pipe == '|' || last_pipe == 'F' || last_pipe == '7' || last_pipe == 'S') {
        return true
    }

    if direction_x == -1 && (pipe == '|' || pipe == '7' || pipe == 'F')  && (last_pipe == '|' || last_pipe == 'J' || last_pipe == 'L' || last_pipe == 'S') {
        return true
    }

    return false
}