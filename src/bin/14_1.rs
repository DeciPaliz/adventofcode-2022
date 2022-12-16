use std::io;

fn drop_sand(map: &mut Vec<Vec<bool>>, start: (usize, usize)) -> bool { let mut sand_pos = start;

    while sand_pos.0 < map[0].len() && sand_pos.1 < map.len() {
        if sand_pos.1 + 1 >= map.len() { return false; }
        
        if !map[sand_pos.1 + 1][sand_pos.0] {
            sand_pos = (sand_pos.0, sand_pos.1 + 1);
            continue;
        }

        if !map[sand_pos.1 + 1][sand_pos.0 - 1] {
            sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
            continue;
        }

        if !map[sand_pos.1 + 1][sand_pos.0 + 1] {
            sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
            continue;
        }

        map[sand_pos.1][sand_pos.0] = true;
        return true;
    }

    false
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut walls = Vec::<Vec<(usize, usize)>>::new();

    for line in stdin.lines() {
        let line = line?;

        if line.trim().len() == 0 { continue; }

        let mut wall = Vec::<(usize, usize)>::new();
        for part in line.split(" -> ") {
            let pos = part.trim().split(',').map(|el| el.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
            wall.push((pos[0], pos[1]));
        }
        walls.push(wall);
    }

    let width = walls.iter().flatten().map(|el| el.0).max().unwrap() + 2;
    let depth = walls.iter().flatten().map(|el| el.1).max().unwrap() + 2;

    let mut map = vec![vec![false; width]; depth];

    for seq in walls {
        for i in 1..seq.len() {
            if seq[i-1].0 == seq[i].0 {
                // vertical wall
                for j in seq[i-1].1.min(seq[i].1)..seq[i-1].1.max(seq[i].1) + 1 {
                    map[j][seq[i].0] = true;
                }
            } else {
                // horisontal wall
                for j in seq[i-1].0.min(seq[i].0)..seq[i-1].0.max(seq[i].0) + 1 {
                    map[seq[i].1][j] = true;
                }
            }
        }
    }

    let mut count = 0;

    while drop_sand(&mut map, (500, 0)) { count += 1 }

    println!("{}", count);

    Ok(())
}
