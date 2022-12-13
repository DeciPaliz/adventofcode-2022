use std::{io, collections::HashSet};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut tail_positions: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    let mut head_pos: (i32, i32) = (0, 0);
    let mut tail_pos: (i32, i32) = (0, 0);

    for line in stdin.lines() {
        let line = line?;
        let mut split = line.split(' ');
        let direction = split.next().unwrap().trim();
        let amount = split.next().unwrap().trim().parse::<usize>().unwrap();

        for _ in 0..amount {
            let prev_pos: (i32, i32) = head_pos;
            match direction {
                "R" => head_pos = (head_pos.0 + 1, head_pos.1),
                "L" => head_pos = (head_pos.0 - 1, head_pos.1),
                "U" => head_pos = (head_pos.0, head_pos.1 + 1),
                "D" => head_pos = (head_pos.0, head_pos.1 - 1),
                _ => panic!()
            }

            if head_pos.0.abs_diff(tail_pos.0) > 1 || head_pos.1.abs_diff(tail_pos.1) > 1 {
                tail_pos = prev_pos;
                tail_positions.insert(tail_pos);
            }
        }
    }

    println!("{}", tail_positions.len());

    Ok(())
}
