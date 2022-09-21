//! # Prediction module
//!
//! This module contain the structure use to define a prediction

use crate::regiment::Regiment;

/// # This resume what happens to a regiment after a phase of the game.
///
/// ## Members
/// regiment (Regiment): The regiment affected by the phase
///
/// point_gained (usize): The points earned by the regiment during the phase
#[derive(Debug, Clone)]
pub struct UnitResult {
    pub regiment: Regiment,
    pub points_gained: usize,
}

/// This is a prediction of what happened after a phase of the game.
///
/// ## Members
/// first_unit_result (UnitResult): The first regiment
///
/// second_unit_result (UnitResult): The second regiment
///
/// achievement_probability (f64): The probability of the achievement of the prophecy
#[derive(Debug)]
pub struct Prediction {
    first_unit_result: UnitResult,
    second_unit_result: UnitResult,
    achivement_probability: f64,
}

impl Prediction {
    /// Create a new Prediction
    ///
    /// ## Return
    /// Prediction -> The newly created Prediction
    pub fn new(first_unit: UnitResult, second_unit: UnitResult, probability: f64) -> Prediction {
        Prediction {
            first_unit_result: first_unit,
            second_unit_result: second_unit,
            achivement_probability: probability,
        }
    }

    pub fn get_first_unit_result(&self) -> UnitResult {
        self.first_unit_result.clone()
    }

    pub fn get_second_unit_result(&self) -> UnitResult {
        self.second_unit_result.clone()
    }

    pub fn get_probability(&self) -> f64 {
        self.achivement_probability
    }
}

#[cfg(test)]
mod tests {
}