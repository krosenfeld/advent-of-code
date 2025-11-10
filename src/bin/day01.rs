use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Read in the data
fn read_two_columns(filename: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                col1.push(num1);
                col2.push(num2);
            }
        }
    }

    Ok((col1, col2))
}
fn main() {
    match read_two_columns("../../inputs/day01.txt") {
        Ok((mut col1, mut col2)) => {
            println!("Read in {}, {} numbers", col1.len(), col2.len());
            // Sort the lists
            col1.sort();
            col2.sort();
            // Take the absolute differences
            let mut total = 0;
            // Iterate over pairs
            for (num1, num2) in col1.iter().zip(col2.iter()) {
                total += (num1 - num2).abs()
            }
            println!("Total is {}", total)
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
