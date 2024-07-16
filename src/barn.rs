use std::io::{self, BufRead};
use std::cmp;

fn main() {
    let (n, m, grid) = read_input();
    let max_area = largest_rectangle_area(&grid);
    println!("Maximum area of the barn: {}", max_area);
    print_ascii_barn(&grid, max_area);
}

fn read_input() -> (usize, usize, Vec<Vec<i32>>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read N and M
    let nm: Vec<usize> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (n, m) = (nm[0], nm[1]);

    // Initialize the grid
    let mut grid = vec![vec![0; m]; n];

    // Read number of obstacles
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read and mark obstacles
    for _ in 0..t {
        let xy: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        grid[xy[0]][xy[1]] = 1;
    }

    (n, m, grid)
}

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
    let mut found = false;
    for i in 0..n {
        for j in 0..m {
            if !found {
                let (height, width) = find_largest_barn(grid, i, j, max_area);
                if height * width == max_area {
                    mark_barn(&mut ascii_grid, i, j, height, width);
                    found = true;
                    break;
                }
            }
        }
        if found {
            break;
        }
    }

    // Print the ASCII grid
    for row in ascii_grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn find_largest_barn(grid: &Vec<Vec<i32>>, i: usize, j: usize, max_area: i32) -> (i32, i32) {
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    let mut height = (max_area as f64).sqrt().ceil() as i32;
    let mut width = max_area / height;

    while height > 0 && width > 0 {
        if i as i32 + height <= n && j as i32 + width <= m {
            let mut fits = true;
            for x in i..i + height as usize {
                for y in j..j + width as usize {
                    if grid[x][y] == 1 {
                        fits = false;
                        break;
                    }
                }
                if !fits {
                    break;
                }
            }
            if fits {
                return (height, width);
            }
        }
        height -= 1;
        width = max_area / height;
    }
    (0, 0)
}

fn mark_barn(ascii_grid: &mut Vec<Vec<char>>, i: usize, j: usize, height: i32, width: i32) {
    for x in i..i + height as usize {
        for y in j..j + width as usize {
            ascii_grid[x][y] = '*';
        }
    }
}

fn can_fit_barn(grid: &Vec<Vec<i32>>, i: usize, j: usize, area: i32) -> bool {
    let n = grid.len();
    let m = grid[0].len();
    let height = (area as f64).sqrt().ceil() as usize;
    let width = area as usize / height;

    if i + height > n || j + width > m {
        return false;
    }

    for x in i..i + height {
        for y in j..j + width {
            if grid[x][y] == 1 {
                return false;
            }
        }
    }

    true
}

// fn mark_barn(ascii_grid: &mut Vec<Vec<char>>, i: usize, j: usize, area: i32) {
//     let height = (area as f64).sqrt().ceil() as usize;
//     let width = area as usize / height;

//     for x in i..i + height {
//         for y in j..j + width {
//             ascii_grid[x][y] = '*';
//         }
//     }
// }

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