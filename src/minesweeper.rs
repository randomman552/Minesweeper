use rand::prelude::*;
use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

pub type Position = (u16, u16);

/// Result of opening a minesweeper field
pub enum OpenResult {
    Mine,
    NoMine(u8),
}

/// Display implementation for [OpenResult]
impl Display for OpenResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let &OpenResult::Mine = self {
            f.write_str("Mine")?;
        }
        if let &OpenResult::NoMine(count) = self {
            write!(f, "NoMine({})", count)?;
        }

        Ok(())
    }
}

/// Enum listing the possible states of the game
#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    Win,
    Loss,
}

/// Enum listing the possible states of a field
pub enum FieldState {
    Unknown,
    MineRevealed,
    NoMine,
    MineDetonated,
    Open(u8),
    Flagged,
    Question,
}

/// Minesweeper game implementation
#[derive(Debug)]
pub struct Minesweeper {
    pub width: u16,
    pub height: u16,
    pub game_state: GameState,
    opened: HashSet<Position>,
    flagged: HashSet<Position>,
    question: HashSet<Position>,
    mines: HashSet<Position>,
}

impl Minesweeper {
    pub fn new(width: u16, height: u16, num_mines: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            opened: HashSet::new(),
            flagged: HashSet::new(),
            question: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();
                let mut rng = rand::rng();

                while mines.len() < num_mines {
                    let pos = (rng.random_range(0..width), rng.random_range(0..height));
                    mines.insert(pos);
                }

                mines
            },
            game_state: GameState::InProgress,
        }
    }

    // region Position checks

    pub fn is_mined(&self, pos: Position) -> bool {
        self.mines.contains(&pos)
    }

    pub fn is_open(&self, pos: Position) -> bool {
        self.opened.contains(&pos)
    }

    pub fn is_flagged(&self, pos: Position) -> bool {
        self.flagged.contains(&pos)
    }

    pub fn is_question(&self, pos: Position) -> bool {
        self.question.contains(&pos)
    }

    pub fn is_in_bounds(&self, pos: Position) -> bool {
        pos.0 < self.width && pos.1 < self.height
    }

    /// Get the state of the field with the given position
    pub fn get_field_state(&self, pos: Position) -> FieldState {
        if self.game_state == GameState::InProgress {
            // Show flagged field
            if self.is_flagged(pos) {
                return FieldState::Flagged;
            }

            // Show question field
            if self.is_question(pos) {
                return FieldState::Question;
            }
        } else {
            if self.is_mined(pos) {
                // Player opened a mine, whoops
                if self.is_open(pos) {
                    return FieldState::MineDetonated;
                }
                return FieldState::MineRevealed;
            }
            // Player falsely flagged this as a mine
            else if self.is_flagged(pos) {
                return FieldState::NoMine;
            }

            // Player incorrectly flagged this was a mine
            if !self.is_mined(pos) && self.is_flagged(pos) {
                return FieldState::NoMine;
            }
        }

        // Show open field
        if self.is_open(pos) {
            let mine_count = self.neighboring_mines(pos);
            return FieldState::Open(mine_count);
        }

        return FieldState::Unknown;
    }

    // endregion

    // region Win/loss condition checks

    fn check_game_state(&mut self) -> GameState {
        // Check for opened mines
        if self.mines.intersection(&self.opened).count() > 0 {
            self.game_state = GameState::Loss;
            return GameState::Loss;
        }

        // Player wins once all fields without a mine have been revealed
        if self.opened.len() == usize::from(self.width * self.height) - self.mines.len() {
            self.game_state = GameState::Win;
            return GameState::Win;
        }

        return GameState::InProgress;
    }

    /// Get the remaining number of mines (according to the player)
    pub fn remaining_mines(&self) -> usize {
        if self.mines.len() > self.flagged.len() {
            return self.mines.len() - self.flagged.len();
        }
        return self.flagged.len() - self.mines.len();
    }

    pub fn has_started(&self) -> bool {
        return self.opened.len() > 0;
    }

    // endregion

    // region Neighboring fields methods

    pub fn neighboring_fields_iter(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let height = self.height;
        let width = self.width;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    pub fn neighboring_mines(&self, pos: Position) -> u8 {
        self.neighboring_fields_iter(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn neighboring_flags(&self, pos: Position) -> u8 {
        self.neighboring_fields_iter(pos)
            .filter(|pos| self.flagged.contains(pos))
            .count() as u8
    }

    pub fn neighboring_closed_fields(&self, pos: Position) -> u8 {
        self.neighboring_fields_iter(pos)
            .filter(|pos| !self.is_open(*pos))
            .filter(|pos| self.is_in_bounds(*pos))
            .count() as u8
    }

    // endregion

    // region Player interaction methods

    pub fn open(&mut self, pos: Position) -> Option<OpenResult> {
        // Skip the position if the field is flagged
        if self.is_flagged(pos)
            || self.is_question(pos)
            || self.is_open(pos)
            || self.game_state == GameState::Loss
        {
            return None;
        }
        // Skip the position if the field is out of bounds
        if !self.is_in_bounds(pos) {
            return None;
        }

        // Open the field
        self.opened.insert(pos);

        // Don't open neighboring fields if this one is mined
        if self.is_mined(pos) {
            self.game_state = GameState::Loss;
            return Some(OpenResult::Mine);
        }

        // Open the neighboring fields if safe to do so
        let mine_count = self.neighboring_mines(pos);
        let flag_count = self.neighboring_flags(pos);
        if mine_count == flag_count {
            for neighbor in self.neighboring_fields_iter(pos) {
                if !self.is_flagged(neighbor) && !self.is_open(neighbor) {
                    self.open(neighbor);
                }
            }
        }

        self.check_game_state();
        return Some(OpenResult::NoMine(mine_count));
    }

    pub fn flag(&mut self, pos: Position) {
        // Skip the position if the field is opened
        if self.is_open(pos) || self.game_state == GameState::Loss {
            return;
        }
        // Skip the position if the field is out of bounds
        if !self.is_in_bounds(pos) {
            return;
        }

        if self.is_flagged(pos) {
            self.flagged.remove(&pos);
            self.question.insert(pos);
        } else if self.is_question(pos) {
            self.question.remove(&pos);
        } else {
            self.flagged.insert(pos);
            self.check_game_state();
        }
    }

    // endregion
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Indicate the number of mines remaining
        let remaining_mines = self.mines.len() - self.flagged.len();
        write!(f, "Remaining Mines: {}\n", remaining_mines)?;

        // Print board
        for y in (0..self.width).rev() {
            // Print row heading
            write!(f, "|{}|", y)?;

            // Print each column for the given row
            for x in 0..self.height {
                let pos = (x, y);

                // Display the mine in the current space if the game is lost or the mine is revealed
                if self.is_mined(pos) {
                    if self.is_open(pos) || self.game_state == GameState::Loss {
                        f.write_str("💣 ")?;
                        continue;
                    }
                }

                // If the tile is open, display the number of neighboring mines
                if self.is_open(pos) {
                    let neighboring_mines = self.neighboring_mines(pos);
                    if neighboring_mines > 0 {
                        write!(f, " {} ", neighboring_mines)?;
                    } else {
                        f.write_str("   ")?;
                    }
                    continue;
                }

                // If the field is flagged, display a flag
                if self.is_flagged(pos) {
                    f.write_str("🚩 ")?;
                    continue;
                }

                // Otherwise display an empty field
                f.write_str("⬜ ")?;
            }
            f.write_char('\n')?;
        }

        // Print column headings
        write!(f, "| |")?;
        for x in 0..self.width {
            write!(f, "|{}|", x)?;
        }
        write!(f, "\n")?;

        // Inform the user if they have lost
        if self.game_state == GameState::Loss {
            f.write_str("You lost!\n")?;
        } else if self.game_state == GameState::Win {
            f.write_str("You won!\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::minesweeper::*;

    #[test]
    fn generation_test() {
        // Create minesweeper instance with known number of mines
        let num_mines = 5;
        let ms = Minesweeper::new(10, 10, num_mines);

        // Check the number of mines is correct
        println!("{:?}", ms);
        assert!(ms.mines.len() == num_mines);
    }

    #[test]
    fn play_test() {
        let mut ms = Minesweeper::new(10, 10, 10);

        ms.open((4, 4));
        ms.flag((5, 5));

        println!("{}", ms);
    }

    #[test]
    fn bounds_test() {
        let ms = Minesweeper::new(10, 10, 10);
        assert!(ms.is_in_bounds((5, 5)) == true);
    }
}
