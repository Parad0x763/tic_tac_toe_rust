const EMPTY_SPACE: char = ' ';

struct Board {
    board: [char; 9],
    is_cross: bool,
}

impl Board {
    fn new() -> Self {
        let default_board: [char; 9] = [
            EMPTY_SPACE, EMPTY_SPACE, EMPTY_SPACE,
            EMPTY_SPACE, EMPTY_SPACE, EMPTY_SPACE,
            EMPTY_SPACE, EMPTY_SPACE, EMPTY_SPACE,
        ];
        return Board { board: default_board, is_cross: true };
    }

    fn action(&mut self, input: usize) {
        if input > 0 && input < 10 {
            if self.board[input - 1] == ' ' {
                self.board[input - 1] = if self.is_cross { 'X' } else { 'O' };
                self.is_cross = !self.is_cross;
            }
        }
    }

    fn completed_all_turns(&self) -> bool {
        for index in 0..9 {
            if self.board[index] == ' ' {
                return false;
            }
        }
        return true;
    }

    fn game_won(&self) -> bool {
        // win conditions
        /*
            same symbols at 0,1,2; 3,4,5; 6,7,8; 0,4,8; 2,4,6, 0,3,6; 1,4,7; 2,5,8;
        */
        // game ends when on win or all spots filled
        if (self.board[0] == self.board[1] && self.board[1] == self.board[2] && self.board[0] != EMPTY_SPACE) ||
            (self.board[3] == self.board[4] && self.board[4] == self.board[5] && self.board[3] != EMPTY_SPACE) ||
            (self.board[6] == self.board[7] && self.board[7] == self.board[8] && self.board[6] != EMPTY_SPACE) ||
            (self.board[0] == self.board[4] && self.board[4] == self.board[8] && self.board[0] != EMPTY_SPACE) ||
            (self.board[2] == self.board[4] && self.board[4] == self.board[6] && self.board[2] != EMPTY_SPACE) ||
            (self.board[0] == self.board[3] && self.board[3] == self.board[6] && self.board[0] != EMPTY_SPACE) ||
            (self.board[1] == self.board[4] && self.board[4] == self.board[7] && self.board[1] != EMPTY_SPACE) ||
            (self.board[2] == self.board[5] && self.board[5] == self.board[8] && self.board[2] != EMPTY_SPACE) {
                return true;
        }
        return false;
    }

    fn print_board(&self) {
        print!("
| {} {} {} |\n
| {} {} {} |\n
| {} {} {} |
        ", self.board[0], self.board[1], self.board[2],
            self.board[3], self.board[4], self.board[5],
            self.board[6], self.board[7], self.board[8]
        );
    }
}

fn main() {
    let mut user_input: String = String::new();
    let mut game: Board = Board::new();

    println!("Start game\nEnter 1..9");
    loop {
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("An error occurred reading input");

        let trimmed_input = user_input.trim();

        match trimmed_input.parse::<usize>() {
            Ok(input) => game.action(input),
            Err(e) => {
                println!("Error, please input 1..9");
                continue;
            },
        };

        user_input.clear();
        
        game.print_board();

        if game.game_won() || game.completed_all_turns() {
            println!("Game over!");
            break;
        }
    }
}
