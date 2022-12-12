use std::io;

struct Stacks {
    vecs: [Vec<char>; 10]
}

impl Stacks {
    pub fn new() -> Self {
        Self { vecs: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()] }
    }

    pub fn input_line(&mut self, line: String) {
        let parsed = Stacks::parse_line(line);
        for i in 0..10 as usize {
            if let Some(c) = parsed[i] {
                self.vecs[i].insert(0, c);
            }
        }
    }

    fn parse_line(line: String) -> [Option<char>; 10] {
        let mut res = [None; 10];
        let line = line.chars().collect::<Vec<char>>();
        for i in 0..9 as usize {
            if line[4*i + 1] != ' ' { res[i + 1] = Some(line[4*i + 1]) }
        }
        res
    }

    pub fn get_output(&self) -> String {
        self.vecs.iter().map(|x| x.last()).filter(|x| x.is_some()).map(|x| x.unwrap()).collect::<String>()
    }

    fn move_helper(&mut self, from: usize, to: usize) {
        if self.vecs[from].last().is_none() { return }
        let c = self.vecs[from].pop().unwrap();
        self.vecs[to].push(c);
    }

    pub fn move_crates(&mut self, amount: usize, from: usize, to: usize) {
        for _ in 0..amount {
            self.move_helper(from, to);
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stacks = Stacks::new();
    let mut listening_for_crates = true;

    for line in stdin.lines() {
        let line = line?;
        if listening_for_crates {
            if line.split(' ').all(|x| x.trim().parse::<usize>().is_ok()) {
                continue;
            }
            if line.len() == 0 {
                listening_for_crates = false;
                continue;
            }
            stacks.input_line(line);
            continue;
        }
        let command = line.split(' ').filter(|x| x.trim().parse::<usize>().is_ok()).map(|x| x.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
        stacks.move_crates(command[0], command[1], command[2]);
    }

    println!("{}", stacks.get_output());

    Ok(())
}
