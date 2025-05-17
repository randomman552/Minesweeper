use std::{collections::HashMap, fmt::Display};

use crate::minesweeper::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SolverStep {
    Open(Position),
    Flag(Position),
    None,
}

impl Display for SolverStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolverStep::None => write!(f, "None")?,
            SolverStep::Flag(pos) => write!(f, "Flag ({}, {})", pos.0, pos.1)?,
            SolverStep::Open(pos) => write!(f, "Open ({}, {})", pos.0, pos.1)?,
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MineChance {
    WithInformation(f32),
    NoInformation(f32),
}

impl Display for MineChance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MineChance::NoInformation(chance) => write!(f, "{:.0}%?", chance * 100.0)?,
            MineChance::WithInformation(chance) => write!(f, "{:.0}%", chance * 100.0)?,
        };

        Ok(())
    }
}

#[derive(Debug)]
pub struct Solver {
    field: HashMap<Position, MineChance>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            field: HashMap::new(),
        }
    }

    pub fn solve_step(&mut self, game: &Minesweeper) -> SolverStep {
        if game.game_state != GameState::InProgress {
            return SolverStep::None;
        }

        self.calculate_field(game);

        // Default action is to do nothing
        let mut action = SolverStep::None;
        let mut best_guess: Option<Position> = None;

        // Logic for fields with concerete information
        for (pos, chance) in self.field.clone() {
            if let MineChance::WithInformation(probability) = chance {
                // Flag any fields with 100% chance that are not already flagged
                if probability >= 1.0 && !game.is_flagged(pos) {
                    log::info!(
                        "Solver suggests flagging field ({}, {}), guaranteed mine",
                        pos.0,
                        pos.1
                    );
                    action = SolverStep::Flag(pos);
                    break;
                }

                // Open any fields with 0% chance
                if probability <= 0.0 && !game.is_open(pos) {
                    log::info!(
                        "Solver suggests opening field ({}, {}), guaranteed safe",
                        pos.0,
                        pos.1
                    );
                    action = SolverStep::Open(pos);
                    break;
                }

                // Is this better than the current best guess?
                if !game.is_open(pos) && !game.is_flagged(pos) {
                    if best_guess.is_some() {
                        let guess_pos = best_guess.unwrap();
                        let guess_chance = self.get_mine_chance(guess_pos);

                        // Check probability of this guess would be lower than the current best guess
                        if let MineChance::WithInformation(guess_probability) = guess_chance {
                            if probability < guess_probability {
                                best_guess = Some(pos);
                            }
                        }
                        // Always displace guesses with no information
                        if let MineChance::NoInformation(_) = guess_chance {
                            best_guess = Some(pos);
                        }
                    } else {
                        best_guess = Some(pos);
                    }
                }
            }
        }

        // Logic for fields without concrete information
        // These take lower priority compared to those with information
        for (pos, chance) in self.field.clone() {
            if let MineChance::NoInformation(probability) = chance {
                // Flag any fields with 100% chance that are not already flagged
                if probability >= 1.0 && !game.is_flagged(pos) {
                    log::info!(
                        "Solver suggests flagging field ({}, {}), guaranteed mine",
                        pos.0,
                        pos.1
                    );
                    action = SolverStep::Flag(pos);
                    break;
                }

                // Open any fields with 0% chance
                if probability <= 0.0 && !game.is_open(pos) {
                    log::info!(
                        "Solver suggests opening field ({}, {}), guaranteed safe",
                        pos.0,
                        pos.1
                    );
                    action = SolverStep::Open(pos);
                    break;
                }

                // Is this better than the current best guess?
                if !game.is_open(pos) && !game.is_flagged(pos) {
                    if best_guess.is_some() {
                        let guess_pos = best_guess.unwrap();
                        let guess_chance = self.get_mine_chance(guess_pos);

                        // Check probability of this guess would be lower than the current best guess
                        if let MineChance::NoInformation(guess_probability) = guess_chance {
                            if probability < guess_probability {
                                best_guess = Some(pos);
                            }
                        }
                    } else {
                        best_guess = Some(pos);
                    }
                }
            }
        }

        // We must guess!
        if best_guess.is_some() && action == SolverStep::None {
            let pos = best_guess.unwrap();
            let chance = self.get_mine_chance(pos);
            log::info!(
                "Solver suggests opening field ({}, {}), best safe guess with chance {}",
                pos.0,
                pos.1,
                chance
            );
            action = SolverStep::Open(pos);
        }

        if action == SolverStep::None {
            log::info!("Solver suggests no action");
        }
        return action;
    }

    /// Get the probability (0-1) that the field with the given position is a mine
    ///
    /// Or 0 if not set
    pub fn get_mine_chance(&self, pos: Position) -> MineChance {
        let chance = self.field.get(&pos);

        if chance.is_some() {
            return *chance.unwrap();
        }

        return MineChance::NoInformation(0.0);
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
    fn calculate_mine_chance(&self, game: &Minesweeper, pos: Position) -> MineChance {
        // If not in bounds, chance is 0
        if !game.is_in_bounds(pos) {
            return MineChance::WithInformation(0.0);
        }

        // If already open, chance is 0
        if game.is_open(pos) {
            // Don't return 0 if the field is mined
            // So we can review why the sovler lost the game
            if !game.is_mined(pos) {
                return MineChance::WithInformation(0.0);
            }
        }

        // If flagged, chance is 100%
        if game.is_flagged(pos) {
            return MineChance::WithInformation(1.0);
        }

        // Build a list of neighbors (that are within the game bounds)
        let neighbors = game
            .neighboring_fields_iter(pos)
            .filter(|neighbor| game.is_in_bounds(*neighbor));

        // Iterate over all neighbors
        let mut guess_chance: Vec<f32> = Vec::new();
        for neighbor in neighbors {
            // We can only evaluate a neighbor if it is open
            // The is_mined check here is only useful in the case that a mine has already been revealed, so it's not cheating!
            if game.is_open(neighbor) && !game.is_mined(neighbor) {
                // Look for neighbors that only have the exact number of times surrounding them as mines
                // Means this tile is guaranteed to be a mineif game.is_open(neighbor) {
                let mine_count = game.neighboring_mines(neighbor);
                let closed_field_count = game.neighboring_closed_fields(neighbor);
                let flag_count = game.neighboring_flags(neighbor);

                // If flag count is equal to the mine count, this tile cannot be mined
                if mine_count == flag_count {
                    return MineChance::WithInformation(0.0);
                }

                // If the mine count is equal to the closed field count, this field must be mined
                if mine_count == closed_field_count {
                    return MineChance::WithInformation(1.0);
                }

                // No guarantees, we must guess according to chance (if we can find no other option)
                guess_chance.push(mine_count as f32 / closed_field_count as f32);
            }
        }

        // If we have guess chance's, return the average of the guesses
        if guess_chance.len() > 0 {
            let sum: f32 = guess_chance.iter().sum();
            let count = guess_chance.len();
            if count == 0 {
                return MineChance::WithInformation(0.0);
            } else {
                let avg = sum / count as f32;
                return MineChance::WithInformation(avg);
            }
        }

        // Otherwise, chance is the base chance of finding a mine in any field
        return MineChance::NoInformation(
            game.remaining_mines() as f32 / (game.width * game.height) as f32,
        );
    }
}
