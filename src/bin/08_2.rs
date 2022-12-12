use std::io;

fn get_scenic_score(i: usize, j: usize, map: &Vec<Vec<u8>>) -> u32 {
    let row = map[i].clone();
    let col = map.iter().map(|row| row[j]).collect::<Vec<u8>>();
    let val = map[i][j];

    let mut up_score = 0;
    let mut down_score = 0;
    let mut left_score = 0;
    let mut right_score = 0;

    for &tree in col[..i].iter().rev() {
        up_score += 1;
        if tree >= val {
            break;
        }
    }
    if up_score == 0 { return 0 }

    for &tree in col[i+1..].iter() {
        down_score += 1;
        if tree >= val {
            break;
        }
    }
    if down_score == 0 { return 0 }

    for &tree in row[..j].iter().rev() {
        left_score += 1;
        if tree >= val {
            break;
        }
    }
    if left_score == 0 { return 0 }

    for &tree in row[j+1..].iter() {
        right_score += 1;
        if tree >= val {
            break;
        }
    }
    if right_score == 0 { return 0 }

    up_score * down_score * left_score * right_score
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut map: Vec<Vec<u8>> = Vec::new();

    for line in stdin.lines() {
        map.push(line?.trim().chars().map(|x| x.to_digit(10).unwrap() as u8).collect::<Vec<u8>>());
    }

    let mut max_score: u32 = 0;

    let rows = map.len();
    let cols = map[0].len();

    for i in 1..rows-1 {
        for j in 1..cols-1 {
            max_score = max_score.max(get_scenic_score(i, j, &map));
        }
    }

    println!("{}", max_score);

    Ok(())
}
