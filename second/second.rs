use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let graph_file = &args[1];
    let query_file = &args[2];

    let contents = fs::read_to_string(graph_file).expect("Cannot read file");
    let mut lines = contents.lines();
    let node_num: usize = lines.next().unwrap().parse().unwrap();
    let mut nodes: Vec<_> = lines.by_ref().take(node_num).collect();

    let mut graph: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    for node in nodes {
        graph.insert(node.to_string().clone(), Vec::new());
    }
    for line in lines {
        let tmp: Vec<_> = line.split_whitespace().collect();
        graph.get_mut(&tmp[0].to_string()).unwrap().push((tmp[1].to_string(), tmp[2].parse().unwrap()));
    }
    // println!("Nodes: {:?}", nodes);
    let queries = fs::read_to_string(query_file).expect("Cannot read file");
    for query in queries.lines() {
        let tmp_query: Vec<String> = query.split_whitespace().map(|s| s.to_string()).collect();
        let (a, b) = (&tmp_query[0], &tmp_query[1]);
        if a == "o" {
            for node in graph.keys() {
                if node == b {
                    println!("{}", graph[node].len());
                    break;
                }
            }
            // for node in &nodes {
            //     if node[0].0 == *b {
            //         println!("{}", node.len() - 1);
            //         break;
            //     }
            // }
        }
        else if a == "i" {
            let mut indegree = Vec::new();
            for (node, adj) in &graph {
                for (n, w) in adj {
                    if *n == *b {
                        indegree.push((node.clone(), w));
                    }
                }
            }
            // for node in &mut nodes {
            //     for n in &node[1..] {
            //         if n.0 == *b {
            //             // let tmp_node: Node = Node { name: node[0].0.clone(), weight: node[0].weight };
            //             indegree.push((node[0].0.clone(), node[0].weight));
            //         }
            //     }
            // }
            println!("{}", indegree.len());
        }
        else if a == "a" {
            for (node, adj) in &graph {
                if *node == *b {
                    let mut tmp = adj.clone();
                    tmp.sort_by(|a, b| a.0.cmp(&b.0));
                    for n in tmp {
                        print!("{} ", n.0);
                    }
                    println!();
                    break;
                }
            }
            // for node in &mut nodes {
            //     if node[0].0 == *b {
            //         let adj: &mut [Node] = &mut node[1..];
            //         adj.sort();
            //         // println!("{:?}", adj);
            //         for n in adj {
            //             print!("{} ", n.0);
            //         }
            //         println!();
            //         break;
            //     }
            // }
        }
    }
    // let vertices = &line[node_num+1..];
    // for vertex in &line[node_num+1..] {
    //     let tmp_nodes: Vec<String> = vertex.split_whitespace().map(|s| s.to_string()).collect();
    //     let (a, b, c) = (&tmp_nodes[0], &tmp_nodes[1], &tmp_nodes[2]);
    //     for node in &mut nodes {
    //         if node[0].name == *a {
    //             match c.parse::<i32>() {
    //                 Ok(number) => node.push(Node { name: b.clone(), weight: number }),
    //                 Err(e) => println!("Error passing vertex weight"),
    //             }
    //             // node.push(Node { name: b.clone(), weight: c });
    //         }
    //     }
    // }
    // println!("Connected nodes: ");
    // for node in nodes {
    //     println!("{:?}", node);
    // }
    // // println!("connected nodes: {:?}", nodes);

    // let contents = fs::read_to_string(query_file)?;
    // let queries: Vec<_> = contents.split("\n").collect();
    // for query in queries {
    //     let tmp_query: Vec<String> = query.split_whitespace().map(|s| s.to_string()).collect();
    //     let (a, b) = (&tmp_query[0], &tmp_query[1]);
    //     if a == "o" {
    //         for node in &nodes {
    //             if node[0].name == *b {
    //                 println!("{}", node.len() - 1);
    //             }
    //         }
    //     }
    //     else if a == "i" {
    //         let mut indegree: Vec<Node> = vec![];
    //         for node in &mut nodes {
    //             for n in &node[1..] {
    //                 if n.name == *b {
    //                     let tmp_node: Node = Node { name: node[0].name.clone(), weight: node[0].weight };
    //                     indegree.push(tmp_node);
    //                 }
    //             }
    //         }
    //         println!("{}", indegree.len());
    //     }
    //     else if a == "a" {
    //         for node in &mut nodes {
    //             if node[0].name == *b {
    //                 let adj: &mut [Node] = &mut node[1..];
    //                 adj.sort();
    //                 // println!("{:?}", adj);
    //                 for n in adj {
    //                     print!("{} ", n.name);
    //                 }
    //                 println!();
    //             }
    //         }
    //     }
    // }
}