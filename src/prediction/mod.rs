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
#[derive(Debug, Clone, PartialEq, Eq)]
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
            points_gained,
        }
    }

    /// ## Get the Regiment that is represented
    ///
    /// ### Return
    /// Regiment : The Regiment concerned
    pub fn get_regiment(&self) -> &Regiment {
        &self.regiment
    }

    /// ## Get the number of points earn after the phase
    ///
    /// ### Return
    /// usize : The amount of points
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
    pub fn new(
        first_unit: RegimentResult,
        second_unit: RegimentResult,
        probability: f64,
    ) -> Prediction {
        Prediction {
            first_unit_result: first_unit,
            second_unit_result: second_unit,
            achivement_probability: probability,
        }
    }

    /// ## Get the state of the first unit after the computation
    ///
    /// ### Return
    /// RegimentResult : The Result created
    pub fn get_first_unit_result(&self) -> RegimentResult {
        self.first_unit_result.clone()
    }

    /// ## Get the state of the second unit after the computation
    ///
    /// ### Return
    /// RegimentResult : The Result created
    pub fn get_second_unit_result(&self) -> RegimentResult {
        self.second_unit_result.clone()
    }

    /// ## Get the probability that this prediction occurs
    ///
    /// ### Return
    /// f64 : The probability
    pub fn get_probability(&self) -> f64 {
        self.achivement_probability
    }
}

#[cfg(test)]
mod tests {
    use super::{Prediction, RegimentResult};
    use crate::{model, regiment};

    fn initialize_chaos_warrior() -> regiment::Regiment {
        let chaos_warrior_stats: model::Stats = model::Stats::new(
            model::GlobalStats {
                advance: 4,
                march: 8,
                discipline: 8,
            },
            model::DefensiveStats {
                health_point: 1,
                defense: 5,
                resilience: 4,
                armour: 0,
                aegis: 0,
            },
            model::OffensiveStats {
                attack: 2,
                offensive: 5,
                strength: 4,
                armour_penetration: 1,
                agility: 4,
            },
        );
        let chaos_warrior_modifier_stats: model::Stats = model::Stats::new(
            model::GlobalStats {
                advance: 0,
                march: 0,
                discipline: 0,
            },
            model::DefensiveStats {
                health_point: 0,
                defense: 0,
                resilience: 0,
                armour: 0,
                aegis: 0,
            },
            model::OffensiveStats {
                attack: 0,
                offensive: 0,
                strength: 0,
                armour_penetration: 0,
                agility: 0,
            },
        );
        let chaos_warrior_modifier: model::Modifier =
            model::Modifier::new(chaos_warrior_modifier_stats, false, 0, vec![]);
        let model_chaos_warrior: model::Model =
            model::Model::new(chaos_warrior_stats, vec![chaos_warrior_modifier]);
        let chaos_warrior: regiment::Regiment =
            regiment::Regiment::new(model_chaos_warrior, 4, 5, 20, None);
        chaos_warrior
    }

    // RegimentResult

    #[test]
    pub fn test_get_regiment_regiment_result() {
        let regiment = initialize_chaos_warrior();
        let result = RegimentResult::new(regiment, 5);
        assert_eq!(initialize_chaos_warrior(), result.get_regiment().to_owned());
    }

    #[test]
    pub fn test_get_points_regiment_result() {
        let regiment = initialize_chaos_warrior();
        let result = RegimentResult::new(regiment, 5);
        assert_eq!(5, result.get_points_gained());
    }

    // Prediction

    #[test]
    pub fn test_get_first_unit_result_prediction() {
        let first_regiment = initialize_chaos_warrior();
        let first_result = RegimentResult::new(first_regiment, 5);
        let second_regiment = initialize_chaos_warrior();
        let second_result = RegimentResult::new(second_regiment, 4);
        let prediction = Prediction::new(first_result, second_result, 0.2);
        assert_eq!(0.2 - prediction.get_probability() < 0.001, true);
    }

    #[test]
    pub fn test_get_second_unit_result_prediction() {
        let first_regiment = initialize_chaos_warrior();
        let first_result = RegimentResult::new(first_regiment, 5);
        let second_regiment = initialize_chaos_warrior();
        let second_result = RegimentResult::new(second_regiment, 4);
        let prediction = Prediction::new(first_result, second_result, 0.2);
        assert_eq!(
            prediction.get_first_unit_result(),
            RegimentResult::new(initialize_chaos_warrior(), 5)
        );
    }

    #[test]
    pub fn test_get_probability_prediction() {
        let first_regiment = initialize_chaos_warrior();
        let first_result = RegimentResult::new(first_regiment, 5);
        let second_regiment = initialize_chaos_warrior();
        let second_result = RegimentResult::new(second_regiment, 4);
        let prediction = Prediction::new(first_result, second_result, 0.2);
        assert_eq!(
            prediction.get_second_unit_result(),
            RegimentResult::new(initialize_chaos_warrior(), 4)
        );
    }
}
