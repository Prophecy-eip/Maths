//! # Test module
//!
//! This module contain all the initialization and global functions needed by the tests

use crate::{model, regiment};


pub fn initialize_stats(advance: usize, march: usize, discipline: usize, health_point: usize, defense: usize, resilience: usize, armour: usize, aegis: usize, attack: usize, offensive: usize, strength:usize, armour_penetration: usize, agility: usize) -> model::Stats {
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

pub fn initialize_stats_with_1() -> model::Stats {
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

pub fn initialize_modifier_stats(advance: usize, march: usize, discipline: usize, health_point: usize, defense: usize, resilience: usize, armour: usize, aegis: usize, attack: usize, offensive: usize, strength:usize, armour_penetration: usize, agility: usize) -> model::Stats {
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

pub fn initialize_modifier_stats_with_0() -> model::Stats {
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

pub fn initialize_regiment(model_stats: model::Stats, model_modifier_stats: model::Stats, nb_rows: usize, nb_cols: usize, nb_models: usize) -> regiment::Regiment {
    let stats: model::Stats = model_stats;
    let modifier_stats: model::Stats = model_modifier_stats;

    let modifier: model::Modifier =
        model::Modifier::new(modifier_stats, false, 0, vec![]);
    let model: model::Model =
        model::Model::new(stats, vec![modifier]);
    let regiment: regiment::Regiment =
        regiment::Regiment::new(model, nb_rows, nb_cols, nb_models);
    regiment
}