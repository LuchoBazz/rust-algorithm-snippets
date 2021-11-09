#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Graph {
    adj: Vec<Vec<usize>>,
    was: Vec<bool>
}

fn dfs(graph: &Graph, node: usize) -> () {
    graph.was[node] = true;
    for &to in graph.adj[node].clone().iter() {
        if !graph.was[to] {
            dfs(graph, to.clone());
        }
    }
}
// let mut graph = Graph { adj: vec![Vec::new(); n], was: vec![false; n] };
// dfs(&mut graph, from.clone());