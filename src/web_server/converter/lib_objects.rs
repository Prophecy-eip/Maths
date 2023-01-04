//! # web converter module
//!
//! This module contains all the structs and datas needed to convert the data from the library to the web server

use crate::web_server;

pub fn modifier_converter(
    modifier: &crate::modifier::Modifier,
) -> web_server::data_structures::modifier::Modifier {
    match modifier {
        crate::modifier::Modifier::Global(global_modifier) => {
            web_server::data_structures::modifier::Modifier::Global(
                web_server::data_structures::modifier::GlobalModifier {
                    advance: global_modifier.get_advance(),
                    march: global_modifier.get_march(),
                    discipline: global_modifier.get_discipline(),
                },
            )
        }
        crate::modifier::Modifier::Offensive(offensive_modifier) => {
            web_server::data_structures::modifier::Modifier::Offensive(
                web_server::data_structures::modifier::OffensiveModifier {
                    attack: offensive_modifier.get_attack(),
                    offensive: offensive_modifier.get_offensive(),
                    strength: offensive_modifier.get_strength(),
                    armour_penetration: offensive_modifier.get_armour_penetration(),
                    agility: offensive_modifier.get_agility(),
                },
            )
        }
        crate::modifier::Modifier::Defensive(defensive_modifier) => {
            web_server::data_structures::modifier::Modifier::Defensive(
                web_server::data_structures::modifier::DefensiveModifier {
                    health_points: defensive_modifier.get_health_points(),
                    defense: defensive_modifier.get_defense(),
                    resilience: defensive_modifier.get_resilience(),
                    armour: defensive_modifier.get_armour(),
                },
            )
        }
        crate::modifier::Modifier::Weapon(weapon_modifier) => {
            web_server::data_structures::modifier::Modifier::Weapon(
                web_server::data_structures::modifier::WeaponModifier {
                    shots: weapon_modifier.get_shots(),
                    strength: weapon_modifier.get_strength(),
                    armour_penetration: weapon_modifier.get_armour_penetration(),
                },
            )
        }
    }
}

pub fn model_converter(model: &crate::model::Model) -> web_server::data_structures::model::Model {
    let mut web_model = web_server::data_structures::model::Model {
        stats: web_server::data_structures::model::Stats {
            advance: model.get_stats().get_advance(),
            march: model.get_stats().get_march(),
            discipline: model.get_stats().get_discipline(),
            health_points: model.get_stats().get_health_points(),
            defense: model.get_stats().get_defense(),
            resilience: model.get_stats().get_resilience(),
            armour: model.get_stats().get_armour(),
            aegis: model.get_stats().get_aegis(),
            attack: model.get_stats().get_attack(),
            offensive: model.get_stats().get_offensive(),
            strength: model.get_stats().get_strength(),
            armour_penetration: model.get_stats().get_armour_penetration(),
            agility: model.get_stats().get_agility(),
        },
        modifiers: vec![],
    };
    for modifier in model.get_modifiers() {
        web_model.modifiers.push(modifier_converter(modifier));
    }
    web_model
}

pub fn regiment_converter(
    regiment: &crate::regiment::Regiment,
) -> web_server::data_structures::regiment::Regiment {
    web_server::data_structures::regiment::Regiment {
        model: model_converter(regiment.get_model()),
        nb_rows: regiment.get_rows(),
        nb_cols: regiment.get_cols(),
        nb_models: regiment.get_nb_models(),
    }
}
