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
    let mut buf: [Option<HashSet<char>>; 3] = [None, None, None];
    let mut len = 0;

    for line in stdin.lines() {
        buf[len] = Some(HashSet::from_iter(line?.chars()));
        len += 1;
        if len < 3 {
            continue;
        }
        score += code(buf[0].clone().unwrap().intersection(
            &buf[1].clone().unwrap()
        ).map(|x| *x).collect::<HashSet<char>>().intersection(
            &buf[2].clone().unwrap()
        ).next().unwrap());
        len = 0;
    }

    println!("{}", score);

    Ok(())
}
