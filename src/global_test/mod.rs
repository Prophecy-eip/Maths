//! # Test module
//!
//! This module contain all the initialization and global functions needed by the tests
#[cfg(test)]
pub mod tests {
    use crate::{model, regiment};

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
    /// # Return
    /// Stats Model: The Stats created and initialized

    pub fn initialize_stats(
        advance: usize,
        march: usize,
        discipline: usize,
        health_point: usize,
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
                health_point: health_point,
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
                health_point: 1,
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
    /// # Return
    /// Modifier Stats: The modifier stats created and initialized

    pub fn initialize_modifier_stats(
        advance: usize,
        march: usize,
        discipline: usize,
        health_point: usize,
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
                health_point: health_point,
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
        modifier_stats
    }

    /// # Initialize a regiment according to the parameters
    ///
    /// # Parameters
    /// model_stats (model::Stats): the stats of a Model
    ///
    /// model_modifier_stats (model::Stats): the modifier stats of a Model
    ///
    /// nb_rows (usize): the number of rows of the regiment
    ///
    /// nb_cols (usize): the number of columns of the regiment
    ///
    /// nb_models (usize): the number of models in the regiment
    /// # Return
    /// Regiment : a regiment struct initialized

    pub fn initialize_regiment(
        model_stats: model::Stats,
        model_modifier_stats: model::Stats,
        nb_rows: usize,
        nb_cols: usize,
        nb_models: usize,
    ) -> regiment::Regiment {
        let stats: model::Stats = model_stats;
        let modifier_stats: model::Stats = model_modifier_stats;

        let modifier: model::Modifier = model::Modifier::new(modifier_stats, false, 0, vec![]);
        let model: model::Model = model::Model::new(stats, vec![modifier]);
        let regiment: regiment::Regiment =
            regiment::Regiment::new(model, nb_rows, nb_cols, nb_models);
        regiment
    }
}

    let modifier: model::Modifier =
        model::Modifier::new(modifier_stats, false, 0, vec![]);
    let model: model::Model =
        model::Model::new(stats, vec![modifier]);
    let regiment: regiment::Regiment =
        regiment::Regiment::new(model, nb_rows, nb_cols, nb_models);
    regiment
}