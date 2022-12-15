use std::{io, collections::VecDeque};

fn neighbours(map: &Vec<Vec<u8>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = Vec::<(usize, usize)>::new();

    if pos.0 > 0 {
        res.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        res.push((pos.0, pos.1 - 1));
    }
    if pos.0 < map.len() - 1 {
        res.push((pos.0 + 1, pos.1));
    }
    if pos.1 < map[0].len() - 1 {
        res.push((pos.0, pos.1 + 1));
    }

    res.iter().filter(|el| map[el.0][el.1] <= map[pos.0][pos.1] + 1).map(|el| *el).collect()
}

fn reconstruct_path(came_from: Vec<Vec<Option<(usize, usize)>>>, target: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path = vec![target];
    let mut current = target;
    while let Some(cell) = came_from[current.0][current.1] {
        current = cell;
        path.insert(0, current);
    }
    path
}

fn pathfind(map: &Vec<Vec<u8>>, start: (usize, usize), target: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let rows = map.len();
    let cols = map[0].len();

    let mut came_from: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; cols]; rows];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    let mut open = VecDeque::<(usize, usize)>::new();
    open.push_back(start);

    while open.len() > 0 {
        let current = open.pop_front().unwrap();

        if current == target {
            return Some(reconstruct_path(came_from, target));
        }
        for neighbour in neighbours(&map, current).iter().filter(|el| !visited[el.0][el.1]) {
            came_from[neighbour.0][neighbour.1] = Some(current);
            
            if !open.contains(neighbour) {
                open.push_back(*neighbour);
            }
        }
            
        visited[current.0][current.1] = true;
    }

    None
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut map = Vec::<Vec<u8>>::new();
    let mut start: (usize, usize) = (0, 0);
    let mut target: (usize, usize) = (0, 0);

    for line in stdin.lines() {
        let line = line?;
        map.push(
            line.trim().chars().zip(0..line.trim().len()).map(|el| {
                match el.0 {
                    'S' => {
                        start = (map.len(), el.1);
                        0
                    },
                    'E' => {
                        target = (map.len(), el.1);
                        'z' as u8 - 'a' as u8
                    },
                    c => c as u8 - 'a' as u8
                }
            }).collect()
        )
    }

    println!("{}", pathfind(&map, start, target).unwrap().len() - 1);

    Ok(())
}
