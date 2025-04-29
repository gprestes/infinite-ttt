use std::collections::VecDeque;
use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    X,
    O,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

#[derive(Clone)]
pub struct Game {
    pub board: [[Cell; 3]; 3],
    pub moves_x: VecDeque<(usize, usize)>,
    pub moves_o: VecDeque<(usize, usize)>,
    pub current_player: Player,
    pub winner: Option<Player>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: [[Cell::Empty; 3]; 3],
            moves_x: VecDeque::new(),
            moves_o: VecDeque::new(),
            current_player: Player::X,
            winner: None,
        }
    }

    pub fn play(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        if self.winner.is_some() {
            return Err("Game ended");
        }

        if row > 2 || col > 2 {
            return Err("Invalid position");
        }

        if self.board[row][col] != Cell::Empty {
            return Err("Position already occupied");
        }

        let moves = match self.current_player {
            Player::X => &mut self.moves_x,
            Player::O => &mut self.moves_o,
        };

        if moves.len() == 3 {
            let (old_r, old_c) = moves.pop_front().unwrap();
            self.board[old_r][old_c] = Cell::Empty;
        }

        self.board[row][col] = Cell::Occupied(self.current_player);
        moves.push_back((row, col));

        if self.check_winner(self.current_player) {
            self.winner = Some(self.current_player);
        } else {
            self.current_player = match self.current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
        }

        Ok(())
    }

    fn check_winner(&self, player: Player) -> bool {
        let p = Cell::Occupied(player);
        let b = &self.board;

        (0..3).any(|i| b[i][0] == p && b[i][1] == p && b[i][2] == p) ||
        (0..3).any(|i| b[0][i] == p && b[1][i] == p && b[2][i] == p) ||
        (b[0][0] == p && b[1][1] == p && b[2][2] == p) ||
        (b[0][2] == p && b[1][1] == p && b[2][0] == p)
    }

    pub fn play_ai_move(&mut self) -> Result<(), &'static str> {
        if self.winner.is_some() {
            return Err("Game ended");
        }

        if self.current_player != Player::O {
            return Err("Not the AI turn");
        }

        let mut empty_cells = vec![];
        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == Cell::Empty {
                    empty_cells.push((row, col));
                }
            }
        }

        // 1. Play to Win
        for &(row, col) in &empty_cells {
            let mut clone = self.clone();
            clone.play(row, col).ok();
            if clone.winner == Some(Player::O) {
                return self.play(row, col);
            }
        }

        // 2. Block Player X
        for &(row, col) in &empty_cells {
            let mut clone = self.clone();
            clone.current_player = Player::X;
            clone.play(row, col).ok();
            if clone.winner == Some(Player::X) {
                return self.play(row, col);
            }
        }

        // 3. Random play
        let mut rng = rand::thread_rng();
        let &(row, col) = empty_cells.choose(&mut rng).unwrap();
        self.play(row, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_starts_with_empty_board() {
        let game = Game::new();
        for row in 0..3 {
            for col in 0..3 {
                assert_eq!(game.board[row][col], Cell::Empty);
            }
        }
        assert_eq!(game.current_player, Player::X);
        assert_eq!(game.winner, None);
    }

    #[test]
    fn test_play_and_check_winner_row() {
        let mut game = Game::new();
        game.play(0, 0).unwrap(); // X
        game.play(1, 0).unwrap(); // O
        game.play(0, 1).unwrap(); // X
        game.play(1, 1).unwrap(); // O
        game.play(0, 2).unwrap(); // X wins

        assert_eq!(game.winner, Some(Player::X));
    }
}