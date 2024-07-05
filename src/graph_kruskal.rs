use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Edge {
    u: usize,
    v: usize,
    weight: i32,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find(parent: &mut Vec<usize>, i: usize) -> usize {
    if parent[i] != i {
        parent[i] = find(parent, parent[i]);
    }
    parent[i]
}

fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    let x_root = find(parent, x);
    let y_root = find(parent, y);

    if rank[x_root] < rank[y_root] {
        parent[x_root] = y_root;
    } else if rank[x_root] > rank[y_root] {
        parent[y_root] = x_root;
    } else {
        parent[y_root] = x_root;
        rank[x_root] += 1;
    }
}

fn kruskal_mst(graph: &Vec<Vec<i32>>) -> Vec<Edge> {
    let mut edges = Vec::new();
    let n = graph.len();

    for i in 0..n {
        for j in 0..graph[i].len() / 2 {
            let v = graph[i][j * 2];
            let weight = graph[i][j * 2 + 1];
            edges.push(Edge { u: i, v: v as usize, weight });
        }
    }

    edges.sort_unstable();

    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0; n];
    let mut result = Vec::new();

    for edge in edges {
        let x = find(&mut parent, edge.u);
        let y = find(&mut parent, edge.v);

        if x != y {
            result.push(edge);
            union(&mut parent, &mut rank, x, y);
        }

        if result.len() == n - 1 {
            break;
        }
    }

    result
}


use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn prim(graph: &Vec<Vec<i32>>) -> Vec<Edge> {
    let n = graph.len();
    let mut visited = vec![false; n];
    let mut mst = Vec::new();
    let mut pq = BinaryHeap::new();

    // Start with vertex 0
    visited[0] = true;
    for j in (0..graph[0].len()).step_by(2) {
        let v = graph[0][j] as usize;
        let weight = graph[0][j+1];
        pq.push(Reverse((weight, 0, v)));
    }

    while let Some(Reverse((weight, u, v))) = pq.pop() {
        if visited[v] {
            continue;
        }

        visited[v] = true;
        mst.push(Edge { u, v, weight });

        for j in (0..graph[v].len()).step_by(2) {
            let next_v = graph[v][j] as usize;
            let next_weight = graph[v][j+1];
            if !visited[next_v] {
                pq.push(Reverse((next_weight, v, next_v)));
            }
        }
    }

    mst
}

fn boruvka(graph: &Vec<Vec<i32>>) -> Vec<Edge> {
    let n = graph.len();
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0; n];
    let mut mst = Vec::new();

    loop {
        let mut cheapest = vec![None; n];

        // Find the cheapest edge for each component
        for i in 0..n {
            for j in (0..graph[i].len()).step_by(2) {
                let v = graph[i][j] as usize;
                let weight = graph[i][j+1];
                let root_i = find(&mut parent, i);
                let root_v = find(&mut parent, v);

                if root_i != root_v {
                    match cheapest[root_i] {
                        None => cheapest[root_i] = Some(Edge { u: i, v, weight }),
                        Some(edge) if weight < edge.weight => cheapest[root_i] = Some(Edge { u: i, v, weight }),
                        _ => {}
                    }
                }
            }
        }

        // Add the cheapest edges to the MST
        let mut num_components = 0;
        for i in 0..n {
            if let Some(edge) = cheapest[i] {
                let root_u = find(&mut parent, edge.u);
                let root_v = find(&mut parent, edge.v);
                if root_u != root_v {
                    union(&mut parent, &mut rank, root_u, root_v);
                    mst.push(edge);
                    num_components += 1;
                }
            }
        }

        // If no more components to join, we're done
        if num_components == 0 {
            break;
        }
    }

    mst
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

    println!("Kruskal's MST:");
    for edge in kruskal_mst(&graph) {
        println!("{} -- {} : {}", edge.u, edge.v, edge.weight);
    }

    println!("\nPrim's MST:");
    for edge in prim(&graph) {
        println!("{} -- {} : {}", edge.u, edge.v, edge.weight);
    }

    println!("\nBoruvka's MST:");
    for edge in boruvka(&graph) {
        println!("{} -- {} : {}", edge.u, edge.v, edge.weight);
    }

    println!();
    println!("Kruskal's MST: {:?}", kruskal_mst(&graph));
    println!("Prim's MST: {:?}", prim(&graph));
    println!("Boruvka's MST: {:?}", boruvka(&graph));
}