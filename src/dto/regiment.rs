//! RegimentDto module
//!
//! This module contains the RegimentDto struct and its implementation.
//! The goal is to be able to communicate easily with outside of the library.

/// Struct used to represent a Regiment outside of the library.
///
/// # Attributes
///
/// model (ModelDto): The model of the regiment.
///
/// nb_rows (usize): The number of rows of the regiment.
///
/// nb_cols (usize): The number of columns of the regiment.
///
/// nb_models (usize): The number of models in the regiment.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RegimentDto {
    model: super::model::ModelDto,
    nb_rows: usize,
    nb_cols: usize,
    nb_models: usize,
}

impl RegimentDto {
    /// Creates a new RegimentDto.
    ///
    /// # Arguments
    ///
    /// model (ModelDto): The model of the regiment.
    ///
    /// nb_rows (usize): The number of rows of the regiment.
    ///
    /// nb_cols (usize): The number of columns of the regiment.
    ///
    /// nb_models (usize): The number of models in the regiment.
    ///
    /// # Return
    ///
    /// RegimentDto: The newly created RegimentDto.
    pub fn new(
        model: super::model::ModelDto,
        nb_rows: usize,
        nb_cols: usize,
        nb_models: usize,
    ) -> RegimentDto {
        RegimentDto {
            model,
            nb_rows,
            nb_cols,
            nb_models,
        }
    }

    /// Hydrates the RegimentDto into a Regiment.
    ///
    /// # Return
    ///
    /// Regiment: The newly created Regiment.
    pub fn hydrate(&self) -> crate::regiment::Regiment {
        crate::regiment::Regiment::new(
            self.model.hydrate(),
            self.nb_rows,
            self.nb_cols,
            self.nb_models,
            None,
        )
    }

    /// Dehydrates a Regiment into a RegimentDto.
    ///
    /// # Arguments
    ///
    /// regiment (Regiment): The Regiment to dehydrate.
    ///
    /// # Return
    ///
    /// RegimentDto: The newly created RegimentDto.
    pub fn dehydrate(regiment: &crate::regiment::Regiment) -> RegimentDto {
        RegimentDto {
            model: super::model::ModelDto::dehydrate(regiment.get_model()),
            nb_rows: regiment.get_rows(),
            nb_cols: regiment.get_cols(),
            nb_models: regiment.get_nb_models(),
        }
    }
}
