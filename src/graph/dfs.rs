struct Graph {
    adj: Vec<Vec<usize>>
}

struct State {
    was: Vec<bool>
}

fn dfs(graph: &Graph, state: &mut State, node: usize) -> () {
    state.was[node] = true;
    for &to in graph.adj[node].iter() {
        if !state.was[to] {
            dfs(graph, state, to.clone());
        }
    }
}
// let mut graph = Graph { adj: vec![Vec::new(); n] };
// let mut state = State { was: vec![false; n] };
// dfs(&graph, &mut state, from.clone());