use std::io;

fn test_assignment_overlap(assignment: Vec<u32>) -> bool {
    assignment[0] <= assignment[3] && assignment[1] >= assignment[2]
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut count: u64 = 0;

    for line in stdin.lines() {
        let assignment = line?.split(',').map(|x| x.split('-').collect::<Vec<&str>>()).flatten().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        if test_assignment_overlap(assignment) { count += 1 }
    }

    println!("{}", count);

    Ok(())
}
