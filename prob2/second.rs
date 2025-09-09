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
    let query_file = &args[2];

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
    println!("Nodes: {:?}", nodes);
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
    // println!("Connected nodes: ");
    // for node in nodes {
    //     println!("{:?}", node);
    // }
    // // println!("connected nodes: {:?}", nodes);

    let contents = fs::read_to_string(query_file)?;
    let queries: Vec<_> = contents.split("\n").collect();
    for query in queries {
        let tmp_query: Vec<String> = query.split_whitespace().map(|s| s.to_string()).collect();
        let (a, b) = (&tmp_query[0], &tmp_query[1]);
        if a == "o" {
            for node in &nodes {
                if node[0].name == *b {
                    println!("{}", node.len() - 1);
                }
            }
        }
        else if a == "i" {
            let mut indegree: Vec<Node> = vec![];
            for node in &mut nodes {
                for n in &node[1..] {
                    if n.name == *b {
                        let tmp_node: Node = Node { name: node[0].name.clone(), weight: node[0].weight };
                        indegree.push(tmp_node);
                    }
                }
            }
            println!("{}", indegree.len());
        }
        else if a == "a" {
            for node in &mut nodes {
                if node[0].name == *b {
                    let adj: &mut [Node] = &mut node[1..];
                    adj.sort();
                    // println!("{:?}", adj);
                    for n in adj {
                        print!("{} ", n.name);
                    }
                    println!();
                }
            }
        }
    }


    Ok(())
}