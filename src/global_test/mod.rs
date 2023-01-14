//! # Test module
//!
//! This module contain all the initialization and global functions needed by the tests
#[cfg(test)]
pub mod tests {
    use crate::{model, modifier, regiment};

    /// # Initialize the stats structs with some values given as parameters
    ///
    /// # Parameters
    /// advance (usize): the advance statistic of the model
    ///
    /// march (usize): the march statistic of the model
    ///
    /// discipline (usize): the discipline statistic of the model
    ///
    /// health_point (usize): the health_point statistic of the model
    ///
    /// defense (usize): the defense statistic of the model
    ///
    /// resilience (usize): the resilience statistic of the model
    ///
    /// armour (usize): the armour statistic of the model
    ///
    /// aegis (usize): the aegis statistic of the model
    ///
    /// attack (usize): the attack statistic of the model
    ///
    /// offensive (usize): the offensive statistic of the model
    ///
    /// strength (usize): the strength statistic of the model
    ///
    /// armour_penetration (usize): the armour_penetration statistic of the model
    ///
    /// agility (usize): the agility statistic of the model
    ///
    /// # Return
    /// Stats Model: The Stats created and initialized

    pub fn initialize_stats(
        advance: usize,
        march: usize,
        discipline: usize,
        health_points: usize,
        defense: usize,
        resilience: usize,
        armour: usize,
        aegis: usize,
        attack: usize,
        offensive: usize,
        strength: usize,
        armour_penetration: usize,
        agility: usize,
    ) -> model::Stats {
        let stats: model::Stats = model::Stats::new(
            model::GlobalStats {
                advance: advance,
                march: march,
                discipline: discipline,
            },
            model::DefensiveStats {
                health_points: health_points,
                defense: defense,
                resilience: resilience,
                armour: armour,
                aegis: aegis,
            },
            model::OffensiveStats {
                attack: attack,
                offensive: offensive,
                strength: strength,
                armour_penetration: armour_penetration,
                agility: agility,
            },
        );
        stats
    }

    /// # Initialize all stats structs values to 1
    ///
    /// # Return
    /// Stats Model: Our mock Stats struct

    pub fn initialize_mock_stats() -> model::Stats {
        let stats: model::Stats = model::Stats::new(
            model::GlobalStats {
                advance: 1,
                march: 1,
                discipline: 1,
            },
            model::DefensiveStats {
                health_points: 1,
                defense: 1,
                resilience: 1,
                armour: 1,
                aegis: 1,
            },
            model::OffensiveStats {
                attack: 1,
                offensive: 1,
                strength: 1,
                armour_penetration: 1,
                agility: 1,
            },
        );
        stats
    }

    /// # Initialize the modifier stats structs with some values given as parameters
    ///
    /// # Parameters
    /// advance (usize): the advance modifier statistic of the model
    ///
    /// march (usize): the march modifier statistic of the model
    ///
    /// discipline (usize): the discipline modifier statistic of the model
    ///
    /// health_point (usize): the health_point modifier statistic of the model
    ///
    /// defense (usize): the defense modifier statistic of the model
    ///
    /// resilience (usize): the resilience modifier statistic of the model
    ///
    /// armour (usize): the armour modifier statistic of the model
    ///
    /// aegis (usize): the aegis modifier statistic of the model
    ///
    /// attack (usize): the attack modifier statistic of the model
    ///
    /// offensive (usize): the offensive modifier statistic of the model
    ///
    /// strength (usize): the strength modifier statistic of the model
    ///
    /// armour_penetration (usize): the armour_penetration modifier statistic of the model
    ///
    /// agility (usize): the agility modifier statistic of the model
    ///
    /// # Return
    /// Modifier Stats: The modifier stats created and initialized

    pub fn initialize_modifier_stats(
        advance: usize,
        march: usize,
        discipline: usize,
        health_points: usize,
        defense: usize,
        resilience: usize,
        armour: usize,
        aegis: usize,
        attack: usize,
        offensive: usize,
        strength: usize,
        armour_penetration: usize,
        agility: usize,
    ) -> model::Stats {
        let modifier_stats: model::Stats = model::Stats::new(
            model::GlobalStats {
                advance: advance,
                march: march,
                discipline: discipline,
            },
            model::DefensiveStats {
                health_points: health_points,
                defense: defense,
                resilience: resilience,
                armour: armour,
                aegis: aegis,
            },
            model::OffensiveStats {
                attack: attack,
                offensive: offensive,
                strength: strength,
                armour_penetration: armour_penetration,
                agility: agility,
            },
        );
        modifier_stats
    }

    /// #  Initialize all modifier stats structs values to 0
    ///
    /// # Return
    /// Modifier Stats: Our mock modifier Stats struct
    pub fn initialize_mock_modifier_stats() -> model::Stats {
        let modifier_stats: model::Stats = model::Stats::new(
            model::GlobalStats {
                advance: 0,
                march: 0,
                discipline: 0,
            },
            model::DefensiveStats {
                health_points: 0,
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
        modifier_stats
    }

    /// # Initialize a regiment according to the parameters
    ///
    /// # Parameters
    /// model_stats (model::Stats): the stats of a Model
    ///
    /// nb_rows (usize): the number of rows of the regiment
    ///
    /// nb_cols (usize): the number of columns of the regiment
    ///
    /// nb_models (usize): the number of models in the regiment
    ///
    /// banner (bool): if the regiment has a banner or not
    ///
    /// # Return
    /// Regiment : a regiment struct initialized
    pub fn initialize_regiment(
        model_stats: model::Stats,
        nb_rows: usize,
        nb_cols: usize,
        nb_models: usize,
        banner: bool,
    ) -> regiment::Regiment {
        let stats: model::Stats = model_stats;

        let modifier: modifier::Modifier = modifier::Modifier::new_weapon(None, 0, 0);
        let model: model::Model = model::Model::new(stats, vec![modifier], banner);
        let regiment: regiment::Regiment =
            regiment::Regiment::new(model, nb_rows, nb_cols, nb_models, None);
        regiment
    }

    /// Initialize a model with a stats that values 1 for each of it's fields and and a dummy weapon
    ///
    /// ## Return
    /// model: Our mock model struct
    pub fn initialize_mock_model() -> model::Model {
        let stats: model::Stats = initialize_mock_stats();

        let modifier: modifier::Modifier = modifier::Modifier::new_weapon(None, 0, 0);
        let model: model::Model = model::Model::new(stats, vec![modifier], false);
        model
    }

    /// Initialize a regiment with a model that have 1 for each element of it's stat and a dummy weapon
    ///
    /// ## Return
    /// regiment: Our mock modifier Stats struct
    pub fn initialize_mock_regiment() -> regiment::Regiment {
        let stats: model::Stats = initialize_mock_stats();

        let modifier: modifier::Modifier = modifier::Modifier::new_weapon(None, 0, 0);
        let model: model::Model = model::Model::new(stats, vec![modifier], false);
        let regiment: regiment::Regiment = regiment::Regiment::new(model, 1, 1, 1, None);
        regiment
    }
}
