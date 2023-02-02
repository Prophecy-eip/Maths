//! ModelDto module
//!
//! This module contains the ModelDto struct and its implementation.
//! The goal is to be able to communicate easily with outside of the library.

/// Struct used to represent a model outside of rust code
///
/// # Attributes
///
/// stats (stat::StatsDto): The stats of the model
///
/// modifiers (Vec<modifier::ModifierDto>): The modifiers of the model
///
/// banner_bearer (bool): Whether the model is a banner bearer or not
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ModelDto {
    stats: super::stat::StatsDto,
    modifiers: Vec<super::modifier::ModifierDto>,
    banner_bearer: bool,
}

impl ModelDto {
    /// Create a new ModelDto
    ///
    /// # Parameters
    ///
    /// stats (stat::StatsDto): The stats of the model
    ///
    /// modifiers (Vec<modifier::ModifierDto>): The modifiers of the model
    ///
    /// banner_bearer (bool): Whether the model is a banner bearer or not
    ///
    /// # Return
    ///
    /// ModelDto: The newly created ModelDto
    pub fn new(
        stats: super::stat::StatsDto,
        modifiers: Vec<super::modifier::ModifierDto>,
        banner_bearer: bool,
    ) -> ModelDto {
        ModelDto {
            stats,
            modifiers,
            banner_bearer,
        }
    }

    /// Hydrate a ModelDto into a Model
    ///
    /// # Return
    ///
    /// Model: The hydrated Model
    pub fn hydrate(&self) -> crate::model::Model {
        crate::model::Model::new(
            self.stats.hydrate(),
            self.modifiers
                .iter()
                .map(|modifier| modifier.hydrate())
                .collect(),
            self.banner_bearer,
        )
    }

    /// Dehydrate a Model into a ModelDto
    ///
    /// # Parameters
    ///
    /// model (Model): The model to dehydrate
    ///
    /// # Return
    ///
    /// ModelDto: The dehydrated ModelDto
    pub fn dehydrate(model: &crate::model::Model) -> ModelDto {
        ModelDto {
            stats: super::stat::StatsDto::dehydrate(model.get_stats()),
            modifiers: model
                .get_modifiers()
                .iter()
                .map(super::modifier::ModifierDto::dehydrate)
                .collect(),
            banner_bearer: model.is_banner_bearer(),
        }
    }
}
