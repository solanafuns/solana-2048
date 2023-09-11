use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub board: Vec<Vec<u16>>,
    pub steps: u16,
}

impl Game {
    pub fn init(&mut self) {
        self.board = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        self.steps = 0;
        self.add_block();
        self.add_block();
    }

    pub fn move_up(&mut self) {
        for col in 0..4 {
            let mut merged = [false; 4];
            for row in 1..4 {
                if self.board[row][col] != 0 {
                    let mut current_row = row;
                    while current_row > 0 {
                        let prev_row = current_row - 1;
                        if self.board[prev_row][col] == 0 {
                            self.board[prev_row][col] = self.board[current_row][col];
                            self.board[current_row][col] = 0;
                            current_row = prev_row;
                        } else if self.board[prev_row][col] == self.board[current_row][col]
                            && !merged[prev_row]
                            && !merged[current_row]
                        {
                            self.board[prev_row][col] *= 2;
                            self.board[current_row][col] = 0;
                            merged[prev_row] = true;
                            break;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        self.add_block();
    }

    pub fn move_down(&mut self) {
        for col in 0..4 {
            let mut merged = [false; 4];
            for row in (0..3).rev() {
                if self.board[row][col] != 0 {
                    let mut current_row = row;
                    while current_row < 3 {
                        let next_row = current_row + 1;
                        if self.board[next_row][col] == 0 {
                            self.board[next_row][col] = self.board[current_row][col];
                            self.board[current_row][col] = 0;
                            current_row = next_row;
                        } else if self.board[next_row][col] == self.board[current_row][col]
                            && !merged[next_row]
                            && !merged[current_row]
                        {
                            self.board[next_row][col] *= 2;
                            self.board[current_row][col] = 0;
                            merged[next_row] = true;
                            break;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        self.add_block();
    }

    pub fn move_left(&mut self) {
        for row in 0..4 {
            let mut merged = [false; 4];
            for col in 1..4 {
                if self.board[row][col] != 0 {
                    let mut current_col = col;
                    while current_col > 0 {
                        let prev_col = current_col - 1;
                        if self.board[row][prev_col] == 0 {
                            self.board[row][prev_col] = self.board[row][current_col];
                            self.board[row][current_col] = 0;
                            current_col = prev_col;
                        } else if self.board[row][prev_col] == self.board[row][current_col]
                            && !merged[prev_col]
                            && !merged[current_col]
                        {
                            self.board[row][prev_col] *= 2;
                            self.board[row][current_col] = 0;
                            merged[prev_col] = true;
                            break;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        self.add_block();
    }

    pub fn move_right(&mut self) {
        for row in 0..4 {
            let mut merged = [false; 4];
            for col in (0..3).rev() {
                if self.board[row][col] != 0 {
                    let mut current_col = col;
                    while current_col < 3 {
                        let next_col = current_col + 1;
                        if self.board[row][next_col] == 0 {
                            self.board[row][next_col] = self.board[row][current_col];
                            self.board[row][current_col] = 0;
                            current_col = next_col;
                        } else if self.board[row][next_col] == self.board[row][current_col]
                            && !merged[next_col]
                            && !merged[current_col]
                        {
                            self.board[row][next_col] *= 2;
                            self.board[row][current_col] = 0;
                            merged[next_col] = true;
                            break;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        self.add_block();
    }

    fn add_block(&mut self) {
        let clock = Clock::get().unwrap();
        let current_time = clock.unix_timestamp;

        let empty_cells: Vec<(usize, usize)> = (0..4)
            .flat_map(|row| (0..4).map(move |col| (row, col)))
            .filter(|&(row, col)| self.board[row][col] == 0)
            .collect();

        if empty_cells.is_empty() {
            // 游戏板已满，无法添加新块
            return;
        }

        let (row, col) = empty_cells[(current_time as usize) % empty_cells.len()];
        // 根据时间戳生成伪随机的值（示例中是1或2）
        let new_value = if current_time % 2 == 0 { 2 } else { 4 };

        self.board[row][col] = new_value;

        msg!(
            "new block at row = {}, col = {}, value = {}",
            row,
            col,
            new_value
        );
    }
}
