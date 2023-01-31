//! # web converter module
//!
//! This module contains all the structs and datas needed to convert the data from the web server to the library

use crate::web_server;

/// Create a modifier usable by the library from a web modifier
///
/// ## Parameters
/// (web_server::data_structures::modifier::Modifier) web_modifier: The web modifier to convert
///
/// ## Return
/// (crate::modifier::Modifier) The converted modifier
pub fn modifier_converter(
    web_modifier: &web_server::data_structures::modifier::Modifier,
) -> maths::modifier::Modifier {
    match web_modifier {
        web_server::data_structures::modifier::Modifier::Global(web_global_modifier) => {
            maths::modifier::Modifier::new_global(
                web_global_modifier.advance,
                web_global_modifier.march,
                web_global_modifier.discipline,
            )
        }
        web_server::data_structures::modifier::Modifier::Offensive(web_offensive_modifier) => {
            maths::modifier::Modifier::new_offensive(
                web_offensive_modifier.attack,
                web_offensive_modifier.offensive,
                web_offensive_modifier.strength,
                web_offensive_modifier.armour_penetration,
                web_offensive_modifier.agility,
            )
        }
        web_server::data_structures::modifier::Modifier::Defensive(web_defensive_modifier) => {
            maths::modifier::Modifier::new_defensive(
                web_defensive_modifier.health_points,
                web_defensive_modifier.defense,
                web_defensive_modifier.resilience,
                web_defensive_modifier.armour,
            )
        }
        web_server::data_structures::modifier::Modifier::Weapon(web_weapon_modifier) => {
            maths::modifier::Modifier::new_weapon(
                web_weapon_modifier.shots,
                web_weapon_modifier.strength,
                web_weapon_modifier.armour_penetration,
            )
        }
    }
}

/// Create a model usable by the library from a web model
///
/// ## Parameters
/// (web_server::data_structures::model::Model) web_model: The web model to convert
///
/// ## Return
/// (crate::model::Model) The converted model
pub fn model_converter(
    web_model: &web_server::data_structures::model::Model,
) -> maths::model::Model {
    let mut model: maths::model::Model = maths::model::Model::new(
        maths::stat::Stats::new(
            maths::stat::GlobalStats {
                advance: web_model.stats.advance,
                march: web_model.stats.march,
                discipline: web_model.stats.discipline,
            },
            maths::stat::DefensiveStats {
                health_points: web_model.stats.health_points,
                defense: web_model.stats.defense,
                resilience: web_model.stats.resilience,
                armour: web_model.stats.armour,
                aegis: web_model.stats.aegis,
            },
            maths::stat::OffensiveStats {
                attack: web_model.stats.attack,
                offensive: web_model.stats.offensive,
                strength: web_model.stats.strength,
                armour_penetration: web_model.stats.armour_penetration,
                agility: web_model.stats.agility,
            },
        ),
        vec![],
        web_model.banner_bearer,
    );

    for web_modifier in &web_model.modifiers {
        model.add_modifier(modifier_converter(web_modifier));
    }

    model
}

/// Create a regiment usable by the library from a web regiment
///
/// ## Parameters
/// (web_server::data_structures::regiment::Regiment) web_regiment: The web regiment to convert
///
/// ## Return
/// (crate::regiment::Regiment) The converted regiment
pub fn regiment_converter(
    web_regiment: &web_server::data_structures::regiment::Regiment,
) -> maths::regiment::Regiment {
    maths::regiment::Regiment::new(
        model_converter(&web_regiment.model),
        web_regiment.nb_rows,
        web_regiment.nb_cols,
        web_regiment.nb_models,
        None,
    )
}

pub fn attacking_position_converter(attacking_position: String) -> maths::fight::AttackPosition {
    match attacking_position.as_str() {
        "front" => maths::fight::AttackPosition::FRONT,
        "flank" => maths::fight::AttackPosition::FLANK,
        "back" => maths::fight::AttackPosition::BACK,
        _ => maths::fight::AttackPosition::FRONT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web_server::data_structures::model::Model as WebModel;
    use crate::web_server::data_structures::modifier::Modifier as WebModifier;
    use crate::web_server::data_structures::regiment::Regiment as WebRegiment;

    #[test]
    fn test_modifier_converter() {
        let web_modifier: WebModifier =
            WebModifier::Global(web_server::data_structures::modifier::GlobalModifier {
                advance: 1,
                march: 2,
                discipline: 3,
            });
        let modifier: maths::modifier::Modifier = modifier_converter(&web_modifier);
        assert_eq!(modifier, maths::modifier::Modifier::new_global(1, 2, 3));

        let web_modifier: WebModifier =
            WebModifier::Offensive(web_server::data_structures::modifier::OffensiveModifier {
                attack: 1,
                offensive: 2,
                strength: 3,
                armour_penetration: 4,
                agility: 5,
            });
        let modifier: maths::modifier::Modifier = modifier_converter(&web_modifier);
        assert_eq!(
            modifier,
            maths::modifier::Modifier::new_offensive(1, 2, 3, 4, 5)
        );

        let web_modifier: WebModifier =
            WebModifier::Defensive(web_server::data_structures::modifier::DefensiveModifier {
                health_points: 1,
                defense: 2,
                resilience: 3,
                armour: 4,
            });
        let modifier: maths::modifier::Modifier = modifier_converter(&web_modifier);
        assert_eq!(
            modifier,
            maths::modifier::Modifier::new_defensive(1, 2, 3, 4)
        );

        let web_modifier: WebModifier =
            WebModifier::Weapon(web_server::data_structures::modifier::WeaponModifier {
                shots: None,
                strength: 2,
                armour_penetration: 3,
            });
        let modifier: maths::modifier::Modifier = modifier_converter(&web_modifier);
        assert_eq!(modifier, maths::modifier::Modifier::new_weapon(None, 2, 3));
    }

    #[test]
    fn test_model_converter() {
        let web_model: WebModel = WebModel {
            stats: web_server::data_structures::model::Stats {
                advance: 1,
                march: 2,
                discipline: 3,
                health_points: 4,
                defense: 5,
                resilience: 6,
                armour: 7,
                aegis: 8,
                attack: 9,
                offensive: 10,
                strength: 11,
                armour_penetration: 12,
                agility: 13,
            },
            modifiers: vec![],
            banner_bearer: false,
        };
        let model: maths::model::Model = model_converter(&web_model);
        assert_eq!(
            model,
            maths::model::Model::new(
                maths::stat::Stats::new(
                    maths::stat::GlobalStats {
                        advance: 1,
                        march: 2,
                        discipline: 3,
                    },
                    maths::stat::DefensiveStats {
                        health_points: 4,
                        defense: 5,
                        resilience: 6,
                        armour: 7,
                        aegis: 8,
                    },
                    maths::stat::OffensiveStats {
                        attack: 9,
                        offensive: 10,
                        strength: 11,
                        armour_penetration: 12,
                        agility: 13,
                    },
                ),
                vec![],
                false
            )
        );
    }

    #[test]
    fn test_regiment_converter() {
        let web_regiment: WebRegiment = WebRegiment {
            model: web_server::data_structures::model::Model {
                stats: web_server::data_structures::model::Stats {
                    advance: 1,
                    march: 2,
                    discipline: 3,
                    health_points: 4,
                    defense: 5,
                    resilience: 6,
                    armour: 7,
                    aegis: 8,
                    attack: 9,
                    offensive: 10,
                    strength: 11,
                    armour_penetration: 12,
                    agility: 13,
                },
                modifiers: vec![],
                banner_bearer: false,
            },
            nb_rows: 1,
            nb_cols: 2,
            nb_models: 3,
        };
        let regiment: maths::regiment::Regiment = regiment_converter(&web_regiment);
        assert_eq!(
            regiment,
            maths::regiment::Regiment::new(
                maths::model::Model::new(
                    maths::stat::Stats::new(
                        maths::stat::GlobalStats {
                            advance: 1,
                            march: 2,
                            discipline: 3,
                        },
                        maths::stat::DefensiveStats {
                            health_points: 4,
                            defense: 5,
                            resilience: 6,
                            armour: 7,
                            aegis: 8,
                        },
                        maths::stat::OffensiveStats {
                            attack: 9,
                            offensive: 10,
                            strength: 11,
                            armour_penetration: 12,
                            agility: 13,
                        },
                    ),
                    vec![],
                    false
                ),
                1,
                2,
                3,
                None
            )
        );
    }
}
