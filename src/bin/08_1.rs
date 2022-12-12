use std::io;

fn check_visibility(i: usize, j: usize, map: &Vec<Vec<u8>>) -> bool {
    if i == 0 || j == 0 || i == map.len() - 1 || j == map[0].len() - 1 { return true }
    let row = map[i].clone();
    let col = map.iter().map(|row| row[j]).collect::<Vec<u8>>();
    let up_max = *col[..i].iter().max().unwrap();
    let down_max = *col[i+1..].iter().max().unwrap();
    let left_max = *row[..j].iter().max().unwrap();
    let right_max = *row[j+1..].iter().max().unwrap();
    let val = map[i][j];
    val > up_max || val > down_max || val > left_max || val > right_max
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut map: Vec<Vec<u8>> = Vec::new();

    for line in stdin.lines() {
        map.push(line?.trim().chars().map(|x| x.to_digit(10).unwrap() as u8).collect::<Vec<u8>>());
    }

    let mut count = 0;
    
    let rows = map.len();
    let cols = map[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if check_visibility(i, j, &map) { count += 1 }
        }
    }

    println!("{}", count);

    Ok(())
}
