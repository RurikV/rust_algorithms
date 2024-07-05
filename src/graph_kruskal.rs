#[derive(Debug, Clone, Copy)]
struct Edge {
    v1: usize,
    v2: usize,
    weight: i32,
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
}

fn kruskal(graph: &Vec<Vec<i32>>) -> Vec<Edge> {
    let n = graph.len();
    let mut edges = Vec::new();

    // Collect all edges
    for i in 0..n {
        for j in 0..graph[i].len() {
            if graph[i][j] != 0 {
                edges.push(Edge { v1: i, v2: j, weight: graph[i][j] });
            }
        }
    }

    // Sort edges by weight
    edges.sort_by_key(|e| e.weight);

    let mut union_find = UnionFind::new(n);
    let mut mst = Vec::new();

    for edge in edges {
        if union_find.find(edge.v1) != union_find.find(edge.v2) {
            union_find.union(edge.v1, edge.v2);
            mst.push(Edge { v1: edge.v1, v2: edge.v2, weight: edge.weight });
        }
    }

    mst
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
    for (j, &weight) in graph[0].iter().enumerate() {
        if weight != 0 {
            pq.push(Reverse((weight, 0, j)));
        }
    }

    while let Some(Reverse((weight, v1, v2))) = pq.pop() {
        if visited[v2] {
            continue;
        }

        visited[v2] = true;
        mst.push(Edge { v1, v2, weight });

        for (j, &w) in graph[v2].iter().enumerate() {
            if w != 0 && !visited[j] {
                pq.push(Reverse((w, v2, j)));
            }
        }
    }

    mst
}

fn boruvka(graph: &Vec<Vec<i32>>) -> Vec<Edge> {
    let n = graph.len();
    let mut union_find = UnionFind::new(n);
    let mut mst = Vec::new();

    loop {
        let mut cheapest = vec![None; n];

        // Find the cheapest edge for each component
        for i in 0..n {
            for j in 0..graph[i].len() {
                if graph[i][j] != 0 {
                    let root_i = union_find.find(i);
                    let root_j = union_find.find(j);

                    if root_i != root_j {
                        match cheapest[root_i] {
                            None => cheapest[root_i] = Some(Edge { v1: i, v2: j, weight: graph[i][j] }),
                            Some(edge) if graph[i][j] < edge.weight => cheapest[root_i] = Some(Edge { v1: i, v2: j, weight: graph[i][j] }),
                            _ => {}
                        }
                    }
                }
            }
        }

        // Add the cheapest edges to the MST
        let mut num_components = 0;
        for i in 0..n {
            if let Some(edge) = cheapest[i] {
                if union_find.find(edge.v1) != union_find.find(edge.v2) {
                    union_find.union(edge.v1, edge.v2);
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
    let graph = vec![
        vec![0, 2, 0, 6, 0],
        vec![2, 0, 3, 8, 5],
        vec![0, 3, 0, 0, 7],
        vec![6, 8, 0, 0, 9],
        vec![0, 5, 7, 9, 0],
    ];
    display_graph(&graph);

    println!("Kruskal's MST: {:?}", kruskal(&graph));
    println!("Prim's MST: {:?}", prim(&graph));
    println!("Boruvka's MST: {:?}", boruvka(&graph));
}