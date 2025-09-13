use std::env;
use std::fs;
use std::collections::{VecDeque, HashMap};

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
    println!("{:?}", graph);
    let query = fs::read_to_string(query_file).expect("Cannot read file");
    for source in query.lines() {
        let source = source.to_string();
        let mut node_order: Vec<String> = Vec::new();
        bfs(&source, &graph, &mut node_order);

        for node in node_order {
            print!("{} ", node);
        }
        println!();
    }
}
fn topological_sort(adj: &Vec<String>) -> Vec<String> {
    let mut tmp = adj.clone();
    tmp.sort_by(|a, b| a.cmp(&b));
    tmp
}
fn bfs(source: &String, graph: &HashMap<String, Vec<String>>, order: &mut Vec<String>) {
    let mut traverse_q = VecDeque::new(); 

    traverse_q.push_back(source.clone());
    while let Some(node) = traverse_q.pop_front() {
        order.push(node.clone());
        for n in topological_sort(&graph[&node]) {
            let nxt = n.clone();
            if !order.contains(&nxt) && !traverse_q.contains(&nxt) {
                traverse_q.push_back(nxt);
            }
        }
    }
}