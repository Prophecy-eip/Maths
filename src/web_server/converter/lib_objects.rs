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
    let mut web_model: web_server::data_structures::model::Model =
        web_server::data_structures::model::Model {
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
    fn test_modifier_global_converter() {
        let modifier: Modifier = Modifier::Global(crate::modifier::GlobalModifier::new(1, 2, 3));
        let web_modifier: WebModifier = modifier_converter(&modifier);
        match web_modifier {
            WebModifier::Global(global_modifier) => {
                assert_eq!(global_modifier.advance, 1);
                assert_eq!(global_modifier.march, 2);
                assert_eq!(global_modifier.discipline, 3);
            }
            _ => panic!("Wrong modifier type"),
        }
    }

    #[test]
    fn test_modifier_offensive_converter() {
        let modifier: Modifier =
            Modifier::Offensive(crate::modifier::OffensiveModifier::new(1, 2, 3, 4, 5));
        let web_modifier: WebModifier = modifier_converter(&modifier);
        match web_modifier {
            WebModifier::Offensive(offensive_modifier) => {
                assert_eq!(offensive_modifier.attack, 1);
                assert_eq!(offensive_modifier.offensive, 2);
                assert_eq!(offensive_modifier.strength, 3);
                assert_eq!(offensive_modifier.armour_penetration, 4);
                assert_eq!(offensive_modifier.agility, 5);
            }
            _ => panic!("Wrong modifier type"),
        }
    }

    #[test]
    fn test_modifier_defensive_converter() {
        let modifier: Modifier =
            Modifier::Defensive(crate::modifier::DefensiveModifier::new(1, 2, 3, 4));
        let web_modifier: WebModifier = modifier_converter(&modifier);
        match web_modifier {
            WebModifier::Defensive(defensive_modifier) => {
                assert_eq!(defensive_modifier.health_points, 1);
                assert_eq!(defensive_modifier.defense, 2);
                assert_eq!(defensive_modifier.resilience, 3);
                assert_eq!(defensive_modifier.armour, 4);
            }
            _ => panic!("Wrong modifier type"),
        }
    }

    #[test]
    fn test_modifier_weapon_converter() {
        let modifier: Modifier = Modifier::Weapon(crate::modifier::WeaponModifier::new(None, 2, 3));
        let web_modifier: WebModifier = modifier_converter(&modifier);
        match web_modifier {
            WebModifier::Weapon(weapon_modifier) => {
                assert_eq!(weapon_modifier.shots, None);
                assert_eq!(weapon_modifier.strength, 2);
                assert_eq!(weapon_modifier.armour_penetration, 3);
            }
            _ => panic!("Wrong modifier type"),
        }
    }

    #[test]
    fn test_model_converter() {
        let model: Model = crate::global_test::tests::initialize_mock_model();
        let web_model: WebModel = model_converter(&model);
        assert_eq!(web_model.stats.advance, model.get_stats().get_advance());
        assert_eq!(web_model.stats.march, model.get_stats().get_march());
        assert_eq!(
            web_model.stats.discipline,
            model.get_stats().get_discipline()
        );
        assert_eq!(
            web_model.stats.health_points,
            model.get_stats().get_health_points()
        );
        assert_eq!(web_model.stats.defense, model.get_stats().get_defense());
        assert_eq!(
            web_model.stats.resilience,
            model.get_stats().get_resilience()
        );
        assert_eq!(web_model.stats.armour, model.get_stats().get_armour());
        assert_eq!(web_model.stats.aegis, model.get_stats().get_aegis());
        assert_eq!(web_model.stats.attack, model.get_stats().get_attack());
        assert_eq!(web_model.stats.offensive, model.get_stats().get_offensive());
        assert_eq!(web_model.stats.strength, model.get_stats().get_strength());
        assert_eq!(
            web_model.stats.armour_penetration,
            model.get_stats().get_armour_penetration()
        );
        assert_eq!(web_model.stats.agility, model.get_stats().get_agility());
    }

    #[test]
    fn test_regiment_converter() {
        let regiment: Regiment = crate::global_test::tests::initialize_mock_regiment();
        let web_regiment: WebRegiment = regiment_converter(&regiment);
        assert_eq!(web_regiment.nb_rows, regiment.get_rows());
        assert_eq!(web_regiment.nb_cols, regiment.get_cols());
        assert_eq!(web_regiment.nb_models, regiment.get_nb_models());
        assert_eq!(web_regiment.model, model_converter(regiment.get_model()));
    }
}
