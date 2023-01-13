//! # Web_server Regiment module
//!
//! This module contains all the structs and datas needed by a Regiment in the web server

use crate::web_server::data_structures::model;

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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Regiment {
    pub model: model::Model,
    pub nb_rows: usize,
    pub nb_cols: usize,
    pub nb_models: usize,
}
