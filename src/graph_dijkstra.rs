use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Edge {
    u: usize,
    v: usize,
    weight: i32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &Vec<Vec<i32>>, start: usize) -> Vec<Edge> {
    let n = graph.len();
    let mut dist = vec![i32::MAX; n];
    let mut parent = vec![None; n];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, node: start });

    while let Some(State { cost, node }) = heap.pop() {
        if cost > dist[node] {
            continue;
        }

        for i in (0..graph[node].len()).step_by(2) {
            let next = graph[node][i] as usize;
            let next_cost = graph[node][i + 1];
            let new_cost = cost + next_cost;

            if new_cost < dist[next] {
                dist[next] = new_cost;
                parent[next] = Some(node);
                heap.push(State { cost: new_cost, node: next });
            }
        }
    }

    // Reconstruct the shortest paths
    let mut edges = Vec::new();
    for node in 0..n {
        if node != start && dist[node] != i32::MAX {
            let mut current = node;
            while let Some(prev) = parent[current] {
                edges.push(Edge {
                    u: prev,
                    v: current,
                    weight: dist[current] - dist[prev],
                });
                current = prev;
            }
        }
    }

    edges
}

fn floyd_warshall(graph: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = graph.len();
    let mut dist = vec![vec![i32::MAX; n]; n];

    // Initialize distances
    for i in 0..n {
        for j in (0..graph[i].len()).step_by(2) {
            let v = graph[i][j] as usize;
            let weight = graph[i][j + 1];
            dist[i][v] = weight;
        }
        dist[i][i] = 0; // Distance to self is 0
    }

    // the main part of Floyd-Warshall algorithm 
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != i32::MAX && dist[k][j] != i32::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][k].saturating_add(dist[k][j]));
                }
            }
        }
    }

    dist
}

fn bellman_ford(graph: &Vec<Vec<i32>>, start: usize) -> Option<Vec<i32>> {
    let n = graph.len();
    let mut dist = vec![i32::MAX; n];
    dist[start] = 0;

    let mut edges = Vec::new();
    for i in 0..n {
        for j in (0..graph[i].len()).step_by(2) {
            edges.push(Edge {
                u: i,
                v: graph[i][j] as usize,
                weight: graph[i][j + 1],
            });
        }
    }

    // Relax edges |V| - 1 times
    for _ in 0..n-1 {
        for edge in &edges {
            if dist[edge.u] != i32::MAX && dist[edge.u].saturating_add(edge.weight) < dist[edge.v] {
                dist[edge.v] = dist[edge.u].saturating_add(edge.weight);
            }
        }
    }

    // Check for negative-weight cycles
    for edge in &edges {
        if dist[edge.u] != i32::MAX && dist[edge.u].saturating_add(edge.weight) < dist[edge.v] {
            return None; // Negative-weight cycle detected
        }
    }

    Some(dist)
}

fn display_graph(graph: &Vec<Vec<i32>>) {
    println!("Graph visualization:");
    println!("--------------------");

    for (i, edges) in graph.iter().enumerate() {
        println!("Vertex {}:", i);
        for chunk in edges.chunks(2) {
            if chunk.len() == 2 {
                println!("  ├─({})─> {}", chunk[1], chunk[0]);
            }
        }
        if i < graph.len() - 1 {
            println!("  │");
        }
    }

    println!("--------------------");
}

fn main() {
    let graph: Vec<Vec<i32>> = vec![
        vec![1, 2, 2, 2, 4, 1, 6, 3],  // Vertex 0
        vec![0, 2, 2, 2, 3, 3],        // Vertex 1
        vec![0, 2, 1, 3],              // Vertex 2
        vec![1, 3, 4, 3],              // Vertex 3
        vec![0, 1, 3, 2, 6, 3],        // Vertex 4
        vec![4, 4, 6, 2],              // Vertex 5
        vec![5, 2, 4, 4]               // Vertex 6
    ];

    display_graph(&graph);

    let start_node = 0;
    let shortest_paths = dijkstra(&graph, start_node);

    println!("Dijkstra's shortest paths from node {}:", start_node);
    for edge in shortest_paths {
        println!("{} -- {} : {}", edge.u, edge.v, edge.weight);
    }

    println!();
    println!("{:?}", dijkstra(&graph, start_node));
    println!();

    println!("Floyd-Warshall Algorithm Results:");
    let floyd_warshall_result = floyd_warshall(&graph);
    for (i, paths) in floyd_warshall_result.iter().enumerate() {
        println!("Shortest paths from node {}:", i);
        for edge in paths {
            println!("  {}", edge);
        }
        println!();
    }
    println!("{:?}", floyd_warshall(&graph));

    println!("\nBellman-Ford Algorithm Results:");
    match bellman_ford(&graph, start_node) {
        Some(paths) => {
            println!("Shortest paths from node {}:", start_node);
            for edge in &paths {
                println!("  {}", edge);
            }
            println!();
            println!("{:?}", paths);
        },
        None => println!("Negative-weight cycle detected"),
    }
    println!("{:?}", bellman_ford(&graph, start_node));
}