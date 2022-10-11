//! # Regiment module
//!
//! This module contain all the structs and datas needed by a Regiment

use crate::model;

/// Struct containing all the informations about a Regiment
///
/// ## Attributes
/// model (model::Model): The Model composing the Regiment
///
/// nb_rows (usize): Number of rows in the Regiment
///
/// nb_cols (usize): Number of columns in the Regiment
///
/// nb_models (usize): Number of Model remaining in the Regiment
///
/// regiment_health_point (usize): The total number of health points that left to the regiment
///
/// points (usize): The points earned by the regiment
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Regiment {
    model: model::Model,
    nb_rows: usize,
    nb_cols: usize,
    nb_models: usize,
    regiment_health_point: usize,
    points: usize,
}

impl Regiment {
    /// Create a new Regiment of the model with the specified number of unit
    ///
    /// ## Parameters
    /// (model::Model) model: The model that compose the regiment
    ///
    /// (usize) nb_rows: The number of rows that compose the regiment
    ///
    /// (usize) nb_cols: The number of columns that compose the regiment
    ///
    /// (usize) nb_models: The number of models in the regiment
    ///
    /// Option<usize> regiment_health_point: The number of health points that left to the regiment.
    /// If None, the default value will be (the number of model * the health_point stats of the model)
    ///
    /// ## Return
    /// Regiment: The Regiment created
    pub fn new(
        model: model::Model,
        nb_rows: usize,
        nb_cols: usize,
        nb_models: usize,
        regiment_health_point: Option<usize>,
    ) -> Regiment {
        let health_points: usize = model.get_stats().get_health_point();
        Regiment {
            model,
            nb_rows,
            nb_cols,
            nb_models,
            regiment_health_point: regiment_health_point.unwrap_or(nb_models * health_points),
            points: 0,
        }
    }
    /// Get the Model of the Regiment
    ///
    /// ## Return
    /// &model::Model: The Model in the Regiment
    pub fn get_model(&self) -> &model::Model {
        &self.model
    }

    /// Get the number of rows in the Regiment
    ///
    /// ## Return
    /// usize: The number of rows in the Regiment
    pub fn get_rows(&self) -> usize {
        self.nb_rows
    }

    /// Get the number of columns in the Regiment
    ///
    /// ## Return
    /// usize: The number of columns in the Regiment
    pub fn get_cols(&self) -> usize {
        self.nb_cols
    }

    /// Get the number of Models in the Regiment
    ///
    /// ## Return
    /// usize: The number of Models in the Regiment
    pub fn get_nb_models(&self) -> usize {
        self.nb_models
    }

    /// Get the regiment total health points
    ///
    /// ## Return
    /// usize: The regiment total health points
    pub fn get_regiment_health_points(&self) -> usize {
        self.regiment_health_point
    }

    /// Inflict damage to the regiment and rearrange the ranks
    ///
    /// ## Return
    /// usize: The new amount of regiment health points
    pub fn take_damage(&mut self, amount: usize) -> usize {
        self.regiment_health_point = self.regiment_health_point.saturating_sub(amount);
        self.nb_models = (self.regiment_health_point as f64
            / self.model.get_stats().get_health_point() as f64)
            .ceil() as usize;
        if self.nb_models >= self.nb_cols {
            self.nb_rows = (self.nb_models as f64 / self.nb_cols as f64).ceil() as usize;
        } else {
            self.nb_rows = 1;
            self.nb_cols = self.nb_models;
        }
        self.regiment_health_point
    }

    pub fn get_points(&self) -> usize {
        self.points
    }

    pub fn earn_points(&mut self, points: usize) -> usize {
        self.points += points;
        self.points
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_get_model() {
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
        let chaos_warrior: regiment::Regiment = initialize_chaos_warrior();
        assert_eq!(chaos_warrior.get_model().to_owned(), model_chaos_warrior);
    }

    #[test]
    fn test_get_rows() {
        let chaos_warrior: regiment::Regiment = initialize_chaos_warrior();
        assert_eq!(chaos_warrior.get_rows(), 4);
    }

    #[test]
    fn test_get_cols() {
        let chaos_warrior: regiment::Regiment = initialize_chaos_warrior();
        assert_eq!(chaos_warrior.get_cols(), 5);
    }

    #[test]
    fn test_get_nb_models() {
        let chaos_warrior: regiment::Regiment = initialize_chaos_warrior();
        assert_eq!(chaos_warrior.get_nb_models(), 20);
    }

    #[test]
    fn test_get_regiment_health() {
        let chaos_warrior: regiment::Regiment = initialize_chaos_warrior();
        assert_eq!(chaos_warrior.get_regiment_health_points(), 20);
    }

    #[test]
    fn test_take_damage() {
        let mut chaos_warrior: regiment::Regiment = initialize_chaos_warrior();
        chaos_warrior.take_damage(5);
        assert_eq!(chaos_warrior.get_regiment_health_points(), 15);
        assert_eq!(chaos_warrior.get_nb_models(), 15);
    }

    #[test]
    fn test_take_damage_loose_a_line() {
        let mut chaos_warrior: regiment::Regiment = initialize_chaos_warrior();
        chaos_warrior.take_damage(6);
        assert_eq!(chaos_warrior.get_regiment_health_points(), 14);
        assert_eq!(chaos_warrior.get_nb_models(), 14);
        assert_eq!(chaos_warrior.get_cols(), 5);
        assert_eq!(chaos_warrior.get_rows(), 3);
    }

    #[test]
    fn test_take_damage_less_than_a_line() {
        let mut chaos_warrior: regiment::Regiment = initialize_chaos_warrior();
        chaos_warrior.take_damage(17);
        assert_eq!(chaos_warrior.get_regiment_health_points(), 3);
        assert_eq!(chaos_warrior.get_nb_models(), 3);
        assert_eq!(chaos_warrior.get_cols(), 3);
        assert_eq!(chaos_warrior.get_rows(), 1);
    }

    #[test]
    fn test_get_points() {
        let chaos_warrior: regiment::Regiment = initialize_chaos_warrior();
        assert_eq!(0, chaos_warrior.get_points());
    }

    #[test]
    fn test_earn_points() {
        let mut chaos_warrior: regiment::Regiment = initialize_chaos_warrior();

        assert_eq!(0, chaos_warrior.get_points());
        chaos_warrior.earn_points(5);
        assert_eq!(5, chaos_warrior.get_points());
        chaos_warrior.earn_points(6);
        assert_eq!(11, chaos_warrior.get_points());
    }
}
