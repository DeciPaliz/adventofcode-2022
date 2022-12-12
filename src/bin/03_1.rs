use std::{io, collections::HashSet};

fn code(c: &char) -> u64 {
    if c.is_uppercase() {
        return *c as u64 - 38;
    }
    *c as u64 - 96
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut score: u64 = 0;

    for line in stdin.lines() {
        let line = line?;
        let (first, last) = line.split_at(line.len() / 2);
        score += code(HashSet::<char>::from_iter(first.chars()).intersection(&HashSet::<char>::from_iter(last.chars())).last().unwrap());
    }

    println!("{}", score);

    Ok(())
}
