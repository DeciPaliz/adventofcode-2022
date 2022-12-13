use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut x = 1;
    let mut sum = 0;
    let mut cycle = 1;

    for line in stdin.lines() {
        let line = line?;
        let mut split = line.split(' ');
        let command = split.next().unwrap().trim();
        let amount: Option<i32> = if let Some(s) = split.next() { Some(s.trim().parse::<i32>().unwrap()) } else { None };

        if cycle >= 20 && (cycle - 20) % 40 == 0 {
            sum += x * cycle;
        }
        
        match command {
            "noop" => { cycle += 1 },
            "addx" => { 
                cycle += 1;
                if cycle >= 20 && cycle <= 220 && (cycle - 20) % 40 == 0 {
                    sum += x * cycle;
                }
                cycle += 1;
                x += amount.unwrap();
            },
            _ => panic!()
        }
    }

    println!("{}", sum);

    Ok(())
}
