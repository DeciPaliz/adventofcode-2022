use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut score: u64 = 0;
    let scores: HashMap<(char, char), u64> = [
        (('A', 'X'), 3),
        (('A', 'Y'), 4),
        (('A', 'Z'), 8),
        (('B', 'X'), 1),
        (('B', 'Y'), 5),
        (('B', 'Z'), 9),
        (('C', 'X'), 2),
        (('C', 'Y'), 6),
        (('C', 'Z'), 7),
    ].iter().cloned().collect();

    for line in stdin.lines() {
        let chars = line?.chars().collect::<Vec<char>>();
        let opponent_move = chars[0];
        let my_move = chars[2];

        score += scores.get(&(opponent_move, my_move)).unwrap();
    }

    println!("{}", score);

    Ok(())
}
