use std::fmt;
use std::fs;

#[derive(Debug)]
struct Grid(Vec<Vec<bool>>); // true == tree

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for has_tree in row {
                let mut c = '.';
                if *has_tree {
                    c = '#';
                }
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    // load
    let mut grid = Grid(Vec::new());
    for row in fs::read_to_string("input.txt").unwrap().lines() {
        let mut r = Vec::new();
        for c in row.chars() {
            r.push(c == '#');
        }
        grid.0.push(r);
    }
    let mut result = Vec::new();
    result.push(num_trees(1, 1, &grid));
    result.push(num_trees(3, 1, &grid));
    result.push(num_trees(5, 1, &grid));
    result.push(num_trees(7, 1, &grid));
    result.push(num_trees(1, 2, &grid));
    println!("{:?}", result);

    let m = result.iter().product::<usize>();
    println!("{}", m);
}

fn num_trees(right: usize, down: usize, grid: &Grid) -> usize {
    let width = grid.0[0].len();
    let mut num_trees = 0;
    let mut row = 0;
    let mut col = 0;
    while row < grid.0.len() {
        if grid.0[row][col % width] {
            num_trees += 1;
        }
        col += right;
        row += down;
    }
    num_trees
}
