//TODO describe the module

use crate::math_tools;


/// Struct containing all the statistics in the game for a Model
///
/// ## Attributes
/// advance (usize): The distance the Model can advance per turn
///
/// march (usize): The distance the Model can forcefully advance per turn
///
/// discipline (usize): The discipline of the Model
///
/// health_points (usize): The number of hit the Model can endure before being removed from the Regiment
///
/// defense (usize): The defense of the Model
///
/// resilience (usize): The resilience of the Model
///
/// armour (usize): The armour of the Model
///
/// aegis (usize): The special armour of the Model
///
/// attack (usize): The number of attack the Model do
///
/// offensive (usize): The offensive of the Model
///
/// strength (usize): The strength of the Model
///
/// amour_penetration (usize): The strength of the Model
///
/// agility (usize): The agility of the Model
///
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Stats {
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
}

/// Struct containing the global stats of a model
///
/// ## Attributes
/// advance (usize): The distance the Model can advance per turn
///
/// march (usize): The distance the Model can forcefully advance per turn
///
/// discipline (usize): The discipline of the Model
pub struct GlobalStats {
    pub advance: usize,
    pub march: usize,
    pub discipline: usize,
}

/// Struct containing the defending stats of a model
///
/// ## Attributes
/// health_points (usize): The number of hit the Model can endure before being removed from the Regiment
///
/// defense (usize): The defense of the Model
///
/// resilience (usize): The resilience of the Model
///
/// armour (usize): The armour of the Model
///
/// aegis (usize): The special armour of the Model
pub struct DefensiveStats {
    pub health_points: usize,
    pub defense: usize,
    pub resilience: usize,
    pub armour: usize,
    pub aegis: usize,
}

/// Struct containing the offending stats of a model
///
/// ## Attributes
/// attack (usize): The number of attack the Model do
///
/// offensive (usize): The offensive of the Model
///
/// strength (usize): The strength of the Model
///
/// amour_penetration (usize): The strength of the Model
///
/// agility (usize): The agility of the Model
pub struct OffensiveStats {
    pub attack: usize,
    pub offensive: usize,
    pub strength: usize,
    pub armour_penetration: usize,
    pub agility: usize,
}

impl Stats {
    /// Create new Stats
    ///
    /// # Return
    /// Stats: The Stats created
    pub fn new(
        GlobalStats {
            advance,
            march,
            discipline,
        }: GlobalStats,
        DefensiveStats {
            health_points,
            defense,
            resilience,
            armour,
            aegis,
        }: DefensiveStats,
        OffensiveStats {
            attack,
            offensive,
            strength,
            armour_penetration,
            agility,
        }: OffensiveStats,
    ) -> Stats {
        Stats {
            advance,
            march,
            discipline,
            health_points,
            defense,
            resilience,
            armour,
            aegis,
            attack,
            offensive,
            strength,
            armour_penetration,
            agility,
        }
    }
    /// Get the advance rate of the Model
    ///
    /// ## Return
    /// advance (usize): The advance rate of the Model
    pub fn get_advance(&self) -> usize {
        self.advance
    }

    /// Get the march rate of the Model
    ///
    /// ## Return
    /// march (usize): The march rate of the Model
    pub fn get_march(&self) -> usize {
        self.march
    }

    /// Get the discipline of the Model
    ///
    /// ## Return
    /// discipline (usize): The discipline of the Model
    pub fn get_discipline(&self) -> usize {
        self.discipline
    }

    /// Get the health point of the Model
    ///
    /// ## Return
    /// health_points (usize): The health point of the Model
    pub fn get_health_points(&self) -> usize {
        self.health_points
    }

    /// Get the defense of the Model
    ///
    /// ## Return
    /// defense (usize): The defense of the Model
    pub fn get_defense(&self) -> usize {
        self.defense
    }

    /// Get the resilience of the Model
    ///
    /// ## Return
    /// resilience (usize): The resilience of the Model
    pub fn get_resilience(&self) -> usize {
        self.resilience
    }

    /// Get the armour of the Model
    ///
    /// ## Return
    /// armour (usize): The armour of the Model
    pub fn get_armour(&self) -> usize {
        self.armour
    }

    /// Get the aegis of the Model
    ///
    /// ## Return
    /// aegis (usize): The aegis of the Model
    pub fn get_aegis(&self) -> usize {
        self.aegis
    }

    /// Get the attack of the Model
    ///
    /// ## Return
    /// attack (usize): The attack of the Model
    pub fn get_attack(&self) -> usize {
        self.attack
    }

    /// Get the offensive of the Model
    ///
    /// ## Return
    /// offensive (usize): The offensive of the Model
    pub fn get_offensive(&self) -> usize {
        self.offensive
    }

    /// Get the strength of the Model
    ///
    /// ## Return
    /// strength (usize): The strength of the Model
    pub fn get_strength(&self) -> usize {
        self.strength
    }

    /// Get the armour penetration of the Model
    ///
    /// ## Return
    /// armour_penetration (usize): The armour penetration of the Model
    pub fn get_armour_penetration(&self) -> usize {
        self.armour_penetration
    }

    /// Get the agility of the Model
    ///
    /// ## Return
    /// agility (usize): The agility of the Model
    pub fn get_agility(&self) -> usize {
        self.agility
    }

    /// Buff the Stats with a GlobalModifier
    ///
    /// ## Parameters
    /// (&modifier::GlobalModifier) modifier: The GlobalModifier to buff the Stats with
    fn apply_global_modifier(&mut self, modifier: &crate::modifier::GlobalModifier) {
        self.advance = math_tools::safe_add_signed_unsigned(self.advance, modifier.get_advance());
        self.march = math_tools::safe_add_signed_unsigned(self.march, modifier.get_march());
        self.discipline =
            math_tools::safe_add_signed_unsigned(self.discipline, modifier.get_discipline());
    }

    /// Buff the Stats with a OffensiveModifier
    ///
    /// ## Parameters
    /// (&modifier::OffensiveModifier) modifier: The OffensiveModifier to buff the Stats with
    fn apply_offensive_modifier(&mut self, modifier: &crate::modifier::OffensiveModifier) {
        self.attack = math_tools::safe_add_signed_unsigned(self.attack, modifier.get_attack());
        self.offensive =
            math_tools::safe_add_signed_unsigned(self.offensive, modifier.get_offensive());
        self.strength =
            math_tools::safe_add_signed_unsigned(self.strength, modifier.get_strength());
        self.armour_penetration = math_tools::safe_add_signed_unsigned(
            self.armour_penetration,
            modifier.get_armour_penetration(),
        );
        self.agility = math_tools::safe_add_signed_unsigned(self.agility, modifier.get_agility());
    }

    /// Buff the Stats with a DefensiveModifier
    ///
    /// ## Parameters
    /// (&modifier::DefensiveModifier) modifier: The DefensiveModifier to buff the Stats with
    fn apply_defensive_modifier(&mut self, modifier: &crate::modifier::DefensiveModifier) {
        self.health_points =
            math_tools::safe_add_signed_unsigned(self.health_points, modifier.get_health_points());
        self.defense = math_tools::safe_add_signed_unsigned(self.defense, modifier.get_defense());
        self.resilience =
            math_tools::safe_add_signed_unsigned(self.resilience, modifier.get_resilience());
        self.armour = math_tools::safe_add_signed_unsigned(self.armour, modifier.get_armour());
    }

    /// Buff the Stats with the given weapon modifier
    ///
    /// ## Parameters
    /// (&modifier::WeaponModifier) modifier : The modifier to apply
    fn apply_weapon_modifier(&mut self, weapon: &crate::modifier::WeaponModifier) {
        self.armour_penetration = math_tools::safe_add_signed_unsigned(
            self.armour_penetration,
            weapon.get_armour_penetration(),
        );
        self.strength = math_tools::safe_add_signed_unsigned(self.strength, weapon.get_strength());
    }

    /// Apply the given modifier to the Stats
    ///
    /// ## Parameters
    /// (&modifier::Modifier) modifier: The Modifier to apply
    pub fn apply_modifier(&mut self, modifier: &crate::modifier::Modifier) {
        match modifier {
            crate::modifier::Modifier::Weapon(weapon) => self.apply_weapon_modifier(weapon),
            crate::modifier::Modifier::Global(modifier) => self.apply_global_modifier(modifier),
            crate::modifier::Modifier::Offensive(modifier) => self.apply_offensive_modifier(modifier),
            crate::modifier::Modifier::Defensive(modifier) => self.apply_defensive_modifier(modifier),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::global_test;
//     use super::Stats;

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
//         stats.apply_weapon_modifier(&crate::modifier::WeaponModifier::new(Some(1), 3, 4));
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
