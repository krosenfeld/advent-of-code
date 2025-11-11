use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("inputs/day02.txt")?;

    let rows: Vec<Vec<i64>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let mut counter = 0;
    for row in &rows {
        let row_length = row.len();
        let mut flag = false;
        let mut req = 'n';
        for i in 0..(row_length - 1) {
            // get element difference
            let diff = row[i + 1] - row[i];
            // check whether the difference is allowed (0 or |d| < 4)
            if diff == 0 {
                flag = true;
            } else if diff.abs() > 3 {
                flag = true;
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
                    flag = true;
                } else if (diff > 0) & (req == 'd') {
                    flag = true;
                }
            }
        }
        // Check flag, if true that not safe
        if !flag {
            counter = counter + 1;
        }
    }
    println!("Number of safe reports: {}", counter);
    Ok(())
}
