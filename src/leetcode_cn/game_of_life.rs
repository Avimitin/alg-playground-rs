#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        if board.is_empty() {
            return;
        }

        let neighbor_index = vec![
            vec![-1, -1], // top left
            vec![0, -1],  // top
            vec![1, -1],  // top right
            vec![-1, 0],  // left
            vec![1, 0],   // right
            vec![-1, 1],  // bottom left
            vec![0, 1],   // bottom
            vec![1, 1],   // bottom right
        ];
        let is_valid = |i: i32, limit: i32| i >= 0 && i < limit;

        let row = board.len() as i32;
        let col = board[0].len() as i32;

        // 0 dead in both old and new metrix
        // 1 alive in both old and new metrix
        // 2 dead in old metrix, alive in new metrix
        // -1 alive in old metrix, dead in new metrix
        for i in 0..row {
            for j in 0..col {
                let mut live_neighbor = 0;
                for neighbor in &neighbor_index {
                    // neighbor row and column index
                    let nb_r = i + neighbor[0];
                    let nb_c = j + neighbor[1];
                    // if neighbor is accessable
                    if is_valid(nb_r, row) && is_valid(nb_c, col) {
                        let nb_r = nb_r as usize;
                        let nb_c = nb_c as usize;
                        // if this neighbor is alive, or *was* alive.
                        live_neighbor += if board[nb_r][nb_c] == 1 || board[nb_r][nb_c] == -1 {
                            1
                        } else {
                            0
                        }
                    }
                }

                // if the current node is dead, and has three alive neighbor
                let r = i as usize;
                let c = j as usize;
                // 4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
                if board[r][c] == 0 && live_neighbor == 3 {
                    board[r][c] = 2;
                } else if board[r][c] == 1 {
                    // 1. Any live cell with fewer than two live neighbors dies as if caused by under-population.
                    // 2. Any live cell with two or three live neighbors lives on to the next generation.
                    // 3. Any live cell with more than three live neighbors dies, as if by over-population.
                    if live_neighbor < 2 || live_neighbor > 3 {
                        board[r][c] = -1;
                    }
                }
            }
        }

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == -1 {
                    board[i][j] = 0;
                } else if board[i][j] == 2 {
                    board[i][j] = 1;
                }
            }
        }
    }
}

#[test]
fn test() {}
