use std::{
    collections::HashSet,
    fmt::{Display, Write}
};
use rand::prelude::*;

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8)
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    opened: HashSet<Position>,
    flagged: HashSet<Position>,
    mines: HashSet<Position>,
    lost: bool
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, num_mines: usize) -> Minesweeper {
        Minesweeper{
            width,
            height,
            opened: HashSet::new(),
            flagged: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();
                let mut rng = rand::thread_rng();
                
                while mines.len() < num_mines {
                    let pos = (rng.gen_range(0..width), rng.gen_range(0..height));
                    mines.insert(pos);
                }

                mines
            },
            lost: false
        }
    }

    // region Boolean position checks

    pub fn is_mined(&self, pos: Position) -> bool {
        self.mines.contains(&pos)
    }

    pub fn is_open(&self, pos: Position) -> bool {
        self.opened.contains(&pos)
    }

    pub fn is_flagged(&self, pos: Position) -> bool {
        self.flagged.contains(&pos)
    }

    // endregion


    // region Neighboring fields methods

    pub fn neighboring_fields_iter(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let height = self.height;
        let width = self.width;
        
        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| {
                (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j))
            })
            .filter(move |&pos| pos != (x, y))
    }

    pub fn neighboring_mines(&self, pos: Position) -> u8 {
        self
            .neighboring_fields_iter(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn neighboring_flags(&self, pos: Position) -> u8 {
        self
            .neighboring_fields_iter(pos)
            .filter(|pos| self.flagged.contains(pos))
            .count() as u8
    }

    // endregion


    // region Player interaction methods

    pub fn open(&mut self, pos: Position) -> Option<OpenResult> {
        // Skip the position if the field is flagged
        if self.is_flagged(pos) || self.is_open(pos) || self.lost{ return None }
        
        // Open the field
        self.opened.insert(pos);

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

        if self.is_mined(pos) {
            self.lost = true;
            return Some(OpenResult::Mine);
        } else {
            return Some(OpenResult::NoMine(0));
        }
    }

    pub fn flag(&mut self, pos: Position) {
        // Skip this position if the field is opened
        if self.is_open(pos) || self.lost { return; }

        if self.is_flagged(pos) {
            self.flagged.remove(&pos);
        } else {
            self.flagged.insert(pos);
        }
    }

    // endregion
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Indicate the number of mines remaining
        let remaining_mines = self.mines.len() - self.flagged.len();
        write!(f, "Remaining Mines: {}\n", remaining_mines)?;

        for y in 0..self.width {
            for x in 0..self.height {
                let pos = (x, y);

                if self.is_open(pos) {
                    if self.is_mined(pos) {
                        f.write_str("💣 ")?;
                    } else {
                        let neighboring_mines = self.neighboring_mines(pos);
                        if (neighboring_mines > 0) {
                            write!(f, " {} ", neighboring_mines)?;
                        } else {
                            f.write_str("   ")?;
                        }
                    }
                } else if self.is_flagged(pos) {
                    f.write_str("🚩 ")?;
                } else {
                    f.write_str("⬜ ")?;
                }
            }
            f.write_char('\n')?;
        }
        
        // Inform the user if they have lost
        if self.lost {
            f.write_str("You lost!")?;
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
}