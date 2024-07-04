use std::collections::HashMap;

// Struct to represent the graph
struct BipartiteGraph {
    left_vertices: Vec<char>,
    right_vertices: Vec<char>,
    edges: Vec<(char, char)>,
}

impl BipartiteGraph {
    fn new(left_vertices: Vec<char>, right_vertices: Vec<char>, edges: Vec<(char, char)>) -> Self {
        BipartiteGraph {
            left_vertices,
            right_vertices,
            edges,
        }
    }

    // 1. Draw the graph using ASCII characters
    fn draw(&self) {
        println!("Bipartite Graph A(3,4) with 5 edges:");
        println!("Left set:  A B C");
        println!("Right set: 1 2 3 4");
        println!("Edges:");
        for (left, right) in &self.edges {
            println!("  {} - {}", left, right);
        }
    }

    // 2. Set enumeration
    fn set_enumeration(&self) {
        println!("Set enumeration:");
        println!("Left set: {:?}", self.left_vertices);
        println!("Right set: {:?}", self.right_vertices);
    }

    // 3. Adjacency matrix
    fn adjacency_matrix(&self) {
        println!("Adjacency matrix:");
        let all_vertices: Vec<char> = self.left_vertices.iter().chain(self.right_vertices.iter()).cloned().collect();
        for &v1 in &all_vertices {
            for &v2 in &all_vertices {
                print!("{} ", if self.edges.contains(&(v1, v2)) || self.edges.contains(&(v2, v1)) { "1" } else { "0" });
            }
            println!();
        }
    }

    // 4. Incidence matrix
    fn incidence_matrix(&self) {
        println!("Incidence matrix:");
        for &v in self.left_vertices.iter().chain(self.right_vertices.iter()) {
            for (_i, &(left, right)) in self.edges.iter().enumerate() {
                if v == left {
                    print!(" 1");
                } else if v == right {
                    print!("-1");
                } else {
                    print!(" 0");
                }
            }
            println!();
        }
    }

    // 5. Edge list
    fn edge_list(&self) {
        println!("Edge list:");
        for &(left, right) in &self.edges {
            println!("({}, {})", left, right);
        }
    }

    // 6. Adjacency vectors
    fn adjacency_vectors(&self) {
        println!("Adjacency vectors:");
        for &v in self.left_vertices.iter().chain(self.right_vertices.iter()) {
            print!("{}: ", v);
            for &(left, right) in &self.edges {
                if v == left {
                    print!("{} ", right);
                } else if v == right {
                    print!("{} ", left);
                }
            }
            println!();
        }
    }

    // 7. Adjacency arrays
    fn adjacency_arrays(&self) {
        println!("Adjacency arrays:");
        let mut adj: HashMap<char, Vec<char>> = HashMap::new();
        for &v in self.left_vertices.iter().chain(self.right_vertices.iter()) {
            adj.insert(v, Vec::new());
        }
        for &(left, right) in &self.edges {
            adj.get_mut(&left).unwrap().push(right);
            adj.get_mut(&right).unwrap().push(left);
        }
        for (v, neighbors) in &adj {
            println!("{}: {:?}", v, neighbors);
        }
    }

    // 8. Adjacency lists
    fn adjacency_lists(&self) {
        println!("Adjacency lists:");
        let mut adj: HashMap<char, Vec<char>> = HashMap::new();
        for &v in self.left_vertices.iter().chain(self.right_vertices.iter()) {
            adj.insert(v, Vec::new());
        }
        for &(left, right) in &self.edges {
            adj.get_mut(&left).unwrap().push(right);
            adj.get_mut(&right).unwrap().push(left);
        }
        for (v, neighbors) in &adj {
            print!("{} -> ", v);
            for (i, &n) in neighbors.iter().enumerate() {
                if i > 0 {
                    print!(" -> ");
                }
                print!("{}", n);
            }
            println!();
        }
    }

    // 9. Structure with table of contents
    fn structure_with_toc(&self) {
        println!("Structure with table of contents:");
        println!("1. Vertices");
        println!("   1.1 Left set: {:?}", self.left_vertices);
        println!("   1.2 Right set: {:?}", self.right_vertices);
        println!("2. Edges");
        for (i, &(left, right)) in self.edges.iter().enumerate() {
            println!("   2.{} ({}, {})", i + 1, left, right);
        }
    }

    // 10. List of vertices and list of edges
    fn vertices_and_edges_lists(&self) {
        println!("List of vertices:");
        for &v in self.left_vertices.iter().chain(self.right_vertices.iter()) {
            println!("{}", v);
        }
        println!("List of edges:");
        for &(left, right) in &self.edges {
            println!("({}, {})", left, right);
        }
    }
}

fn main() {
    let graph = BipartiteGraph::new(
        vec!['A', 'B', 'C'],
        vec!['1', '2', '3', '4'],
        vec![('A', '1'), ('A', '2'), ('B', '2'), ('B', '3'), ('C', '4')]
    );

    graph.draw();
    println!();
    graph.set_enumeration();
    println!();
    graph.adjacency_matrix();
    println!();
    graph.incidence_matrix();
    println!();
    graph.edge_list();
    println!();
    graph.adjacency_vectors();
    println!();
    graph.adjacency_arrays();
    println!();
    graph.adjacency_lists();
    println!();
    graph.structure_with_toc();
    println!();
    graph.vertices_and_edges_lists();
}