use std::fs;

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading the input file.");
            return;
        }
    };

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start :usize = 0;
    let mut end :usize = 0;
    let mut sum : usize = 0;
    let mut start_set : bool = false;

    for (row_idx, row) in grid.iter().enumerate() {
        for (clm_idx, clm) in row.iter().enumerate() {
            if clm.is_digit(10) {
                end = clm_idx;
                if !start_set {
                    start = clm_idx;
                    start_set = true;
                }
            }
            else {
                if check_symbols(&grid, row_idx, start, end) {
                    let current = grid[row_idx][start..(end + 1)].iter().collect::<String>().parse::<usize>().unwrap_or(0);
                    sum += current;

                    println!("Sum: {}. Current: {}", sum, current);
                }

                start = 0;
                end = 0;
                start_set = false;
            }
        }

        if check_symbols(&grid, row_idx, start, end) {
            let current = grid[row_idx][start..(end + 1)].iter().collect::<String>().parse::<usize>().unwrap_or(0);
            sum += current;

            println!("Sum: {}. Current: {}", sum, current);
        }
   }

   println!("{}", sum);
}

fn check_symbols(grid : &Vec<Vec<char>>, row : usize, start : usize, end : usize) -> bool {
    if !grid[row][start].is_digit(10) {
        return false
    }

    let start_idx = start as i32 - 1;
    let end_idx = end as i32+ 1;
    for i in start_idx..=end_idx {
        if check_symbol(&grid, row as i32 - 1, i) {
            return true
        }
    }

    for i in start_idx..=end_idx {
        if check_symbol(&grid, row as i32 + 1  as i32, i) {
            return true
        }
    }

    if check_symbol(&grid, row as i32 , start as i32 - 1) {
        return true
    }

    if check_symbol(&grid, row  as i32, end as i32 + 1) {
        return true
    }

    return false
}

fn check_symbol(grid : &Vec<Vec<char>>, row : i32, index : i32) -> bool {
    if row < 0 {
        return false
    }

    if index < 0 {
        return false
    }
    
    if row as usize >= grid.len() {
        return false
    }

    if index as usize >= grid[0].len() {
        return false
    }

    return !grid[row as usize][index as usize].is_digit(10) && grid[row as usize][index as usize] != '.'
}