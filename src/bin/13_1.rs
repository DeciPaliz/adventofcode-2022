use std::{io, cmp::Ordering};

#[derive(PartialEq, Eq, Clone)]
enum PacketNode {
    List(Vec<PacketNode>),
    Integer(i32)
}

impl PacketNode {
    pub fn from_string(src: impl Into<String>) -> Option<PacketNode> {
        let src: String = src.into().trim().to_string();
        if let Some('[') = src.chars().nth(0) {
            let mut brackets_depth = 0;
            return Some(PacketNode::List(src[1..src.len() - 1]
                .split(|c| {
                    match c {
                        '[' => brackets_depth += 1,
                        ']' => brackets_depth -= 1,
                        ',' => if brackets_depth == 0 { return true },
                        _ => ()
                    }
                    false
                })
                .map(|el| PacketNode::from_string(el))
                .filter(|el| el.is_some())
                .map(|el| el.unwrap()).collect())
            );
        }
        let mut len = 0;
        for i in 0..src.len() {
            if !src.chars().nth(i).unwrap().is_numeric() { break; }
            len += 1;
        }
        match src[0..len].parse::<i32>() {
            Ok(i) => Some(PacketNode::Integer(i)),
            _ => None
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            PacketNode::List(_) => true,
            _ => false
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            PacketNode::Integer(_) => true,
            _ => false
        }
    }

    pub fn get_integer(&self) -> Option<i32> {
        match self {
            PacketNode::Integer(i) => Some(*i),
            _ => None
        }
    }

    pub fn get_list(&self) -> Option<Vec<PacketNode>> {
        match self {
            PacketNode::List(v) => Some(v.clone()),
            _ => None
        }
    }
}

impl PartialOrd for PacketNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_integer() && other.is_integer() {
            return self.get_integer().unwrap().partial_cmp(&other.get_integer().unwrap());
        } else if self.is_list() && other.is_list() {
            for (a, b) in self.get_list().unwrap().iter().zip(other.get_list().unwrap().iter()) {
                let cmp_res = a.partial_cmp(b).unwrap();
                if !cmp_res.is_eq() {
                    return Some(cmp_res);
                }
            }
            return self.get_list().unwrap().len().partial_cmp(&other.get_list().unwrap().len());
        } else if self.is_integer() && other.is_list() {
            let new_self = PacketNode::List(vec![PacketNode::Integer(self.get_integer().unwrap())]);
            return new_self.partial_cmp(other);
        } else {
            let new_other = PacketNode::List(vec![PacketNode::Integer(other.get_integer().unwrap())]);
            return self.partial_cmp(&new_other);
        }
    }
}

impl Ord for PacketNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut packet: Option<PacketNode> = None;
    let mut index = 1;
    let mut sum = 0;

    for line in stdin.lines() {
        let line = line?;

        if line.trim().len() == 0 { continue }

        if packet.is_none() {
            packet = Some(PacketNode::from_string(line.trim()).unwrap());
        } else {
            sum += if packet.clone().unwrap() <= PacketNode::from_string(line.trim()).unwrap() { index } else { 0 };
            index += 1;
            packet = None;
        }
    }

    println!("{}", sum);

    Ok(())
}
