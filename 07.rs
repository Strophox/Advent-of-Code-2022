use std::collections::HashMap;

enum Info {
    File(u32),
    Directory(Vec<usize>),
}

fn main() {
    println!("Opening door ...");
    let input = include_str!("07.txt");

    // Translate from names to numerical IDs
    let mut translations = HashMap::<String, usize>::new();
    // For vertices store parent and file/directory information
    let mut vertices = Vec::<(usize, Info)>::new();

    // Root node
    translations.insert(String::from("/"), 0);
    let root = (0, Info::Directory(vec![]));
    vertices.push(root);
    let mut current_dir: usize = 0;
    let mut current_name = vec![String::from("/")];

    let as_string = |list: &Vec<String>| list.iter().fold(String::new(), |a,b| format!("{a}\\{b}"));

    let mut input_lines = input.lines();
    input_lines.next(); // Discard "$ cd /"
    for line in input_lines {
        let items: Vec<_> = line.split(' ').collect();
        if items[0] == "$" { // Execute command
            if items[1] == "cd" { // Change directory
                current_dir = if items[2] == ".."{
                    current_name.pop();
                    vertices[current_dir].0 // Parent
                } else {
                    current_name.push(String::from(items[2]));
                    *translations.get(&as_string(&current_name)).unwrap() // Subdirectory
                };
            }
        } else { // Modify filesystem structure
            let new_id = vertices.len();
            current_name.push(String::from(items[1]));
            let new_name = as_string(&current_name);//format!("{}/{}", as_string(&current_name), items[1]);
            current_name.pop();
            translations.insert(new_name, new_id); // Add name to translation
            if let Info::Directory(ids) = &mut vertices[current_dir].1 {
                ids.push(new_id); // Add child to current dir
            }
            let new_vertex = if items[0]=="dir" { // New directory
                (current_dir, Info::Directory(vec![]))
            } else { // New file
                (current_dir, Info::File(items[0].parse().unwrap()))
            };
            vertices.push(new_vertex);
        }
    }

    let mut the_sum: u32 = 0;
    let total = dfs(0, &vertices, &mut |&size| {
        if size <= 100_000 {
            the_sum += size;
        }
    });
    println!("Sum of <= 100'000 is {the_sum}");

    let mut best_size: u32 = total;
    dfs(0, &vertices, &mut |&size| {
        if 70_000_000 - (total - size) >= 30_000_000 && size < best_size {
            best_size = size;
        }
    });
    println!("Best candidate directory has size {best_size} (since total is {total}, such that total - best size = {})", total - best_size);

    println!("Goodbye");
}

fn dfs(root: usize, nodes: &Vec::<(usize, Info)>, do_stuff: &mut dyn FnMut(&u32)) -> u32 {
    match &nodes[root].1 {
        Info::File(size) => *size,
        Info::Directory(children) => {
            let temp = children.iter().map(&mut |node: &usize| dfs(*node, &nodes, do_stuff)).sum();
            do_stuff(&temp);
            temp
        },
    }
}

// fn main() {
//     println!("Opening door ...");
//     let input = include_str!("07.txt");
//
//     let mut start = Dir { String::from("/"), Vec::new()};
//     let mut nodes: Vec<Nodes> = vec![start];
//
//     println!("Goodbye");
// }
//
// enum Node<'a> {
//     File {
//         parent: &'a str,
//         size: u32,
//     },
//     Dir {
//         parent: &'a str,
//         children: 'a Vec<String>,
//     },
// }

// struct Node {
//     value: String,
//     neighbors: Vec<u32>,
// }

// struct Dir {
//     parent: Rc<Dir>,
//     sub_dir: HashMap<String, Rc<Dir>>,
//     files: HashMap<u32>,
// }
