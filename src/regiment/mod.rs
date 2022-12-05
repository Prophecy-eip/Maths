//! # Regiment module
//!
//! This module contain all the structs and datas needed by a Regiment

use crate::model;
use serde;

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
/// regiment_health_points (usize): The total number of health points that left to the regiment
///
/// points (usize): The points earned by the regiment
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Regiment {
    model: model::Model,
    nb_rows: usize,
    nb_cols: usize,
    nb_models: usize,
    regiment_health_points: usize,
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
    /// Option<usize> regiment_health_points: The number of health points that left to the regiment.
    /// If None, the default value will be (the number of model * the health_points stats of the model)
    ///
    /// ## Return
    /// Regiment: The Regiment created
    pub fn new(
        model: model::Model,
        nb_rows: usize,
        nb_cols: usize,
        nb_models: usize,
        regiment_health_points: Option<usize>,
    ) -> Regiment {
        let health_points: usize = model.get_boosted_stats().get_health_points();
        Regiment {
            model,
            nb_rows,
            nb_cols,
            nb_models,
            regiment_health_points: regiment_health_points.unwrap_or(nb_models * health_points),
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
        self.regiment_health_points
    }

    /// Inflict damage to the regiment and rearrange the ranks
    ///
    /// ## Parameters
    /// (usize) amount: The amount of damages inflicted to the regiment
    ///
    /// ## Return
    /// usize: The new amount of regiment health points
    pub fn take_damage(&mut self, amount: usize) -> usize {
        self.regiment_health_points = self.regiment_health_points.saturating_sub(amount);
        self.nb_models = (self.regiment_health_points as f64
            / self.model.get_boosted_stats().get_health_points() as f64)
            .ceil() as usize;
        if self.nb_models >= self.nb_cols {
            self.nb_rows = (self.nb_models as f64 / self.nb_cols as f64).ceil() as usize;
        } else {
            self.nb_rows = 1;
            self.nb_cols = self.nb_models;
        }
        self.regiment_health_points
    }

    /// Get the number of points earn by the regiment
    ///
    /// ## Return
    /// usize: The number of points of the regiment
    pub fn get_points(&self) -> usize {
        self.points
    }

    /// Give some points to the regiment
    ///
    /// ## Parameters
    /// (usize) points: The amount of points to give
    ///
    /// ## Return
    /// usize: The new number of points of the regiment
    pub fn earn_points(&mut self, points: usize) -> usize {
        self.points += points;
        self.points
    }
}

#[cfg(test)]
mod tests {
    use crate::{model, regiment, global_test};


    fn initialize_chaos_warrior() -> regiment::Regiment {
        let chaos_warrior: regiment::Regiment = global_test::initialize_regiment(
            global_test::initialize_stats(4, 8, 8, 1, 5, 4, 0, 0, 2, 5, 4, 1, 4),
            global_test::initialize_modifier_stats_with_0(),
            4, 5, 20);
        chaos_warrior
    }

    #[test]
    fn test_get_model() {
        let chaos_warrior_stats: model::Stats = global_test::initialize_stats(4, 8, 8, 1, 5, 4, 0, 0, 2, 5, 4, 1, 4);
        let chaos_warrior_modifier_stats: model::Stats = global_test::initialize_modifier_stats_with_0();

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
