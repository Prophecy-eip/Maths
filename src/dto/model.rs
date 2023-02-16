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
    #[serde(skip_serializing_if = "Vec::is_empty")]
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

    /// Get the stats of the model
    ///
    /// # Return
    ///
    /// StatsDto: The stats of the model
    pub fn get_stats(&self) -> &super::stat::StatsDto {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_model_dto_hydrate() {
        let model_dto = super::ModelDto::new(
            crate::dto::stat::StatsDto::new(
                crate::dto::stat::GlobalStatsDto::new(1, 2, 3),
                crate::dto::stat::DefensiveStatsDto::new(4, 5, 6, 7, 8),
                crate::dto::stat::OffensiveStatsDto::new(9, 10, 11, 12, 13),
            ),
            vec![],
            true,
        );
        let model: crate::model::Model = model_dto.hydrate();
        assert_eq!(model.get_stats().get_advance(), 1);
        assert_eq!(model.get_stats().get_march(), 2);
        assert_eq!(model.get_stats().get_discipline(), 3);
        assert_eq!(model.get_stats().get_health_points(), 4);
        assert_eq!(model.get_stats().get_defense(), 5);
        assert_eq!(model.get_stats().get_resilience(), 6);
        assert_eq!(model.get_stats().get_armour(), 7);
        assert_eq!(model.get_stats().get_aegis(), 8);
        assert_eq!(model.get_stats().get_attack(), 9);
        assert_eq!(model.get_stats().get_offensive(), 10);
        assert_eq!(model.get_stats().get_strength(), 11);
        assert_eq!(model.get_stats().get_armour_penetration(), 12);
        assert_eq!(model.get_stats().get_agility(), 13);
    }

    #[test]
    fn test_model_dto_dehydrate() {
        let model = crate::model::Model::new(
            crate::stat::Stats::new(
                crate::stat::GlobalStats {
                    advance: 1,
                    march: 2,
                    discipline: 3,
                },
                crate::stat::DefensiveStats {
                    health_points: 4,
                    defense: 5,
                    resilience: 6,
                    armour: 7,
                    aegis: 8,
                },
                crate::stat::OffensiveStats {
                    attack: 9,
                    offensive: 10,
                    strength: 11,
                    armour_penetration: 12,
                    agility: 13,
                },
            ),
            vec![],
            true,
        );
        let model_dto: super::ModelDto = super::ModelDto::dehydrate(&model);
        assert_eq!(model_dto.get_stats().get_advance(), 1);
        assert_eq!(model_dto.get_stats().get_march(), 2);
        assert_eq!(model_dto.get_stats().get_discipline(), 3);
        assert_eq!(model_dto.get_stats().get_health_points(), 4);
        assert_eq!(model_dto.get_stats().get_defense(), 5);
        assert_eq!(model_dto.get_stats().get_resilience(), 6);
        assert_eq!(model_dto.get_stats().get_armour(), 7);
        assert_eq!(model_dto.get_stats().get_aegis(), 8);
        assert_eq!(model_dto.get_stats().get_attack(), 9);
        assert_eq!(model_dto.get_stats().get_offensive(), 10);
        assert_eq!(model_dto.get_stats().get_strength(), 11);
        assert_eq!(model_dto.get_stats().get_armour_penetration(), 12);
        assert_eq!(model_dto.get_stats().get_agility(), 13);
    }
}
