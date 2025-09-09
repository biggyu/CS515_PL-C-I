use std::env;
use std::fs;
use std::io;

#[derive(Debug)]
struct Node {
    name: String,
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
        let tmp_node = Node { name: node.to_string() };
        nodes.push(vec![tmp_node]);
    }
    // println!("Nodes: {:?}", nodes);
    // let vertices = &line[node_num+1..];
    for vertex in &line[node_num+1..] {
        let tmp_nodes: Vec<String> = vertex.split_whitespace().map(|s| s.to_string()).collect();
        let (a, b) = (&tmp_nodes[0], &tmp_nodes[1]);
        for node in &mut nodes {
            if node[0].name == *a {
                node.push(Node { name: b.clone()});
            }
            else if node[0].name == *b {
                node.push(Node { name: a.clone()});
                break;
            }
        }
    }
    // println!("Vertices: {:?}", vertices);
    // println!("Connected nodes: ");
    // for node in nodes {
    //     println!("{:?}", node);
    // }
    println!("connected nodes: {:?}", nodes);

    Ok(())
}