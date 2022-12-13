use std::{io, collections::HashSet};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut tail_positions: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    let mut pos: Vec<(i32, i32)> = vec![(0, 0); 10];

    for line in stdin.lines() {
        let line = line?;
        let mut split = line.split(' ');
        let direction = split.next().unwrap().trim();
        let amount = split.next().unwrap().trim().parse::<usize>().unwrap();

        for _ in 0..amount {
            match direction {
                "R" => pos[0] = (pos[0].0 + 1, pos[0].1),
                "L" => pos[0] = (pos[0].0 - 1, pos[0].1),
                "U" => pos[0] = (pos[0].0, pos[0].1 + 1),
                "D" => pos[0] = (pos[0].0, pos[0].1 - 1),
                _ => panic!()
            }

            for i in 1..10 {
                if pos[i-1].0.abs_diff(pos[i].0) > 1 || pos[i-1].1.abs_diff(pos[i].1) > 1 {
                    if pos[i-1].0 == pos[i].0 || pos[i-1].1 == pos[i].1 { // same row/col
                        pos[i] = *[(pos[i].0+1, pos[i].1), (pos[i].0-1, pos[i].1), (pos[i].0, pos[i].1+1), (pos[i].0, pos[i].1-1)].iter()
                            .filter(|p| pos[i-1].0.abs_diff(p.0) <= 1 && pos[i-1].1.abs_diff(p.1) <= 1).last().unwrap();
                    } else { // diagonal movement
                        pos[i] = *[(pos[i].0+1, pos[i].1+1), (pos[i].0-1, pos[i].1+1), (pos[i].0+1, pos[i].1-1), (pos[i].0-1, pos[i].1-1)].iter()
                            .filter(|p| pos[i-1].0.abs_diff(p.0) <= 1 && pos[i-1].1.abs_diff(p.1) <= 1).last().unwrap();
                    }
                }
            }
            tail_positions.insert(*pos.last().unwrap());
        }
    }

    println!("{}", tail_positions.len());

    Ok(())
}
