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

    let empty_rows : Vec<usize> = map
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| if row.iter().all(|clm| *clm == '.') { Some(idx) } else { None })
        .collect();

    let empty_columns: Vec<usize> = (0..map[0].len())
        .filter(|&col| map.iter().all(|row| row[col] == '.'))
        .collect();
    
    let mut galaxies : Vec<(usize, usize)> = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                galaxies.push((i,j));
            }
        }
    }

    let mut result = 0;
    for i in 0..galaxies.len() {
        for j in 0..galaxies.len() {
            if i == j {
                break;
            }

            let max_x = galaxies[i].0.max(galaxies[j].0);
            let max_y = galaxies[i].1.max(galaxies[j].1);

            let min_x = galaxies[i].0.min(galaxies[j].0);
            let min_y = galaxies[i].1.min(galaxies[j].1);

            let x_distance = max_x - min_x;
            let y_distance = max_y - min_y;

            let x_additional = empty_rows
                .iter()
                .enumerate()
                .filter_map(|(idx, row)| if *row > min_x && *row < max_x { Some(idx) } else { None })
                .count();

            let y_additional = empty_columns
                .iter()
                .enumerate()
                .filter_map(|(idx, clm)| if *clm > min_y && *clm < max_y { Some(idx) } else { None })
                .count();

            let distance : i64 = x_additional as i64 * 999999 + x_distance as i64 + y_additional as i64 * 999999  + y_distance as i64;

            result += distance;
        }
    }

    println!("Result is: {}", result);
}
