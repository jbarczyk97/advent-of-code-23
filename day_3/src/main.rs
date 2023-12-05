use std::fs;

// NOT WORKING FOR NOW :D 
fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading the input file.");
            return;
        }
    };

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum: usize = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        for (clm_idx, clm) in row.iter().enumerate() {
            if *clm == '*' {
                let numbers = get_numbers(&grid, row_idx, clm_idx);
                println!("{:?}", numbers);
                if numbers.len() == 2 {
                    sum += numbers[0] * numbers[1];
                }
            }
        }
   }

   println!("{}", sum);
}

fn get_numbers(grid : &Vec<Vec<char>>, row : usize, column : usize) -> Vec<usize> {
    let mut result : Vec<usize> = Vec::new();

    let start_idx = column as i32 - 1;
    let end_idx = column as i32+ 1;
    let idx : usize = 0;
    let start_number : usize = 0;
    let end_number : usize = 0;

    println!("Row {}. Column {}. Text: {:?}", row, column, grid[row]);

    for i in start_idx..=end_idx {
        if is_digit_in_place(&grid, row as i32 - 1, i) {
            //return true
        }
    }

    for i in start_idx..=end_idx {
        if is_digit_in_place(&grid, row as i32 + 1  as i32, i) {
            //return true
        }
    }

    if is_digit_in_place(&grid, row as i32 , column as i32 - 1) {
        for i in (0..=(column - 1)).rev() {
            if(i == 0 || !grid[row][i].is_digit(10)) {
                let text = grid[row][(i+1)..=(column - 1)].iter().collect::<String>();
                println!("{}", text);
                result.push(text.parse::<usize>().unwrap_or(0));
                break;
            }
        }
    }

    if is_digit_in_place(&grid, row  as i32, column as i32 + 1) {
        for i in ((column + 1)..=grid[row].len()).rev() {
            if(i == grid[row].len() || grid[row][i].is_digit(10)) {
                let text = grid[row][(column + 1)..=(i-1)].iter().collect::<String>();
                println!("{}", text);
                result.push(text.parse::<usize>().unwrap_or(0));
                break;
            }
        }
    }

    return result
}

fn is_digit_in_place(grid : &Vec<Vec<char>>, row : i32, index : i32) -> bool {
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

    return grid[row as usize][index as usize].is_digit(10)
}