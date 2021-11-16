use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    dist: Reverse<usize>,
    to: usize
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Graph {
    // to and distance
    adj: Vec<Vec<Edge>>,
    dist: Vec<usize>
}

fn dijkstra(graph: &mut Graph) -> Vec<usize> {
    let mut queue = BinaryHeap::new();
    let mut start: usize = 0usize;
    queue.push(Edge { to: start, dist: Reverse(0usize) });
    graph.dist[start] = 0usize;

    while let Some(node) = queue.pop() {
        if node.dist.0 > graph.dist[node.to] {
            continue;
        }
        for neighbor in graph.adj[node.to].clone() {
            let mut new_dist = graph.dist[node.to] + neighbor.dist.0;
            if new_dist < graph.dist[neighbor.to] {
                graph.dist[neighbor.to] = new_dist.clone();
                queue.push(Edge { to: neighbor.to, dist: Reverse(new_dist.clone())});
            }
        }
    }
    graph.dist.clone()
}
