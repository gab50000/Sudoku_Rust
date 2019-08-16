use std::collections::BTreeSet;

fn get_possible_values(board: &[[i32; 9]; 9], i: usize, j: usize) -> BTreeSet<i32> {
    let mut values: BTreeSet<_> = (1..=9).collect();

    // iterate over rows
    for ii in 0..9 {
        if ii != i {
            values.remove(&board[ii][j]);
        }
    }

    // iterate over columns
    for jj in 0..9 {
        if jj != j {
            values.remove(&board[i][jj]);
        }
    }

    let m = (i / 3) * 3;
    let n = (j / 3) * 3;

    // check values in subfield
    for ii in m..m + 3 {
        for jj in n..n + 3 {
            if ii != i || jj != j {
                values.remove(&board[ii][jj]);
            }
        }
    }
    return values;
}

fn fill_board_recursively(board: [[i32; 9]; 9], i: usize, j: usize) -> Option<[[i32; 9]; 9]> {
    if i == 8 && j == 8 {
        return Some(board);
    }

    let next_i;
    let next_j;

    if j + 1 == 9 {
        next_i = i + 1;
        next_j = 0
    } else {
        next_i = i;
        next_j = j + 1;
    }

    let values = get_possible_values(&board, next_i, next_j);
    let mut new_board: [[i32; 9]; 9] = [[0; 9]; 9];
    new_board.copy_from_slice(&board);

    for val in values {
        new_board[next_i][next_j] = val;
        let result = fill_board_recursively(new_board, next_i, next_j);
        match result {
            Some(result) => return Some(result),
            None => (),
        }
    }

    return None;
}

fn main() {
    let mut board = [[0; 9]; 9];
    board[0][0] = 1;
    let result = fill_board_recursively(board, 0, 0).unwrap();

    for i in 0..9 {
        for j in 0..9 {
            print!("{} ", result[i][j]);
        }
        println!("");
    }
}
