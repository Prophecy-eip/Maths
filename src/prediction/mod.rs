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
pub struct RegimentResult {
    regiment: Regiment,
    points_gained: usize,
}


impl RegimentResult {
    /// ## Create a new RegimentResult
    /// 
    /// ### Return
    /// RegimentResult : The Result created
    pub fn new(regiment: Regiment, points_gained: usize) -> RegimentResult {
        RegimentResult {
            regiment,
            points_gained
        }
    }

    pub fn get_regiment(&self) -> &Regiment {
        &self.regiment
    }

    pub fn get_points_gained(&self) -> usize {
        self.points_gained
    }
}

/// This is a prediction of what happened after a phase of the game.
///
/// ## Members
/// first_unit_result (RegimentResult): The first regiment
///
/// second_unit_result (RegimentResult): The second regiment
///
/// achievement_probability (f64): The probability of the achievement of the prophecy
#[derive(Debug)]
pub struct Prediction {
    first_unit_result: RegimentResult,
    second_unit_result: RegimentResult,
    achivement_probability: f64,
}

impl Prediction {
    /// Create a new Prediction
    ///
    /// ## Return
    /// Prediction -> The newly created Prediction
    pub fn new(first_unit: RegimentResult, second_unit: RegimentResult, probability: f64) -> Prediction {
        Prediction {
            first_unit_result: first_unit,
            second_unit_result: second_unit,
            achivement_probability: probability,
        }
    }

    pub fn get_first_unit_result(&self) -> RegimentResult {
        self.first_unit_result.clone()
    }

    pub fn get_second_unit_result(&self) -> RegimentResult {
        self.second_unit_result.clone()
    }

    pub fn get_probability(&self) -> f64 {
        self.achivement_probability
    }
}

#[cfg(test)]
mod tests {
}