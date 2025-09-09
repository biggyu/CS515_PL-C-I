use std::env;
use std::fs;
use std::io;
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Node {
    name: String,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let graph_file = &args[1];
    let query_file = &args[2];

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
    // println!("Connected nodes: ");
    // for node in nodes {
    //     println!("{:?}", node);
    // }
    // // println!("connected nodes: {:?}", nodes);

    let contents = fs::read_to_string(query_file)?;
    let queries: Vec<_> = contents.split("\n").collect();
    for query in queries {
        let tmp_query: String = query.to_string();
        let mut traverse_q: VecDeque<Node> = VecDeque::new();
        let mut traversed: Vec<Node> = vec![];
        // for node in &nodes {
        //     traversed.insert(node[0].name.clone(), 0);
        // }
        traverse_q.push_back( Node { name: tmp_query.clone() });
        // println!("{:?}",traversed.get(&tmp_query.clone()).unwrap());
        // let to_traverse = traverse_q.pop_front().unwrap();
        // println!("{:?}", to_traverse);
        while traverse_q.len() > 0 {
            let to_traverse = traverse_q.pop_front().unwrap();
            for node in &mut nodes {
                // if node[0].name == to_traverse.name && !traversed.get(&to_traverse.name.clone()).unwrap() {
                if node[0].name == to_traverse.name && !traversed.iter().any(|node| node.name == to_traverse.name.clone()) {
                    traversed.push(Node { name: to_traverse.name.clone() });
                    // traversed.entry(node[0].name.clone()).and_modify(|value| *value = true);
                    let to_traverse: &mut [Node] = &mut node[1..];
                    to_traverse.sort();
                    for n in to_traverse {
                        if !traversed.iter().any(|node| node.name == n.name.clone()) {
                            traverse_q.push_back(Node { name: n.name.clone() });
                        }
                    }
                }
            }
        }
        for n in traversed {
            print!("{} ", n.name);
        }
        println!();
    }


    Ok(())
}