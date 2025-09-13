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

    let contents = fs::read_to_string(graph_file).expect("Cannot read file");;
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

    let source = "A".to_string();
    // for node in graph.values() {
    //     println!("{:?}", topological_sort(node));
    // }
    // let topo_node = topological_sort(&graph);
    // let mut traversed = HashSet::new();
    let mut node_order: Vec<String> = Vec::new();
    // println!("{:?}", graph[source]);
    // println!("{:?}", print_type(&graph[source]));
    // for node in graph.keys() {
    //     dfs(&node, &graph, &mut node_order);
    // }
    dfs(&source, &graph, &mut node_order);
    for node in graph.keys() {
        if !node_order.contains(&node) {
            dfs(&node, &graph, &mut node_order);
        }
    }
    for node in node_order {
        print!("{} ", node);
    }
    println!();
}
fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
fn topological_sort(adj: &Vec<(String, i32)>) -> Vec<(String, i32)> {
    let mut tmp = adj.clone();
    tmp.sort_by(|a, b| a.0.cmp(&b.0));
    tmp
}
// fn dfs(source: &String, graph: &HashMap<String, Vec<(String, i32)>>, traversed: &mut HashSet<String>, order: &mut Vec<String>) {
fn dfs(source: &String, graph: &HashMap<String, Vec<(String, i32)>>, order: &mut Vec<String>) {
    // traversed.insert(source.clone());
    order.push(source.clone());
    for node in topological_sort(&graph[source]) {
        // println!("{:?}", node.0);
        let nxt = node.0.clone();
        if !order.contains(&nxt) {
            dfs(&nxt, graph, order);
        }
    }
}