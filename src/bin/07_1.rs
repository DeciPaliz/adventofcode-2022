use std::{io, slice::Iter};

struct FSNode {
    parent: Option<usize>,
    index: usize,
    children: Vec<usize>,
    dir: bool,
    size: u64,
    name: String
}

impl FSNode {
    pub fn from_string(parent: Option<usize>, index: usize, src: String) -> Self {
        let dir = src.split(' ').next().unwrap() == "dir";
        let split = src.split(' ').collect::<Vec<&str>>();
        Self { parent, index, children: vec![], dir, size: if !dir { split[0].trim().parse::<u64>().unwrap() } else { 0 }, name: split[1].trim().to_string() }
    }

    pub fn add_child(&mut self, child_index: usize) {
        self.children.push(child_index);
    }

    pub fn get_parent(&self) -> Option<usize> {
        self.parent
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_children(&self) -> Vec<usize> {
        self.children.clone()
    }
    
    pub fn is_dir(&self) -> bool {
        self.dir
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

struct FSTree {
    nodes: Vec<FSNode>
}

impl FSTree {
    pub fn new() -> Self { Self { nodes: vec![FSNode { parent: None, index: 0, children: vec![], dir: true, size: 0, name: "/".to_string() }] } }

    pub fn insert_node(&mut self, parent: usize, src: String) -> Result<usize, ()> {
        let index = self.nodes.len();
        if parent >= index {
            return Err(());
        }
        self.nodes.push(FSNode::from_string(Some(parent), index, src));
        self.nodes[parent].add_child(index);

        Ok(index)
    }

    pub fn list_children(&self, parent: usize) -> Result<Vec<&FSNode>, ()> {
        let mut res = Vec::new();

        if parent >= self.nodes.len() { return Err(()) }

        for i in self.nodes[parent].children.iter() {
            if *i >= self.nodes.len() { return Err(()) }
            res.push(self.nodes.get(*i).unwrap());
        }
        
        Ok(res)
    }

    pub fn get_node(&self, index: usize) -> Result<&FSNode, ()> {
        match self.nodes.get(index) {
            Some(n) => Ok(n),
            None => Err(())
        }
    }

    pub fn calc_sizes(&mut self, index: usize) -> u64 {
        let mut size = 0;
        for child in self.nodes[index].get_children() {
            size += if self.nodes[child].is_dir() { self.calc_sizes(child) } else { self.nodes[child].get_size() }
        }
        self.nodes[index].set_size(size);
        size
    }

    pub fn iter(&self) -> Iter<FSNode> {
        self.nodes.iter()
    }
}

struct FSExplorer<'a> {
    tree: &'a mut FSTree,
    pointer: usize
}

impl<'a> FSExplorer<'a> {
    pub fn new(tree: &'a mut FSTree) -> Self { Self { tree, pointer: 0 } }

    pub fn cd(&mut self, to: String) -> Result<(), ()> {
        if to == ".." {
            if let Some(parent) = self.tree.get_node(self.pointer)?.get_parent() {
                self.pointer = parent;
                return Ok(());
            } else {
                return Err(());
            }
        }
        for child in self.tree.list_children(self.pointer)?.iter().filter(|x| x.is_dir()).map(|x| (x.get_index(), x.get_name())) {
            if child.1 == to {
                self.pointer = child.0;
                return Ok(());
            }
        }

        Err(())
    }

    pub fn ls(&mut self, result: &Vec<String>) -> Result<(), ()> {
        for s in result {
            self.tree.insert_node(self.pointer, s.to_string())?;
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut tree = FSTree::new();
    let mut explorer = FSExplorer::new(&mut tree);

    let mut ls_buffer = Vec::<String>::new();
    let mut lines = stdin.lines();
    lines.next().unwrap()?;

    for line in lines {
        let line = line?;
        let inp = line.split(' ').map(|x| x.trim().to_string()).collect::<Vec<String>>();
        if inp[0] == "$" {
            if ls_buffer.len() > 0 {
                explorer.ls(&ls_buffer).unwrap();
                ls_buffer = vec![];
            }
            if inp[1] == "cd" {
                if let Err(_) = explorer.cd(inp[2].clone()) {
                    return Ok(());
                }
            }
            continue;
        }
        ls_buffer.push(line);
    }

    tree.calc_sizes(0);

    let mut total_size = 0;
    tree.iter().filter(|x| x.is_dir()).filter(|x| x.get_size() <= 100000).for_each(|x| total_size += x.get_size());

    println!("{}", total_size);

    Ok(())
}
