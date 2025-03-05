use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;  // open the file
    let reader = io::BufReader::new(file);  

    let mut lines = reader.lines();  

    // Read the first two numbers
    let max_memory: i32 = lines.next().unwrap()?.trim().parse().unwrap_or(0);
    let number_of_processes: i32 = lines.next().unwrap()?.trim().parse().unwrap_or(0);

    println!("Max Memory: {}", max_memory);
    println!("Number of Processes: {}", number_of_processes);

    Ok(())
}
