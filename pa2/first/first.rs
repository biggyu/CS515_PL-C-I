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
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for node in nodes {
        graph.insert(node.to_string().clone(), Vec::new());
    }
    
    for line in lines {
        let tmp: Vec<_> = line.split_whitespace().collect();
        graph.get_mut(&tmp[0].to_string()).unwrap().push(tmp[1].to_string());
        graph.get_mut(&tmp[1].to_string()).unwrap().push(tmp[0].to_string());
    }

    let queries = fs::read_to_string(query_file).expect("Cannot read file");
    for query in queries.lines() {
        let tmp_query: Vec<String> = query.split_whitespace().map(|s| s.to_string()).collect();
        let (a, b) = (&tmp_query[0], &tmp_query[1]);
        if a == "d" {
            for node in graph.keys() {
                if node == b {
                    println!("{}", graph[node].len());
                    break;
                }
            }
        }
        else if a == "a" {
            for (node, adj) in &graph {
                if *node == *b {
                    let mut tmp = adj.clone();
                    tmp.sort_by(|a, b| a.cmp(&b));
                    for n in tmp {
                        print!("{} ", n);
                    }
                    println!();
                    break;
                }
            }
        }
    }
}