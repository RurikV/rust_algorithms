use std::io::{self, Write, BufRead};
use std::cmp;

#[allow(dead_code)]
fn main() {
    println!("Welcome to the Maximum Barn Area Calculator!");
    println!("Follow the instructions to input data about your farm.");
    println!();

    let (_n, _m, grid) = read_input();
    let max_area = largest_rectangle_area(&grid);
    
    println!();
    println!("Results:");
    println!("Maximum barn area: {}", max_area);
    println!();
    
    if max_area > 0 {
        println!("Visualization of the farm and maximum barn:");
        print_ascii_barn(&grid, max_area);
    } else {
        println!("There is no space available for a barn on this farm.");
    }
}

fn read_input() -> (usize, usize, Vec<Vec<i32>>) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut lines = stdin.lock().lines();

    // Read N and M
    println!("Enter the matrix size N M (space-separated, 1 <= N, M <= 1000): ");
    stdout.flush().unwrap();
    let nm: Vec<usize> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (n, m) = (nm[0], nm[1]);

    // Initialize the grid
    let mut grid = vec![vec![0; m]; n];

    // Read number of obstacles
    println!("Enter the number of buildings T (0 <= T <= 10000): ");
    stdout.flush().unwrap();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read and mark obstacles
    println!("Enter the coordinates of buildings (two numbers X Y per line, where 0 <= X < N and 0 <= Y < M):");
    for i in 1..=t {
        println!("Building {}: ", i);
        stdout.flush().unwrap();
        let xy: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        grid[xy[0]][xy[1]] = 1;
    }

    (n, m, grid)
}

#[allow(dead_code)]
fn largest_rectangle_area(grid: &Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut heights = vec![0; m];
    let mut max_area = 0;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                heights[j] = 0;
            } else {
                heights[j] += 1;
            }
        }
        max_area = cmp::max(max_area, largest_rectangle_histogram(&heights));
    }

    max_area
}

fn largest_rectangle_histogram(heights: &Vec<i32>) -> i32 {
    let mut stack = Vec::new();
    let mut max_area = 0;
    let n = heights.len();

    for i in 0..=n {
        let h = if i == n { 0 } else { heights[i] };
        while !stack.is_empty() && h < heights[*stack.last().unwrap()] {
            let height = heights[stack.pop().unwrap()];
            let width = if stack.is_empty() {
                i as i32
            } else {
                (i - stack.last().unwrap() - 1) as i32
            };
            max_area = cmp::max(max_area, height * width);
        }
        stack.push(i);
    }

    max_area
}

#[allow(dead_code)]
fn print_ascii_barn(grid: &Vec<Vec<i32>>, max_area: i32) {
    let n = grid.len();
    let m = grid[0].len();
    let mut ascii_grid = vec![vec!['.'; m]; n];

    // Mark obstacles
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                ascii_grid[i][j] = '#';
            }
        }
    }

    // Find and mark the largest barn
    if max_area > 0 {
        'outer: for i in 0..n {
            for j in 0..m {
                if let Some((height, width)) = find_largest_barn(grid, i, j, max_area) {
                    mark_barn(&mut ascii_grid, i, j, height, width);
                    break 'outer;
                }
            }
        }
    }

    // Print the ASCII grid
    println!("Legend: '.' - empty cell, '#' - obstacle, '*' - largest possible barn");
    println!();
    for row in ascii_grid {
        println!("{}", row.iter().collect::<String>());
    }
}

#[allow(dead_code)]
fn find_largest_barn(grid: &Vec<Vec<i32>>, i: usize, j: usize, max_area: i32) -> Option<(i32, i32)> {
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    
    for height in (1..=max_area).rev() {
        if max_area % height == 0 {
            let width = max_area / height;
            if i as i32 + height <= n && j as i32 + width <= m {
                if can_fit_barn(grid, i, j, height, width) {
                    return Some((height, width));
                }
            }
        }
    }
    None
}
#[allow(dead_code)]
fn can_fit_barn(grid: &Vec<Vec<i32>>, i: usize, j: usize, height: i32, width: i32) -> bool {
    for x in i..i + height as usize {
        for y in j..j + width as usize {
            if grid[x][y] == 1 {
                return false;
            }
        }
    }
    true
}

fn mark_barn(ascii_grid: &mut Vec<Vec<char>>, i: usize, j: usize, height: i32, width: i32) {
    for x in i..i + height as usize {
        for y in j..j + width as usize {
            ascii_grid[x][y] = '*';
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_grid() {
        let grid = vec![
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 0],
        ];
        assert_eq!(largest_rectangle_area(&grid), 3);
    }

    #[test]
    fn test_large_grid() {
        let grid = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(largest_rectangle_area(&grid), 10);
    }

    #[test]
    fn test_full_grid() {
        let grid = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        assert_eq!(largest_rectangle_area(&grid), 9);
    }

    #[test]
    fn test_blocked_grid() {
        let grid = vec![
            vec![1, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 1],
        ];
        assert_eq!(largest_rectangle_area(&grid), 0);
    }
}