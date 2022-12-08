use std::fs;

#[derive(Debug, Default)]
struct ArenaTree<String> {
    arena: Vec<Node<String>>,
}

impl ArenaTree<String> {
    fn node(&mut self, cur_idx: usize, val: String) -> usize {
        //first see if it exists
        if &self.arena.len() > &0 {
            let cur_node = &self.arena[cur_idx];
            for child_idx in &cur_node.children {
                if self.arena[*child_idx].val == val {
                    return self.arena[*child_idx].idx;
                }
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }

    fn insert(&mut self, p_idx: usize, c: String) {
        // Init nodes
        let outer = self.node(p_idx, c);
        // set orbit
        match self.arena[outer].parent {
            Some(_) => panic!("Attempt to overwrite existing orbit"),
            None => self.arena[outer].parent = Some(p_idx),
        }
        // set parents
        self.arena[p_idx].children.push(outer);
    }

    // Now, we will traverse each node and its children.
    fn traverse(&self, start_idx: usize, size_vec: &mut Vec<i32>) -> i32 {
        let mut file_total: i32 = 0;
        for child in &self.arena[start_idx].children {
            println!(
                "{} -> {}",
                self.arena[start_idx].val, self.arena[*child].val
            );
            if self.arena[*child].val.parse::<i32>().is_ok() {
                file_total += self.arena[*child].val.parse::<i32>().unwrap();
            } else {
                file_total += self.traverse(*child, size_vec);
            }
        }
        println!(
            "File total for {} is {}",
            self.arena[start_idx].val, file_total
        );

        if file_total <= 100000 {
            println!(
                "File total for {} is {}",
                self.arena[start_idx].val, file_total
            );
            size_vec.push(file_total);
        }
        file_total
    }
}

#[derive(Debug)]
struct Node<String> {
    idx: usize,
    val: String,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node<String> {
    fn new(idx: usize, val: String) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: Vec::new(),
        }
    }
}

fn parse_file(fp: &str) -> ArenaTree<String> {
    let mut tree: ArenaTree<String> = ArenaTree::default();

    let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");
    let mut files: bool = false;
    let mut cur_dir_idx: usize = 0;

    for l in contents.lines() {
        if l.starts_with("$") {
            // Remove the $ and the leading space.
            let instruction = &l[2..];
            if instruction.contains("ls") {
                files = true;
            } else if instruction.contains("cd") {
                files = false;
                let dir = instruction[3..].to_string();
                if dir == ".." {
                    // Then, we set the current directory to the parent of the current directory.
                    cur_dir_idx = tree.arena[cur_dir_idx].parent.unwrap();
                } else {
                    cur_dir_idx = tree.node(cur_dir_idx, dir);
                }
            }
        } else if files {
            let name = parse_internal_file(l);
            tree.insert(cur_dir_idx, name);
        }
    }

    tree
}

fn parse_internal_file(file: &str) -> String {
    let splitted = file.split(" ").collect::<Vec<&str>>();
    // Test if the first element is a number or not.
    if splitted[0].parse::<i32>().is_ok() {
        // If it is a number, then it is a file. We want the size.
        splitted[0].to_string()
    } else {
        // If it is not a number, then it is a directory. We want the name.
        splitted[1].to_string()
    }
}

fn main() {
    let mut size_vec: Vec<i32> = Vec::new();
    let tree = parse_file("input.txt");
    println!("{:?}", tree);
    tree.traverse(0, &mut size_vec);
    println!("The sum is: {}", size_vec.iter().sum::<i32>());
    // let hello = tree.node("123".into());
    // let world = tree.node("456".into());
    // tree.arena[hello].children.push(world);
    // tree.arena[world].parent = Some(hello);

    // println!("{:?}", tree);
}
