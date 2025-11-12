use std::fs;

// Return True if not safe and False if safe
fn check_row(row: &Vec<i64>) -> bool {
    let row_length = row.len();
    let mut is_row_not_safe = false;
    let mut req = 'n';
    for i in 0..(row_length - 1) {
        // get element difference
        let diff = row[i + 1] - row[i];

        // check whether the difference is allowed (0 or |d| < 4)
        if diff == 0 {
            is_row_not_safe = true;
        } else if diff.abs() > 3 {
            is_row_not_safe = true;
        }
        // set increasing or decreasing requirement
        if i == 0 {
            if diff > 0 {
                req = 'i';
            } else {
                req = 'd';
            }
        } else {
            if (diff < 0) & (req == 'i') {
                is_row_not_safe = true;
            } else if (diff > 0) & (req == 'd') {
                is_row_not_safe = true;
            }
        }
    }
    is_row_not_safe
}

// Part A
fn part_a(rows: &Vec<Vec<i64>>) {
    let mut counter = 0;
    for row in rows {
        let is_row_not_safe = check_row(&row);
        // Check flag, if true then not safe
        if !is_row_not_safe {
            counter = counter + 1;
        }
    }
    println!("Number of safe reports: {}", counter); // Ans: 421
}

fn part_b(rows: &Vec<Vec<i64>>) {
    let mut counter = 0;
    for row in rows {
        // first check the row
        let mut is_row_not_safe = check_row(&row);
        // if not safe then we should apply the problem dampener
        if is_row_not_safe {
            let row_length = row.len();
            // Iterate over all levels to remove
            for i in 0..row_length {
                let mut new_row = Vec::new();
                // Fill in the new_row array, skipping i
                for j in 0..row_length {
                    if j != i {
                        new_row.push(row[j]);
                    }
                }
                // Now check this row to see if now safe, remember if check_row is true then not safe
                let is_new_row_not_safe = check_row(&new_row);
                if !is_new_row_not_safe {
                    is_row_not_safe = false;
                    break;
                }
            }
        }
        // Check flag to count if safe, if true then not safe
        if !is_row_not_safe {
            counter = counter + 1;
        }
    }
    println!("Number of safe rows: {}", counter)
}

fn main() -> std::io::Result<()> {
    // Read in the data
    let contents = fs::read_to_string("inputs/day02.txt")?;
    let rows: Vec<Vec<i64>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    // Run part A
    part_a(&rows);

    // Run part B
    part_b(&rows);

    Ok(())
}
