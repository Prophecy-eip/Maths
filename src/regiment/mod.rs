//! # Regiment module
//!
//! This module contain all the structs and datas needed by a Regiment

use crate::model;

/// Struct containing all the informations about a Regiment
///
/// # Attributes
/// model (model::Model): The Model composing the Regiment
///
/// nb_rows (usize): Number of rows in the Regiment
///
/// nb_cols (usize): Number of columns in the Regiment
///
/// nb_models (usize): Number of Model remaining in the Regiment
///
pub struct Regiment {
    model: model::Model,
    nb_rows: usize,
    nb_cols: usize,
    nb_models: usize,
}

impl Regiment {
    /// # Create a new Regiment of the model with the specified number of unit
    ///
    /// ## Return
    /// Regiment: The Regiment created
    pub fn new(model: model::Model, nb_rows: usize, nb_cols: usize, nb_models: usize) -> Regiment {
        Regiment {
            model,
            nb_rows,
            nb_cols,
            nb_models,
        }
    }
    /// # Get the Model of the Regiment
    ///
    /// ## Return
    /// &model::Model: The Model in the Regiment
    pub fn get_model(&self) -> &model::Model {
        &self.model
    }

    /// # Get the number of rows in the Regiment
    ///
    /// ## Return
    /// usize: The number of rows in the Regiment
    pub fn get_rows(&self) -> usize {
        self.nb_rows
    }

    /// # Get the number of columns in the Regiment
    ///
    /// ## Return
    /// usize: The number of columns in the Regiment
    pub fn get_cols(&self) -> usize {
        self.nb_cols
    }

    /// # Get the number of Models in the Regiment
    ///
    /// ## Return
    /// usize: The number of Models in the Regiment
    pub fn get_nb_models(&self) -> usize {
        self.nb_models
    }
}

#[cfg(test)]
mod tests {
    use crate::{global_test, model, regiment};

    fn initialize_chaos_warrior() -> regiment::Regiment {
        let chaos_warrior: regiment::Regiment = global_test::tests::initialize_regiment(
            global_test::tests::initialize_stats(4, 8, 8, 1, 5, 4, 0, 0, 2, 5, 4, 1, 4),
            global_test::tests::initialize_mock_modifier_stats(),
            4,
            5,
            20,
        );
        chaos_warrior
    }

    #[test]
    fn test_get_model() {
        let chaos_warrior_stats: model::Stats =
            global_test::tests::initialize_stats(4, 8, 8, 1, 5, 4, 0, 0, 2, 5, 4, 1, 4);
        let chaos_warrior_modifier_stats: model::Stats =
            global_test::tests::initialize_mock_modifier_stats();

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
}
