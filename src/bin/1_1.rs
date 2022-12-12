use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut calories = Vec::<u64>::new();
    let mut current_calories: u64 = 0;

    for line in stdin.lines() {
        let line = line?;
        if line.len() == 0 {
            calories.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<u64>().unwrap();
        }
    }

    calories.sort();

    println!("{}", calories.last().unwrap());

    Ok(())
}
