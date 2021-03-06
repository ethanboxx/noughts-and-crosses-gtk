use crate::{class, ButtonMatrix};
use gtk::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    X,
    O,
}
use Player::{O, X};

impl Player {
    fn show(self) -> &'static str {
        match self {
            X => "X",
            O => "O",
        }
    }
    fn swap(self) -> Self {
        match self {
            X => O,
            O => X,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    board: [[Option<Player>; 3]; 3],
    current: Player,
    end: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: [[None; 3]; 3],
            current: X,
            end: false,
        }
    }
    pub fn next(
        mut self,
        current_button: &gtk::Label,
        all_buttons: &ButtonMatrix,
        status: &gtk::Label,
        restart_button_style: &gtk::StyleContext,
        row: usize,
        column: usize,
    ) -> Self {
        if self.board[row][column].is_none() && !self.end {
            current_button.set_label(self.current.show());
            self.board[row][column] = Some(self.current);
            self.current = self.current.swap();
            match &self.winner(all_buttons, restart_button_style) {
                Some(player) => {
                    self.end = true;
                    status.set_markup(&format!("Player {} WINS!", player.show()))
                }
                None => {
                    if self.full() {
                        status.set_markup("Tie!");
                        restart_button_style.add_class(class::SHOULD_RESTART);
                        self.end = true;
                    } else {
                        status.set_markup(&format!("Player {} turn", self.current.show()))
                    }
                }
            };
            self
        } else if self.end {
            status.set_markup("Game has ended. Press restart.");
            self
        } else {
            status.set_markup("Tile already taken.");
            self
        }
    }
    // Functions used have side affects so should be called every time (not using `or_else`).
    #[allow(clippy::or_fun_call)]
    pub fn winner(
        &self,
        all_buttons: &ButtonMatrix,
        restart_button_style: &gtk::StyleContext,
    ) -> Option<Player> {
        let check = |x| self.check(x, all_buttons, restart_button_style);
        check([(0, 0), (1, 0), (2, 0)])
            .or(check([(0, 1), (1, 1), (2, 1)]))
            .or(check([(0, 2), (1, 2), (2, 2)]))
            .or(check([(0, 0), (0, 1), (0, 2)]))
            .or(check([(1, 0), (1, 1), (1, 2)]))
            .or(check([(2, 0), (2, 1), (2, 2)]))
            .or(check([(0, 0), (1, 1), (2, 2)]))
            .or(check([(2, 0), (1, 1), (0, 2)]))
    }
    fn check(
        &self,
        checks: [(usize, usize); 3],
        all_buttons: &ButtonMatrix,
        restart_button_style: &gtk::StyleContext,
    ) -> Option<Player> {
        for possible_winner in &[O, X] {
            if checks
                .iter()
                .map(|(x, y)| self.board[*x][*y])
                .all(|x| x == Some(*possible_winner))
            {
                for check in &checks {
                    all_buttons[check.0][check.1]
                        .get_style_context()
                        .add_class(class::WINNING_TILE);
                }
                restart_button_style.add_class(class::SHOULD_RESTART);
                return Some(*possible_winner);
            }
        }
        None
    }
    fn full(&self) -> bool {
        self.board.iter().flatten().all(Option::is_some)
    }
}
