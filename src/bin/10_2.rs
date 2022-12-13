use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut x = 1;
    let mut cycle = 0;
    let mut crt_row = String::new();
    let mut crt = String::new();

    for line in stdin.lines() {
        let line = line?;
        let mut split = line.split(' ');
        let command = split.next().unwrap().trim();
        let amount: Option<i32> = if let Some(s) = split.next() { Some(s.trim().parse::<i32>().unwrap()) } else { None };

        if cycle % 40 == 0 {
            crt.push_str(&crt_row);
            crt.push('\n');
            crt_row = String::new();
        }

        crt_row.push(if (cycle % 40 as i32).abs_diff(x) <= 1 { '#' } else { '.' });
        
        match command {
            "noop" => { cycle += 1 },
            "addx" => { 
                cycle += 1;

                if cycle % 40 == 0 {
                    crt.push_str(&crt_row);
                    crt.push('\n');
                    crt_row = String::new();
                }
                crt_row.push(if (cycle % 40 as i32).abs_diff(x) <= 1 { '#' } else { '.' });

                cycle += 1;
                x += amount.unwrap();
            },
            _ => panic!()
        }
    }

    println!("{}{}\n", crt, crt_row);

    Ok(())
}
