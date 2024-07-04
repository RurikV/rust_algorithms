fn main() {
    let set_a = vec!["A1", "A2", "A3"];
    let set_b = vec!["B1", "B2", "B3", "B4"];
    
    let edges = vec![
        ("A1", "B1"),
        ("A2", "B2"),
        ("A3", "B3"),
        ("A1", "B4"),
        ("A2", "B3"),
    ];
    
    // Drawing the bipartite graph
    draw_bipartite_graph(&set_a, &set_b, &edges);
    
    // Displaying the adjacency matrix
    let adjacency_matrix = vec![
        vec![1, 0, 0, 1], // A1 -> B1, B4
        vec![0, 1, 1, 0], // A2 -> B2, B3
        vec![0, 0, 1, 0], // A3 -> B3
    ];
    display_adjacency_matrix(&adjacency_matrix);
    
    // Displaying the incidence matrix
    let incidence_matrix = vec![
        vec![1, 0, 0, 1, 0], // A1
        vec![0, 1, 0, 0, 1], // A2
        vec![0, 0, 1, 0, 0], // A3
    ];
    display_incidence_matrix(&incidence_matrix);
    
    // Displaying the adjacency lists
    let adjacency_lists = vec![
        ("A1", vec!["B1", "B4"]),
        ("A2", vec!["B2", "B3"]),
        ("A3", vec!["B3"]),
    ];
    display_adjacency_lists(&adjacency_lists);
}

fn draw_bipartite_graph(set_a: &Vec<&str>, set_b: &Vec<&str>, edges: &Vec<(&str, &str)>) {
    println!("Bipartite Graph A(3,4) with 5 edges:");
    for &a in set_a {
        print!("{} ", a);
        for &b in set_b {
            if edges.contains(&(a, b)) {
                print!("-- {} ", b);
            }
        }
        println!();
    }
}

fn display_adjacency_matrix(matrix: &Vec<Vec<u8>>) {
    println!("\nAdjacency Matrix:");
    for row in matrix {
        for &val in row {
            print!("{} ", val);
        }
        println!();
    }
}

fn display_incidence_matrix(matrix: &Vec<Vec<u8>>) {
    println!("\nIncidence Matrix:");
    for row in matrix {
        for &val in row {
            print!("{} ", val);
        }
        println!();
    }
}

fn display_adjacency_lists(lists: &Vec<(&str, Vec<&str>)>) {
    println!("\nAdjacency Lists:");
    for &(vertex, ref neighbors) in lists {
        print!("{}: ", vertex);
        for &neighbor in neighbors {
            print!("{} ", neighbor);
        }
        println!();
    }
}
