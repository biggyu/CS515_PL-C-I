use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::fs;
use std::env;

#[derive(Eq)]
struct State {
    cur_dist: i32,
    node: String,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse order for min-heap
        other.cur_dist.cmp(&self.cur_dist)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cur_dist == other.cur_dist && self.node == other.node
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let graph_file = &args[1];
    let query_file = &args[2];

    let contents = fs::read_to_string("graph.txt").expect("Cannot read file");
    let mut lines = contents.lines();
    let node_num: usize = lines.next().unwrap().parse().unwrap();

    let mut nodes: Vec<_> = lines.by_ref().take(node_num).collect();

    let mut graph: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    for node in nodes {
        graph.insert(node.to_string(), Vec::new());
    }

    for line in lines {
        let tmp: Vec<_> = line.split_whitespace().collect();
        graph.get_mut(&tmp[0].to_string()).unwrap().push((tmp[1].to_string(), tmp[2].parse().unwrap()));
        // graph[tmp[0]].push((tmp[1].to_string(), tmp[2].parse().unwrap()));
    }

    let query = fs::read_to_string(query_file).expect("Cannot read file");
    for source in query.lines() {
        let source = source.to_string();
        let dist = dijkstra(&source, &graph);
    
        let mut sorted_node: Vec<_> = dist.keys().map(|s| s.to_string()).collect();
        sorted_node.sort();
        for node in sorted_node {
            println!("{} {}", node, if dist[&node] == i32::MAX { "INF" } else { &dist[&node].to_string() });
        }
        println!();
    }
}

fn dijkstra(source: &String,graph: &HashMap<String, Vec<(String, i32)>>) -> HashMap<String, i32> {
    let mut dist: HashMap<String, i32> = graph.keys().map(|k| (k.clone(), i32::MAX)).collect();
    let mut heap = BinaryHeap::new();

    dist.insert(source.clone(), 0);
    heap.push(State { cur_dist: 0, node: source.clone() });

    while let Some(State { cur_dist, node }) = heap.pop() {
        if cur_dist > dist[&node] {
            continue;
        }

        for (n, w) in &graph[&node] {
            let next_dist = cur_dist + w;
            if next_dist < dist[n] {
                dist.insert(n.clone(), next_dist);
                heap.push(State { cur_dist: next_dist, node: n.clone() });
            }
        }
    }

    dist
}
