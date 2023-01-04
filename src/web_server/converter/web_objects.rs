//! # web converter module
//!
//! This module contains all the structs and datas needed to convert the data from the web server to the library

use crate::web_server;

pub fn modifier_converter(
    web_modifier: &web_server::data_structures::modifier::Modifier,
) -> crate::modifier::Modifier {
    match web_modifier {
        web_server::data_structures::modifier::Modifier::Global(web_global_modifier) => {
            crate::modifier::Modifier::new_global(
                web_global_modifier.advance,
                web_global_modifier.march,
                web_global_modifier.discipline,
            )
        }
        web_server::data_structures::modifier::Modifier::Offensive(web_offensive_modifier) => {
            crate::modifier::Modifier::new_offensive(
                web_offensive_modifier.attack,
                web_offensive_modifier.offensive,
                web_offensive_modifier.strength,
                web_offensive_modifier.armour_penetration,
                web_offensive_modifier.agility,
            )
        }
        web_server::data_structures::modifier::Modifier::Defensive(web_defensive_modifier) => {
            crate::modifier::Modifier::new_defensive(
                web_defensive_modifier.health_points,
                web_defensive_modifier.defense,
                web_defensive_modifier.resilience,
                web_defensive_modifier.armour,
            )
        }
        web_server::data_structures::modifier::Modifier::Weapon(web_weapon_modifier) => {
            crate::modifier::Modifier::new_weapon(
                web_weapon_modifier.shots,
                web_weapon_modifier.strength,
                web_weapon_modifier.armour_penetration,
            )
        }
    }
}

pub fn model_converter(
    web_model: &web_server::data_structures::model::Model,
) -> crate::model::Model {
    let mut model = crate::model::Model::new(
        crate::model::Stats::new(
            crate::model::GlobalStats {
                advance: web_model.stats.advance,
                march: web_model.stats.march,
                discipline: web_model.stats.discipline,
            },
            crate::model::DefensiveStats {
                health_points: web_model.stats.health_points,
                defense: web_model.stats.defense,
                resilience: web_model.stats.resilience,
                armour: web_model.stats.armour,
                aegis: web_model.stats.aegis,
            },
            crate::model::OffensiveStats {
                attack: web_model.stats.attack,
                offensive: web_model.stats.offensive,
                strength: web_model.stats.strength,
                armour_penetration: web_model.stats.armour_penetration,
                agility: web_model.stats.agility,
            },
        ),
        vec![],
    );

    for web_modifier in &web_model.modifiers {
        model.add_modifier(modifier_converter(web_modifier));
    }

    model
}

pub fn regiment_converter(
    web_regiment: &web_server::data_structures::regiment::Regiment,
) -> crate::regiment::Regiment {
    crate::regiment::Regiment::new(
        model_converter(&web_regiment.model),
        web_regiment.nb_rows,
        web_regiment.nb_cols,
        web_regiment.nb_models,
        None,
    )
}
