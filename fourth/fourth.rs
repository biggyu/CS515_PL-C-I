use std::env;
use std::fs;
use std::io;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Node {
    name: String,
    weight: i32,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let graph_file = &args[1];

    let contents = fs::read_to_string(graph_file)?;
    let line: Vec<_> = contents.split("\n").collect();
    let mut nodes: Vec<Vec<Node>> = vec![];
    let node_num: usize = line[0].parse::<usize>().unwrap();
    // println!("Node num: {}", node_num);
    // let nodes_vec = &line[1..=node_num];
    for node in &line[1..=node_num] {
        let tmp_node = Node { name: node.to_string(), weight: 0i32 };
        nodes.push(vec![tmp_node]);
    }
    // println!("Nodes: {:?}", nodes);
    // let vertices = &line[node_num+1..];
    for vertex in &line[node_num+1..] {
        let tmp_nodes: Vec<String> = vertex.split_whitespace().map(|s| s.to_string()).collect();
        let (a, b, c) = (&tmp_nodes[0], &tmp_nodes[1], &tmp_nodes[2]);
        for node in &mut nodes {
            if node[0].name == *a {
                match c.parse::<i32>() {
                    Ok(number) => node.push(Node { name: b.clone(), weight: number }),
                    Err(e) => println!("Error passing vertex weight"),
                }
                // node.push(Node { name: b.clone(), weight: c });
            }
        }
    }
    nodes.sort();
    // println!("Connected nodes: ");
    // for node in &nodes {
    //     println!("{:?}", node);
    // }
    let mut traversed: Vec<Node> = vec![];
    // for node in &mut nodes {
    for i in 0..nodes.len() {
        if !traversed.iter().any(|n| n.name == nodes[i][0].name.clone()) {
            let mut to_traverse: Vec<Node> = vec![ Node { name: nodes[i][0].name.clone(), weight: nodes[i][0].weight }];
            // println!("{:?}", node);
            while to_traverse.len() > 0 {
                // let next_node: Node = to_traverse.pop().unwrap();
                let next_node: &Node = to_traverse.last().unwrap();
                // let n = &next_node.name.clone();
                // println!("{}", n);
                if !traversed.iter().any(|n| n.name == next_node.name.clone()) {
                    traversed.push(Node{ name: next_node.name.clone(), weight: next_node.weight });
                }
                for j in 0..nodes.len() {
                    if nodes[j][0].name == next_node.name {
                        let mut found: bool = false;
                        let tmp_node: &mut [Node] = &mut nodes[j][1..];
                        tmp_node.sort();
                        for k in 0..tmp_node.len() {
                            if !traversed.iter().any(|n| n.name == tmp_node[k].name.clone()) {
                                to_traverse.push(Node { name: tmp_node[k].name.clone(), weight: tmp_node[k].weight });
                                found = true;
                                // println!("traversed {:?}", traversed);
                                // println!("to_traverse {:?}", to_traverse);
                                break;
                            }
                        }
                        if !found {
                            // println!("???????????????? {:?}", to_traverse.pop());
                            to_traverse.pop();
                        }
                        break;
                    }
                }
            }
            
        }
    }
    for node in traversed {
        print!("{} ", node.name);
    }
    println!();
    // println!("{:?}", traversed);


    Ok(())
}