//! # Prediction module
//!
//! This module contain the structure use to define a prediction

use crate::regiment::Regiment;

/// This is a prediction of what happened after a phase of the game.
///
/// ## Attributes
/// attacking_regiment (Regiment): The first regiment
///
/// defending_regiment (Regiment): The second regiment
///
/// occurrence_probability (f64): The probability that the prophecy occurs
#[derive(Debug)]
pub struct Prediction {
    attacking_regiment: Regiment,
    defending_regiment: Regiment,
    occurrence_probability: f64,
}

impl Prediction {
    /// Create a new Prediction
    ///
    /// ## Parameters
    /// (regiment::Regiment) attacking_regiment: The attacking regiment
    ///
    /// (regiment::Regiment) defending_regiment: The defending regiment
    ///
    /// (f64) probability: The occurence probability of the Prediction
    ///
    /// ## Return
    /// Prediction: The newly created Prediction
    pub fn new(
        attacking_regiment: Regiment,
        defending_regiment: Regiment,
        probability: f64,
    ) -> Prediction {
        Prediction {
            attacking_regiment,
            defending_regiment,
            occurrence_probability: probability,
        }
    }

    /// Get the attacking regiment
    ///
    /// ## Return
    /// Regiment : The attacking regiment
    pub fn get_attacking_regiment(&self) -> Regiment {
        self.attacking_regiment.clone()
    }

    /// Get the defending regiment
    ///
    /// ## Return
    /// Regiment : The defending regiment
    pub fn get_defending_regiment(&self) -> Regiment {
        self.defending_regiment.clone()
    }

    /// Get the probability that this prediction occurs
    ///
    /// ## Return
    /// f64 : The probability
    pub fn get_probability(&self) -> f64 {
        self.occurrence_probability
    }
}

#[cfg(test)]
mod tests {
    use super::{Prediction, Regiment};
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

    // Prediction

    #[test]
    pub fn test_prediction_get_probability() {
        let attacking_regiment: Regiment = initialize_chaos_warrior();
        let defending_regiment: Regiment = initialize_chaos_warrior();
        let prediction: Prediction = Prediction::new(attacking_regiment, defending_regiment, 0.2);
        assert_eq!(0.2 - prediction.get_probability() < 0.001, true);
    }

    #[test]
    pub fn test_prediction_get_defending_regiment() {
        let attacking_regiment: Regiment = initialize_chaos_warrior();
        let defending_regiment: Regiment = initialize_chaos_warrior();
        let copy: Regiment = defending_regiment.clone();
        let prediction: Prediction = Prediction::new(attacking_regiment, defending_regiment, 0.2);
        assert_eq!(prediction.get_attacking_regiment(), copy);
    }

    #[test]
    pub fn test_prediction_get_attacking_regiment() {
        let attacking_regiment: Regiment = initialize_chaos_warrior();
        let defending_regiment: Regiment = initialize_chaos_warrior();
        let copy = attacking_regiment.clone();
        let prediction: Prediction = Prediction::new(attacking_regiment, defending_regiment, 0.2);
        assert_eq!(prediction.get_defending_regiment(), copy);
    }
}
