use std::process;

fn display(matrix: &Vec<Vec<i32>>) {
    for i in 0..matrix.len() {
        if i != 0 && i % 3 == 0 {
            println!("---------------------");
        }
        for j in 0..matrix[0].len() {
            if j != 0 && j % 3 == 0 {
                print!("|");
            }
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
}

fn is_safe_place(matrix: &Vec<Vec<i32>>, row: usize, col: usize, data: i32) -> bool {
    for i in 0..9 {
        if matrix[i][col] == data {
            return false;
        }
        if matrix[row][i] == data {
            return false;
        }
    }
    let nrow = (row / 3) * 3;
    let ncol = (col / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if matrix[nrow + i][ncol + j] == data {
                return false;
            }
        }
    }
    true
}

fn sudoku(matrix: &mut Vec<Vec<i32>>, idx: usize) {
    if idx == 81 {
        display(matrix);
        process::exit(200);
    }

    let row = idx / 9;
    let col = idx % 9;
    if matrix[row][col] == 0 {
        for i in 1..=9 {
            if is_safe_place(matrix, row, col, i) {
                matrix[row][col] = i;
                sudoku(matrix, idx + 1);
                matrix[row][col] = 0;
            }
        }
    } else {
        sudoku(matrix, idx + 1);
    }
}

fn main() {
    // The World's hardest Sudoku Puzzle
    let mut matrix = vec![
        vec![1, 0, 0, 0, 0, 7, 0, 9, 0],
        vec![0, 3, 0, 0, 2, 0, 0, 0, 8],
        vec![0, 0, 9, 6, 0, 0, 5, 0, 0],
        vec![0, 0, 5, 3, 0, 0, 9, 0, 0],
        vec![0, 1, 0, 0, 8, 0, 0, 0, 2],
        vec![6, 0, 0, 0, 0, 4, 0, 0, 0],
        vec![3, 0, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 4, 0, 0, 0, 0, 0, 0, 7],
        vec![0, 0, 7, 0, 0, 0, 3, 0, 0],
    ];
    display(&matrix);
    println!("OutPut =>");
    sudoku(&mut matrix, 0);
}
