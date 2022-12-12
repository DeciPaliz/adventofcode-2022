use std::{io::{self, Read}, collections::HashSet};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut input = Vec::<u8>::new();
    stdin.read_to_end(&mut input)?;
    let mut input = input.iter().map(|x| char::from_u32(*x as u32).unwrap()).collect::<Vec<char>>();
    let mut comm_buffer = vec![
        input.remove(0),
        input.remove(0),
        input.remove(0),
        input.remove(0)
    ];
    let mut count = 4;

    for c in input {
        if HashSet::<char>::from_iter(comm_buffer.clone()).len() == 4 {
            println!("{}", count);
            return Ok(());
        }
        count += 1;
        comm_buffer.remove(0);
        comm_buffer.push(c);
    }

    Ok(())
}
