extern crate num;

use std::io;

enum MonkeyOperation {
    Add(u32),
    Multiply(u32),
    Square
}

impl MonkeyOperation {
    pub fn execute(&self, worry_level: u128) -> u128 {
        match self {
            Self::Add(i) => worry_level + *i as u128,
            Self::Multiply(i) => worry_level * *i as u128,
            Self::Square => worry_level * worry_level
        }
    }
}

struct Monkey {
    items: Vec<u128>,
    operation: MonkeyOperation,
    test: u32,
    true_throw: usize,
    false_throw: usize,
    checks: u64
}

impl Monkey {
    pub fn new(buf: &Vec<String>) -> Self {
        let mut buf = buf.iter();
        buf.next().unwrap();
        let items = Vec::from_iter(buf.next().unwrap()[17..].split(", ").map(|x| x.trim().parse::<u128>().unwrap()));

        let operation: MonkeyOperation;
        {
            let mut temp = buf.next().unwrap()[19..].trim().split(' ');
            temp.next().unwrap();
            match temp.next().unwrap().trim() {
                "+" => operation = MonkeyOperation::Add(temp.next().unwrap().trim().parse::<u32>().unwrap()),
                "*" => match temp.next().unwrap().trim() {
                    "old" => operation = MonkeyOperation::Square,
                    x => operation = MonkeyOperation::Multiply(x.parse::<u32>().unwrap())
                },
                _ => panic!()
            }
        }

        let test = buf.next().unwrap()[21..].trim().parse::<u32>().unwrap();
        
        let true_throw = buf.next().unwrap()[29..].trim().parse::<usize>().unwrap();
        let false_throw = buf.next().unwrap()[30..].trim().parse::<usize>().unwrap();

        Self { items, operation, test, true_throw, false_throw, checks: 0 }
    }

    pub fn catch_item(&mut self, item: u128) {
        self.items.push(item);
    }
    
    fn update(&mut self) -> Option<(usize, u128)> {
        if self.items.len() == 0 { return None }

        let mut item = self.items.remove(0);
        self.checks += 1;
        item = self.operation.execute(item);

        if item.clone() % self.test as u128 == 0 {
            return Some((self.true_throw, item));
        }
        Some((self.false_throw, item))
    }

    pub fn turn(&mut self) -> Vec<(usize, u128)> {
        let mut res = vec![];
        while self.items.len() > 0 {
            if let Some(r) = self.update() {
                res.push(r);
            }
        }
        res
    }

    pub fn get_checks(&self) -> u64 {
        self.checks
    }

    pub fn get_test(&self) -> u32 {
        self.test
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut monkeys = Vec::<Monkey>::new();
    let mut buf = Vec::<String>::new();

    for line in stdin.lines() {
        let line = line?;
        if line.trim().len() == 0 {
            monkeys.push(Monkey::new(&buf));
            buf = vec![];
        } else {
            buf.push(line);
        }
    }
    monkeys.push(Monkey::new(&buf));

    let modulo = monkeys.iter().map(|el| el.get_test() as u128).reduce(|acc, el| acc*el).unwrap();

    for i in 0..10000 {
        for i in 0..monkeys.len() {
            let turn = monkeys[i].turn();
            for throw in turn {
                monkeys[throw.0].catch_item(throw.1 % modulo);
            }
        }
    }

    let mut checks = monkeys.iter().map(|monkey| monkey.get_checks()).collect::<Vec<u64>>();
    checks.sort();
    checks.reverse();

    println!("{}", checks[0] * checks[1]);

    Ok(())
}
