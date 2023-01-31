//! # Model module
//!
//! This module contain all the structs and datas needed by a Model
//! A Model is one of the figurines in a Regiment
//! It holds Stats and Modifiers

use crate::{modifier, stat};

/// Struct containing all the information about a Model
///
/// ## Attributes
/// stats (Stats): The statistics of the Model
///
/// boosted_stats (Stats): The statistics of the Model taking account of the modifiers
///
/// modifiers (Vec<Modifier>): The list of Modifier the Model have
///
/// banner_bearer (bool): If the Model is a banner bearer
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Model {
    stats: stat::Stats,
    boosted_stats: stat::Stats,
    modifiers: Vec<modifier::Modifier>,
    banner_bearer: bool,
}

impl Model {
    /// Create a new Model using Stats and list of Modifiers
    ///
    /// ## Parameters
    /// (Stats) stats: The model stats
    ///
    /// (Vec<Modifier>) modifiers: The modifiers applied on the model
    ///
    /// (bool) banner_bearer: If the model is a banner bearer
    ///
    /// ## Return
    /// Model: The Model created
    pub fn new(
        stats: stat::Stats,
        modifiers: Vec<modifier::Modifier>,
        banner_bearer: bool,
    ) -> Model {
        let mut boosted_stats = stats.clone();
        for modifier in &modifiers {
            boosted_stats.apply_modifier(modifier);
        }
        Model {
            stats,
            boosted_stats,
            modifiers,
            banner_bearer,
        }
    }
    /// Get the Stats of the Model taking account of the modifiers
    ///
    /// ## Return
    /// &Stats: The boosted Stats of the Model
    pub fn get_boosted_stats(&self) -> &stat::Stats {
        &self.boosted_stats
    }

    /// Get the Stats of the Model without taking account of the modifiers
    ///
    /// ## Return
    /// &Stats: The Stats of the Model
    pub fn get_stats(&self) -> &stat::Stats {
        &self.stats
    }

    /// Add a modifier to the Model
    ///
    /// ## Parameters
    ///
    /// (modifier::Modifier) modifier: The modifier to add
    pub fn add_modifier(&mut self, modifier: modifier::Modifier) {
        self.modifiers.push(modifier.clone());
        self.boosted_stats.apply_modifier(&modifier);
    }

    /// Get the list of modifiers applied on the Model
    ///
    /// ## Return
    /// &Vec<modifier::Modifier>: The list of modifiers
    pub fn get_modifiers(&self) -> &Vec<modifier::Modifier> {
        &self.modifiers
    }

    /// Tells if the model owns a banner
    ///
    /// ## Return
    /// bool: If the model is a banner bearer
    pub fn is_banner_bearer(&self) -> bool {
        self.banner_bearer
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::global_test;

//     #[test]
//     fn stat_get_advance() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         assert_eq!(stats.get_advance(), 1);
//     }

//     #[test]
//     fn stat_get_march() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         assert_eq!(stats.get_march(), 1);
//     }

//     #[test]
//     fn stat_get_discipline() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         assert_eq!(stats.get_discipline(), 1);
//     }

//     #[test]
//     fn stat_get_health_point() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         assert_eq!(stats.get_health_points(), 1);
//     }

//     #[test]
//     fn stat_get_defense() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         assert_eq!(stats.get_defense(), 1);
//     }

//     #[test]
//     fn stat_get_resilience() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         assert_eq!(stats.get_resilience(), 1);
//     }

//     #[test]
//     fn stat_get_armour() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         assert_eq!(stats.get_armour(), 1);
//     }

//     #[test]
//     fn stat_get_aegis() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         assert_eq!(stats.get_aegis(), 1);
//     }

//     #[test]
//     fn stat_get_attack() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         assert_eq!(stats.get_attack(), 1);
//     }

//     #[test]
//     fn stat_get_offensive() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         assert_eq!(stats.get_offensive(), 1);
//     }

//     #[test]
//     fn stat_get_strength() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         assert_eq!(stats.get_strength(), 1);
//     }

//     #[test]
//     fn stat_get_armour_penetration() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         assert_eq!(stats.get_armour_penetration(), 1);
//     }

//     #[test]
//     fn stat_get_agility() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         assert_eq!(stats.get_agility(), 1);
//     }

//     #[test]
//     fn stat_buff_ranged_weapon() {
//         let mut stats: Stats = global_test::tests::initialize_mock_stats();
//         stats.apply_weapon_modifier(&super::modifier::WeaponModifier::new(Some(1), 3, 4));
//         assert_eq!(stats.get_armour_penetration(), 5);
//         assert_eq!(stats.get_strength(), 4);
//     }

//     #[test]
//     fn model_get_boosted_stats() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         let copy: Stats = stats.clone();

//         let model: super::Model = super::Model::new(stats, vec![], false);
//         assert_eq!(model.get_boosted_stats(), &copy);
//     }

//     #[test]
//     fn model_get_boosted_stats_with_modifier() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         let mut copy: Stats = stats.clone();
//         copy.apply_weapon_modifier(&super::modifier::WeaponModifier::new(Some(1), 1, 1));

//         let model: super::Model = super::Model::new(
//             stats,
//             vec![super::modifier::Modifier::Weapon(
//                 super::modifier::WeaponModifier::new(Some(1), 1, 1),
//             )],
//             false,
//         );
//         assert_eq!(model.get_boosted_stats(), &copy);
//     }

//     #[test]
//     fn model_get_stats() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         let model: super::Model = super::Model::new(stats.clone(), vec![], false);
//         assert_eq!(model.get_stats(), &stats);
//     }

//     #[test]
//     fn test_apply_modifier_ranged_weapon() {
//         let mut stats: Stats = global_test::tests::initialize_mock_stats();
//         let modifier: super::modifier::Modifier =
//             super::modifier::Modifier::new_weapon(Some(1), 3, 4);
//         stats.apply_modifier(&modifier);
//         assert_eq!(stats.get_armour_penetration(), 5);
//         assert_eq!(stats.get_strength(), 4);
//     }

//     #[test]
//     fn test_apply_modifier_global() {
//         let mut stats: Stats = global_test::tests::initialize_mock_stats();
//         let modifier: super::modifier::Modifier = super::modifier::Modifier::new_global(1, 1, 1);
//         stats.apply_modifier(&modifier);
//         assert_eq!(stats.get_advance(), 2);
//         assert_eq!(stats.get_march(), 2);
//         assert_eq!(stats.get_discipline(), 2);
//     }

//     #[test]
//     fn test_apply_modifier_defensive() {
//         let mut stats: Stats = global_test::tests::initialize_mock_stats();
//         let modifier: super::modifier::Modifier =
//             super::modifier::Modifier::new_defensive(1, 1, 1, 1);
//         stats.apply_modifier(&modifier);
//         assert_eq!(stats.get_health_points(), 2);
//         assert_eq!(stats.get_defense(), 2);
//         assert_eq!(stats.get_resilience(), 2);
//         assert_eq!(stats.get_armour(), 2);
//     }

//     #[test]
//     fn test_apply_modifier_offensive() {
//         let mut stats: Stats = global_test::tests::initialize_mock_stats();
//         let modifier: super::modifier::Modifier =
//             super::modifier::Modifier::new_offensive(1, 2, 3, 4, 5);
//         stats.apply_modifier(&modifier);
//         assert_eq!(stats.get_attack(), 2);
//         assert_eq!(stats.get_offensive(), 3);
//         assert_eq!(stats.get_strength(), 4);
//         assert_eq!(stats.get_armour_penetration(), 5);
//         assert_eq!(stats.get_agility(), 6);
//     }

//     #[test]
//     fn test_model_banner_bearing() {
//         let stats: Stats = global_test::tests::initialize_mock_stats();
//         let model: super::Model = super::Model::new(stats.clone(), vec![], true);
//         let second_model: super::Model = super::Model::new(stats, vec![], false);
//         assert_eq!(model.is_banner_bearer(), true);
//         assert_eq!(second_model.is_banner_bearer(), false);
//     }
// }
