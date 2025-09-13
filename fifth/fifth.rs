use std::env;
use std::fs;
use std::collections::{HashMap, HashSet};

// #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
// struct Node {
//     name: String,
//     weight: i32,
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let graph_file = &args[1];
    let query_file = &args[2];

    let contents = fs::read_to_string(graph_file).expect("Cannot read file");
    let mut lines = contents.lines();
    let node_num: usize = lines.next().unwrap().parse().unwrap();

    let mut nodes: Vec<_> = lines.by_ref().take(node_num).collect();
    // println!("{:?}", nodes);
    // println!("{:?}", lines.next().unwrap());
    let mut graph: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    for node in nodes {
        graph.insert(node.to_string().clone(), Vec::new());
    }
    for line in lines {
        let tmp: Vec<_> = line.split_whitespace().collect();
        graph.get_mut(&tmp[0].to_string()).unwrap().push((tmp[1].to_string(), tmp[2].parse().unwrap()));
        // graph[tmp[0]].push((tmp[1].to_string(), tmp[2].parse().unwrap()));
    }
    // println!("{:?}", graph);
    
    let query = fs::read_to_string(query_file).expect("Cannot read file");
    for source in query.lines() {
        let source = source.to_string();
        // for node in graph.values() {
        //     println!("{:?}", topological_sort(node));
        // }
        // let topo_node = topological_sort(&graph);
        // let mut traversed = HashSet::new();
        let mut state: HashMap<String, i32> = graph.keys().map(|k| (k.clone(), 0)).collect();
        let mut node_order: Vec<String> = Vec::new();
        // println!("{:?}", graph[source]);
        // println!("{:?}", print_type(&graph[source]));
        // for node in graph.keys() {
        //     dfs(&node, &graph, &mut node_order);
        // }
        let mut is_cycle: bool = false;
        if state[&source] == 0 {
            if !dfs(&source, &graph, &mut node_order, &mut state) {
                is_cycle = true;
            }
        }
        for node in graph.keys() {
            if !node_order.contains(&node) && state[&node.clone()]  == 0 {
                if !dfs(&node, &graph, &mut node_order, &mut state) {
                    is_cycle = true;
                }
            }
        }
        if is_cycle {
            println!("CYCLE");
        }
        else {
            let path = shortest_path_dag(&graph, &mut node_order, &source);
            let mut sorted_node: Vec<_> = path.keys().map(|s| s.to_string()).collect();
            sorted_node.sort();
            for node in sorted_node {
                println!("{} {}", node, if path[&node] == i32::MAX { "INF" } else { &path[&node].to_string() });
            }
            println!();
        }
    }
}
fn topological_sort(adj: &Vec<(String, i32)>) -> Vec<(String, i32)> {
    let mut tmp = adj.clone();
    tmp.sort_by(|a, b| a.0.cmp(&b.0));
    tmp
}

fn dfs(source: &String, graph: &HashMap<String, Vec<(String, i32)>>, order: &mut Vec<String>, state: &mut HashMap<String, i32>) -> bool {
    state.insert(source.clone(), 1);
    order.push(source.clone());
    for node in topological_sort(&graph[source]) {
        match state[&node.0] {
            0 => {
                if !dfs(&node.0.clone(), graph, order, state) {
                    return false;
                }
            }
            1 => {
                return false;
            }
            _ => {}
        }
    }
    state.insert(source.clone(), 2);
    true
}
fn shortest_path_dag(graph: &HashMap<String, Vec<(String, i32)>>, order: &mut Vec<String>, source: &String) -> HashMap<String, i32> {
    let mut dist: HashMap<String, i32> = graph.keys().map(|k| (k.clone(), i32::MAX)).collect();

    dist.insert(source.clone(), 0);
    for node in order {
        let cur_dist = *dist.get(node).unwrap();
        if cur_dist != i32::MAX {
            for (k, v) in &graph[node] {
                let new_dist = cur_dist + v;
                if new_dist < *dist.get(k).unwrap() {
                    dist.insert(k.clone(), new_dist);
                }
            }
        }
    }
    dist
}