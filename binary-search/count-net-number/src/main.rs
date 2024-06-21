fn main() {
    let grid = vec![
        vec![4, 3, 2, -1],
        vec![3, 2, 1, -1],
        vec![1, 1, -1, -2],
        vec![-1, -1, -2, -3],
    ];

    println!("Number of negatives in grid1: {}", count_negatives(grid));
}


pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {

    if !(grid.len() >= 1 && grid[0].len() <= 100) { return 0 }

    let mut count = 0;
    let mut row = grid.len();
    let mut col = 0;

    let col_len = grid[0].len();

    while row > 0 && col < col_len {

        if !(-100i32 <= grid[row-1][col] && grid[row-1][col] <= 100) { return 0 }

        if grid[row - 1][col] < 0 {
            count += col_len - col;
            row -= 1;
        } else if col+1 < col_len {
            col += 1;
        } else {
            row -= 1;
        }
    }

    count as i32
}