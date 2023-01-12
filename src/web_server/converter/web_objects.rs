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
    let mut model: crate::model::Model = crate::model::Model::new(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Model;
    use crate::modifier::Modifier;
    use crate::regiment::Regiment;
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
        let modifier: Modifier = modifier_converter(&web_modifier);
        assert_eq!(modifier, Modifier::new_global(1, 2, 3));

        let web_modifier: WebModifier =
            WebModifier::Offensive(web_server::data_structures::modifier::OffensiveModifier {
                attack: 1,
                offensive: 2,
                strength: 3,
                armour_penetration: 4,
                agility: 5,
            });
        let modifier: Modifier = modifier_converter(&web_modifier);
        assert_eq!(modifier, Modifier::new_offensive(1, 2, 3, 4, 5));

        let web_modifier: WebModifier =
            WebModifier::Defensive(web_server::data_structures::modifier::DefensiveModifier {
                health_points: 1,
                defense: 2,
                resilience: 3,
                armour: 4,
            });
        let modifier: Modifier = modifier_converter(&web_modifier);
        assert_eq!(modifier, Modifier::new_defensive(1, 2, 3, 4));

        let web_modifier: WebModifier =
            WebModifier::Weapon(web_server::data_structures::modifier::WeaponModifier {
                shots: None,
                strength: 2,
                armour_penetration: 3,
            });
        let modifier: Modifier = modifier_converter(&web_modifier);
        assert_eq!(modifier, Modifier::new_weapon(None, 2, 3));
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
        };
        let model: Model = model_converter(&web_model);
        assert_eq!(
            model,
            Model::new(
                crate::model::Stats::new(
                    crate::model::GlobalStats {
                        advance: 1,
                        march: 2,
                        discipline: 3,
                    },
                    crate::model::DefensiveStats {
                        health_points: 4,
                        defense: 5,
                        resilience: 6,
                        armour: 7,
                        aegis: 8,
                    },
                    crate::model::OffensiveStats {
                        attack: 9,
                        offensive: 10,
                        strength: 11,
                        armour_penetration: 12,
                        agility: 13,
                    },
                ),
                vec![]
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
            },
            nb_rows: 1,
            nb_cols: 2,
            nb_models: 3,
        };
        let regiment: Regiment = regiment_converter(&web_regiment);
        assert_eq!(
            regiment,
            Regiment::new(
                Model::new(
                    crate::model::Stats::new(
                        crate::model::GlobalStats {
                            advance: 1,
                            march: 2,
                            discipline: 3,
                        },
                        crate::model::DefensiveStats {
                            health_points: 4,
                            defense: 5,
                            resilience: 6,
                            armour: 7,
                            aegis: 8,
                        },
                        crate::model::OffensiveStats {
                            attack: 9,
                            offensive: 10,
                            strength: 11,
                            armour_penetration: 12,
                            agility: 13,
                        },
                    ),
                    vec![]
                ),
                1,
                2,
                3,
                None
            )
        );
    }
}
