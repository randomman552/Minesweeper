use std::collections::HashMap;

use crate::minesweeper::*;

pub enum SolverStep {
    Open(u16, u16),
    Flag(u16, u16),
    None,
}

#[derive(Debug)]
pub struct Solver {
    field: HashMap<Position, f32>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            field: HashMap::new(),
        }
    }

    pub fn solve_step(&mut self, game: &Minesweeper) -> SolverStep {
        self.calculate_field(game);

        // TODO: Flag any fields with 100% chance that are not already flagged
        // TODO: Ignore any fields with 100% chance that are flagged
        // TODO: Open any fields with 0% chance

        return SolverStep::None;
    }

    /// Get the probability (0-1) that the field with the given position is a mine
    ///
    /// Or 0 if not set
    pub fn get_mine_chance(&self, pos: Position) -> f32 {
        let chance = self.field.get(&pos);

        if chance.is_some() {
            return *chance.unwrap();
        }

        return 0.0;
    }

    // Calculate mine chance for every
    fn calculate_field(&mut self, game: &Minesweeper) {
        // Clear the field
        self.field.clear();

        // Calculate chance for every position in the game
        for y in 0..game.height {
            for x in 0..game.width {
                let pos: Position = (x, y);
                let mine_chance = self.calculate_mine_chance(game, pos);
                self.field.insert(pos, mine_chance);
            }
        }
    }

    // Get the chance
    fn calculate_mine_chance(&self, game: &Minesweeper, pos: Position) -> f32 {
        // If not in bounds, chance is 0
        if !game.is_in_bounds(pos) {
            return 0.0;
        }

        // If already open, chance is 0
        if game.is_open(pos) {
            return 0.0;
        }

        // If flagged, chance is 100%
        if game.is_flagged(pos) {
            return 1.0;
        }

        // TODO: More complicated calculations

        return 0.0;
    }
}
