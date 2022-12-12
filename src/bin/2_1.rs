use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut score: u64 = 0;
    let scores: HashMap<(char, char), u64> = [
        (('A', 'X'), 4),
        (('A', 'Y'), 8),
        (('A', 'Z'), 3),
        (('B', 'X'), 1),
        (('B', 'Y'), 5),
        (('B', 'Z'), 9),
        (('C', 'X'), 7),
        (('C', 'Y'), 2),
        (('C', 'Z'), 6),
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
