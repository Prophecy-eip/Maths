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

#[cfg(test)]
mod tests {
    #[test]
    fn test_regiment_dto_hydrate() {
        let model_dto = crate::dto::model::ModelDto::new(
            crate::dto::stat::StatsDto::new(
                crate::dto::stat::GlobalStatsDto::new(1, 2, 3),
                crate::dto::stat::DefensiveStatsDto::new(4, 5, 6, 7, 8),
                crate::dto::stat::OffensiveStatsDto::new(9, 10, 11, 12, 13),
            ),
            vec![],
            true,
        );
        let regiment = super::RegimentDto::new(model_dto, 1, 2, 3).hydrate();
        assert_eq!(regiment.get_rows(), 1);
        assert_eq!(regiment.get_cols(), 2);
        assert_eq!(regiment.get_nb_models(), 3);
        assert_eq!(regiment.get_model().get_stats().get_advance(), 1);
        assert_eq!(regiment.get_model().get_stats().get_attack(), 9);
    }

    #[test]
    fn test_regiment_dto_dehydrate() {
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
        let regiment = crate::regiment::Regiment::new(model, 1, 2, 3, None);
        let regiment_dto = super::RegimentDto::dehydrate(&regiment);
        assert_eq!(regiment_dto.nb_rows, 1);
        assert_eq!(regiment_dto.nb_cols, 2);
        assert_eq!(regiment_dto.nb_models, 3);
        assert_eq!(regiment_dto.model.get_stats().get_advance(), 1);
        assert_eq!(regiment_dto.model.get_stats().get_attack(), 9);
    }
}
