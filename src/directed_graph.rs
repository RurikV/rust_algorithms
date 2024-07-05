use std::collections::{HashSet, VecDeque, HashMap};

#[derive(Debug)]
struct DirectedGraph {
    vertices: HashSet<usize>,
    edges: Vec<(usize, usize)>,
    adjacency_list: HashMap<usize, Vec<usize>>,
}

impl Clone for DirectedGraph {
    fn clone(&self) -> Self {
        DirectedGraph {
            vertices: self.vertices.clone(),
            edges: self.edges.clone(),
            adjacency_list: self.adjacency_list.clone(),
        }
    }
}

impl DirectedGraph {
    fn new() -> Self {
        DirectedGraph {
            vertices: HashSet::new(),
            edges: Vec::new(),
            adjacency_list: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: usize) {
        self.vertices.insert(vertex);
        self.adjacency_list.entry(vertex).or_insert(Vec::new());
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push((from, to));
        self.adjacency_list.entry(from).or_insert(Vec::new()).push(to);
    }

    // Breadth-First Search
    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(vertex) = queue.pop_front() {
            result.push(vertex);

            if let Some(neighbors) = self.adjacency_list.get(&vertex) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        result
    }

    // Depth-First Search
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.dfs_recursive(start, &mut visited, &mut result);
        result
    }

    fn dfs_recursive(&self, vertex: usize, visited: &mut HashSet<usize>, result: &mut Vec<usize>) {
        visited.insert(vertex);
        result.push(vertex);

        if let Some(neighbors) = self.adjacency_list.get(&vertex) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    self.dfs_recursive(neighbor, visited, result);
                }
            }
        }
    }

    // Kahn's Algorithm for Topological Sort
    fn topological_sort_kahn(&self) -> Vec<usize> {
        let mut in_degree = HashMap::new();
        for &vertex in &self.vertices {
            in_degree.insert(vertex, 0);
        }
        for &(_, to) in &self.edges {
            *in_degree.entry(to).or_insert(0) += 1;
        }

        let mut queue = VecDeque::new();
        for &vertex in &self.vertices {
            if in_degree[&vertex] == 0 {
                queue.push_back(vertex);
            }
        }

        let mut result = Vec::new();
        while let Some(vertex) = queue.pop_front() {
            result.push(vertex);
            if let Some(neighbors) = self.adjacency_list.get(&vertex) {
                for &neighbor in neighbors {
                    *in_degree.get_mut(&neighbor).unwrap() -= 1;
                    if in_degree[&neighbor] == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        result
    }

    // Tarjan's Algorithm for Topological Sort
    fn topological_sort_tarjan(&self) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        for &vertex in &self.vertices {
            if !visited.contains(&vertex) {
                self.tarjan_dfs(vertex, &mut visited, &mut stack);
            }
        }

        stack.reverse();
        stack
    }

    fn tarjan_dfs(&self, vertex: usize, visited: &mut HashSet<usize>, stack: &mut Vec<usize>) {
        visited.insert(vertex);

        if let Some(neighbors) = self.adjacency_list.get(&vertex) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    self.tarjan_dfs(neighbor, visited, stack);
                }
            }
        }

        stack.push(vertex);
    }

    // Demoucron's Algorithm for Topological Sort
    fn topological_sort_demoucron(&self) -> Vec<usize> {
        let mut result = Vec::new();
        let mut graph = self.clone();

        while !graph.vertices.is_empty() {
            let sources: Vec<usize> = graph.vertices
                .iter()
                .filter(|&v| graph.edges.iter().all(|&(_, to)| to != *v))
                .cloned()
                .collect();

            result.extend(sources.iter());

            for &source in &sources {
                graph.vertices.remove(&source);
                graph.edges.retain(|&(from, _)| from != source);
            }
        }

        result
    }

    fn display(&self) {
        println!("Graph visualization:");
        let mut levels: HashMap<usize, usize> = HashMap::new();
        self.assign_levels(0, &mut levels, 0);

        let max_level = *levels.values().max().unwrap_or(&0);
        for level in 0..=max_level {
            let vertices_at_level: Vec<_> = levels.iter()
                .filter(|(_, &l)| l == level)
                .map(|(&v, _)| v)
                .collect();

            // Print vertices
            for &v in &vertices_at_level {
                print!("{:2}", v);
                print!("{}   ", " ".repeat(3));
            }
            println!();

            // Print edges
            for &v in &vertices_at_level {
                if let Some(neighbors) = self.adjacency_list.get(&v) {
                    if !neighbors.is_empty() {
                        print!(" |");
                    } else {
                        print!("  ");
                    }
                    print!("{}   ", " ".repeat(3));
                }
            }
            println!();

            for &v in &vertices_at_level {
                if let Some(neighbors) = self.adjacency_list.get(&v) {
                    if !neighbors.is_empty() {
                        print!(" v");
                    } else {
                        print!("  ");
                    }
                    print!("{}   ", " ".repeat(3));
                }
            }
            println!();
        }
    }

    fn assign_levels(&self, vertex: usize, levels: &mut HashMap<usize, usize>, current_level: usize) {
        if let Some(&level) = levels.get(&vertex) {
            if level <= current_level {
                return;
            }
        }
        levels.insert(vertex, current_level);
        if let Some(neighbors) = self.adjacency_list.get(&vertex) {
            for &neighbor in neighbors {
                self.assign_levels(neighbor, levels, current_level + 1);
            }
        }
    }
}

// Implementation for the graph with strongly connected components
struct SCCGraph {
    graph: DirectedGraph,
    transpose: DirectedGraph,
}

impl SCCGraph {
    fn new() -> Self {
        SCCGraph {
            graph: DirectedGraph::new(),
            transpose: DirectedGraph::new(),
        }
    }

    fn add_vertex(&mut self, vertex: usize) {
        self.graph.add_vertex(vertex);
        self.transpose.add_vertex(vertex);
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.graph.add_edge(from, to);
        self.transpose.add_edge(to, from);
    }

    // Kosaraju's Algorithm for Strongly Connected Components
    fn kosaraju(&self) -> Vec<Vec<usize>> {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        // First DFS to fill the stack
        for &vertex in &self.graph.vertices {
            if !visited.contains(&vertex) {
                self.fill_order(vertex, &mut visited, &mut stack);
            }
        }

        // Second DFS on transpose graph
        visited.clear();
        let mut components = Vec::new();

        while let Some(vertex) = stack.pop() {
            if !visited.contains(&vertex) {
                let mut component = Vec::new();
                self.dfs_transpose(vertex, &mut visited, &mut component);
                components.push(component);
            }
        }

        components
    }

    fn fill_order(&self, vertex: usize, visited: &mut HashSet<usize>, stack: &mut Vec<usize>) {
        visited.insert(vertex);

        if let Some(neighbors) = self.graph.adjacency_list.get(&vertex) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    self.fill_order(neighbor, visited, stack);
                }
            }
        }

        stack.push(vertex);
    }

    fn dfs_transpose(&self, vertex: usize, visited: &mut HashSet<usize>, component: &mut Vec<usize>) {
        visited.insert(vertex);
        component.push(vertex);

        if let Some(neighbors) = self.transpose.adjacency_list.get(&vertex) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    self.dfs_transpose(neighbor, visited, component);
                }
            }
        }
    }
}

fn main() {
    // Create a DAG with 10 vertices and 10 edges
    let mut dag = DirectedGraph::new();
    for i in 0..10 {
        dag.add_vertex(i);
    }
    dag.add_edge(0, 1);
    dag.add_edge(0, 2);
    dag.add_edge(1, 3);
    dag.add_edge(1, 4);
    dag.add_edge(2, 5);
    dag.add_edge(3, 6);
    dag.add_edge(4, 7);
    dag.add_edge(5, 7);
    dag.add_edge(6, 8);
    dag.add_edge(7, 9);

    dag.display();

    println!("BFS: {:?}", dag.bfs(0));
    println!("DFS: {:?}", dag.dfs(0));
    println!("Topological Sort (Kahn): {:?}", dag.topological_sort_kahn());
    println!("Topological Sort (Tarjan): {:?}", dag.topological_sort_tarjan());
    println!("Topological Sort (Demoucron): {:?}", dag.topological_sort_demoucron());

    // Create a graph with three strongly connected components
    let mut scc_graph = SCCGraph::new();
    for i in 0..9 {
        scc_graph.add_vertex(i);
    }
    // Component 1
    scc_graph.add_edge(0, 1);
    scc_graph.add_edge(1, 2);
    scc_graph.add_edge(2, 0);
    // Component 2
    scc_graph.add_edge(3, 4);
    scc_graph.add_edge(4, 5);
    scc_graph.add_edge(5, 3);
    // Component 3
    scc_graph.add_edge(6, 7);
    scc_graph.add_edge(7, 8);
    scc_graph.add_edge(8, 6);
    // Connections between components
    scc_graph.add_edge(2, 3);
    scc_graph.add_edge(5, 6);

    println!("Strongly Connected Components: {:?}", scc_graph.kosaraju());
}