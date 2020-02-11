use gtk::ToggleButton;
use crate::gtk::ButtonExt;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Block {
    pub value: u32,
    pub button: ToggleButton,
}


pub fn generate_board(board: &mut Vec<Vec<Block>>) {
    for _ in 0..4 {
        let mut row = Vec::new();
        for _ in 0..4 {
            let tmp_block = Block { value: 0, button: ToggleButton::new_with_label("") };
            gtk::WidgetExt::set_widget_name(&tmp_block.button, "btn");
            row.push(tmp_block);
        }
        board.push(row);
    }
    random_new_block(board.as_mut_slice());
    random_new_block(board.as_mut_slice());
}


fn random_new_block(board: &mut [Vec<Block>]) {
    let mut rng = rand::thread_rng();
    let mut possible_empty: Vec<u8> = (0..16).collect();
    let mut max = 16;
    let mut x: usize;
    let mut y: usize;

    while {
        let block_number = possible_empty[rng.gen_range(0, max)] as usize;
        let index = possible_empty.iter().position(|x| *x == block_number as u8).unwrap();
        possible_empty.remove(index);
        max -= 1;
        if possible_empty.is_empty() {
            return;
        }
        x = block_number / 4;
        y = block_number % 4;

        board[x][y].value != 0
    } {}

    let value = rng.gen_range(0, 10);
    if value < 7 {
        board[x][y].value = 2;
    } else {
        board[x][y].value = 4;
    }
}


pub fn update_board(board: &mut [Vec<Block>]) {
    for i in 0..16 {
        let x: usize = i / 4;
        let y: usize = i % 4;
        let value = board[x][y].value;
        if value > 0 {
            gtk::WidgetExt::set_widget_name(&board[x][y].button, "btn");
        } else {
            gtk::WidgetExt::set_widget_name(&board[x][y].button, "hidden");
        }
        board[x][y].button.set_label(&value.to_string());
    }
}


pub fn move_down(board: &mut [Vec<Block>]) {
    let mut moved = false;

    for column in 0..4 {
        for row in (0..4).rev() {
            for compare_row in (0..row).rev() {
                if board[column][compare_row].value == 0 {
                    continue;
                }
                if board[column][row].value == 0 {
                    board[column][row].value = board[column][compare_row].value;
                    board[column][compare_row].value = 0;
                    moved = true;
                    continue;
                } else if board[column][compare_row].value == board[column][row].value {
                    board[column][row].value += board[column][compare_row].value;
                    board[column][compare_row].value = 0;
                    moved = true;
                }
                break;
            }
        }
    }
    if moved {
        random_new_block(board);
        update_board(board);
    }
}


pub fn move_up(board: &mut [Vec<Block>]) {
    let mut moved = false;

    for column in 0..4 {
        for row in 0..4 {
            for compare_row in row + 1..4 {
                if board[column][compare_row].value == 0 {
                    continue;
                }
                if board[column][row].value == 0 {
                    board[column][row].value = board[column][compare_row].value;
                    board[column][compare_row].value = 0;
                    moved = true;
                    continue;
                } else if board[column][compare_row].value == board[column][row].value {
                    board[column][row].value += board[column][compare_row].value;
                    board[column][compare_row].value = 0;
                    moved = true;
                }
                break;
            }
        }
    }

    if moved {
        random_new_block(board);
        update_board(board);
    }
}


pub fn move_right(board: &mut [Vec<Block>]) {
    let mut moved = false;

    for row in 0..4 {
        for column in (0..4).rev() {
            for compare_column in (0..column).rev() {
                if board[compare_column][row].value == 0 {
                    continue;
                }
                if board[column][row].value == 0 {
                    board[column][row].value = board[compare_column][row].value;
                    board[compare_column][row].value = 0;
                    moved = true;
                    continue;
                } else if board[compare_column][row].value == board[column][row].value {
                    board[column][row].value += board[compare_column][row].value;
                    board[compare_column][row].value = 0;
                    moved = true;
                }
                break;
            }
        }
    }

    if moved {
        random_new_block(board);
        update_board(board);
    }
}


pub fn move_left(board: &mut [Vec<Block>]) {
    let mut moved = false;

    for row in 0..4 {
        for column in 0..4 {
            for compare_column in column + 1..4 {
                if board[compare_column][row].value == 0 {
                    continue;
                }
                if board[column][row].value == 0 {
                    board[column][row].value = board[compare_column][row].value;
                    board[compare_column][row].value = 0;
                    moved = true;
                    continue;
                } else if board[compare_column][row].value == board[column][row].value {
                    board[column][row].value += board[compare_column][row].value;
                    board[compare_column][row].value = 0;
                    moved = true;
                }
                break;
            }
        }
    }

    if moved {
        random_new_block(board);
        update_board(board);
    }
}


pub fn end_of_game(board: &mut [Vec<Block>]) -> bool {
    for row in 0..4 {
        for column in 0..4 {
            if board[column][row].value == 0 {
                return false;
            }
            if row > 0 && board[column][row].value == board[column][row - 1].value {
                return false;
            }
            if row < 3 && board[column][row].value == board[column][row + 1].value {
                return false;
            }
            if column > 0 && board[column][row].value == board[column - 1][row].value {
                return false;
            }
            if column < 3 && board[column][row].value == board[column + 1][row].value {
                return false;
            }
        }
    }
    true
}