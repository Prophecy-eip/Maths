//! # Model module
//!
//! This module contain all the structs and datas needed by a Model
//! A Model is one of the figurines in a Regiment
//! It holds Stats and Modifiers

use crate::{math_tools, modifier};

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

    fn buff_global_modifier(&mut self, modifier: &modifier::GlobalModifier) -> &Stats {
        self.advance = math_tools::safe_add_signed_usigned(self.advance, modifier.get_advance());
        self.march = math_tools::safe_add_signed_usigned(self.march, modifier.get_march());
        self.discipline =
            math_tools::safe_add_signed_usigned(self.discipline, modifier.get_discipline());
        self
    }

    fn buff_offensive_modifier(&mut self, modifier: &modifier::OffensiveModifier) -> &Stats {
        self.attack = math_tools::safe_add_signed_usigned(self.attack, modifier.get_attack());
        self.offensive =
            math_tools::safe_add_signed_usigned(self.offensive, modifier.get_offensive());
        self.strength = math_tools::safe_add_signed_usigned(self.strength, modifier.get_strength());
        self.armour_penetration = math_tools::safe_add_signed_usigned(
            self.armour_penetration,
            modifier.get_armour_penetration(),
        );
        self.agility = math_tools::safe_add_signed_usigned(self.agility, modifier.get_agility());
        self
    }

    fn buff_defensive_modifier(&mut self, modifier: &modifier::DefensiveModifier) -> &Stats {
        self.health_points =
            math_tools::safe_add_signed_usigned(self.health_points, modifier.get_health_points());
        self.defense = math_tools::safe_add_signed_usigned(self.defense, modifier.get_defense());
        self.resilience =
            math_tools::safe_add_signed_usigned(self.resilience, modifier.get_resilience());
        self.armour = math_tools::safe_add_signed_usigned(self.armour, modifier.get_armour());
        self
    }

    /// Buff the Stats with the given melee modifier
    ///
    /// # Arguments
    /// modifier (&modifier::MeleeWeaponModifier): The modifier to apply
    ///
    /// # Return
    /// Stats: The Stats buffed
    fn buff_melee_weapon(&mut self, weapon: &modifier::MeleeWeaponModifier) -> &Stats {
        self.armour_penetration = math_tools::safe_add_signed_usigned(
            self.armour_penetration,
            weapon.get_armour_penetration(),
        );
        self.strength = math_tools::safe_add_signed_usigned(self.strength, weapon.get_strength());
        self
    }

    /// Buff the Stats with the given ranged modifier
    ///
    /// # Arguments
    /// modifier (&modifier::RangedWeaponModifier): The modifier to apply
    ///
    /// # Return
    /// Stats: The Stats of the Model with the modifier applied
    fn buff_ranged_weapon(&mut self, weapon: &modifier::RangedWeaponModifier) -> &Stats {
        self.armour_penetration = math_tools::safe_add_signed_usigned(
            self.armour_penetration,
            weapon.get_armour_penetration(),
        );
        self.strength = math_tools::safe_add_signed_usigned(self.strength, weapon.get_strength());
        self
    }

    pub fn apply_modifier(&mut self, modifier: &modifier::Modifier) -> &Stats {
        match modifier {
            modifier::Modifier::MeleeWeapon(weapon) => self.buff_melee_weapon(weapon),
            modifier::Modifier::RangedWeapon(weapon) => self.buff_ranged_weapon(weapon),
            modifier::Modifier::Global(modifier) => self.buff_global_modifier(modifier),
            modifier::Modifier::Offensive(modifier) => self.buff_offensive_modifier(modifier),
            modifier::Modifier::Defensive(modifier) => self.buff_defensive_modifier(modifier),
        }
    }
}

/// Struct containing all the information about a Model
///
/// ## Attributes
/// stats (Stats): The statistics of the Model
///
/// modifiers (Vec<Modifier>): The list of Modifier the Model have
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Model {
    stats: Stats,
    modifiers: Vec<modifier::Modifier>,
}

impl Model {
    /// Create a new Model using Stats and list of Modifiers
    ///
    /// ## Parameters
    /// (Stats) stats: The model stats
    /// (Vec<Modifier>) modifiers: The modifiers applied on the model
    ///
    /// ## Return
    /// Model: The Model created
    pub fn new(stats: Stats, modifiers: Vec<modifier::Modifier>) -> Model {
        Model { stats, modifiers }
    }
    /// Get the Stats of the Model taking account of the modifiers
    ///
    /// ## Return
    /// Stats: The Stats of the Model
    pub fn get_stats(&self) -> Stats {
        let mut copy = self.stats.clone();

        for modifier in &self.modifiers {
            copy.apply_modifier(modifier);
        }
        copy
    }

    /// Get the Stats of the Model without taking account of the modifiers
    ///
    /// ## Return
    /// &Stats: The Stats of the Model
    pub fn get_pure_stats(&self) -> &Stats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::{DefensiveStats, GlobalStats, OffensiveStats, Stats};

    fn initialize_stats() -> Stats {
        Stats::new(
            GlobalStats {
                advance: 1,
                march: 1,
                discipline: 1,
            },
            DefensiveStats {
                health_points: 1,
                defense: 1,
                resilience: 1,
                armour: 1,
                aegis: 1,
            },
            OffensiveStats {
                attack: 1,
                offensive: 1,
                strength: 1,
                armour_penetration: 1,
                agility: 1,
            },
        )
    }

    #[test]
    fn stat_get_advance() {
        let stats: Stats = initialize_stats();
        assert_eq!(stats.get_advance(), 1);
    }

    #[test]
    fn stat_get_march() {
        let stats: Stats = initialize_stats();
        assert_eq!(stats.get_march(), 1);
    }

    #[test]
    fn stat_get_discipline() {
        let stats: Stats = initialize_stats();
        assert_eq!(stats.get_discipline(), 1);
    }

    #[test]
    fn stat_get_health_points() {
        let stats: Stats = initialize_stats();
        assert_eq!(stats.get_health_points(), 1);
    }

    #[test]
    fn stat_get_defense() {
        let stats: Stats = initialize_stats();
        assert_eq!(stats.get_defense(), 1);
    }

    #[test]
    fn stat_get_resilience() {
        let stats: Stats = initialize_stats();
        assert_eq!(stats.get_resilience(), 1);
    }

    #[test]
    fn stat_get_armour() {
        let stats: Stats = initialize_stats();
        assert_eq!(stats.get_armour(), 1);
    }

    #[test]
    fn stat_get_aegis() {
        let stats: Stats = initialize_stats();
        assert_eq!(stats.get_aegis(), 1);
    }

    #[test]
    fn stat_get_attack() {
        let stats: Stats = initialize_stats();
        assert_eq!(stats.get_attack(), 1);
    }

    #[test]
    fn stat_get_offensive() {
        let stats: Stats = initialize_stats();
        assert_eq!(stats.get_offensive(), 1);
    }

    #[test]
    fn stat_get_strength() {
        let stats: Stats = initialize_stats();
        assert_eq!(stats.get_strength(), 1);
    }

    #[test]
    fn stat_get_armour_penetration() {
        let stats: Stats = initialize_stats();
        assert_eq!(stats.get_armour_penetration(), 1);
    }

    #[test]
    fn stat_get_agility() {
        let stats: Stats = initialize_stats();
        assert_eq!(stats.get_agility(), 1);
    }

    #[test]
    fn stat_buff_melee_weapon() {
        let mut stats: Stats = initialize_stats();
        stats.buff_melee_weapon(&super::modifier::MeleeWeaponModifier::new(1, 1));
        assert_eq!(stats.get_armour_penetration(), 2);
        assert_eq!(stats.get_strength(), 2);
    }

    #[test]
    fn stat_buff_ranged_weapon() {
        let mut stats: Stats = initialize_stats();
        stats.buff_ranged_weapon(&super::modifier::RangedWeaponModifier::new(1, 1, 3, 4));
        assert_eq!(stats.get_armour_penetration(), 5);
        assert_eq!(stats.get_strength(), 4);
    }

    #[test]
    fn model_get_stats() {
        let stats: Stats = initialize_stats();
        let copy = stats.clone();

        let model = super::Model::new(stats, vec![]);
        assert_eq!(model.get_stats(), copy);
    }

    #[test]
    fn model_get_stats_with_modifier() {
        let stats: Stats = initialize_stats();
        let mut copy = stats.clone();
        copy.buff_melee_weapon(&super::modifier::MeleeWeaponModifier::new(1, 1));

        let model = super::Model::new(
            stats,
            vec![super::modifier::Modifier::MeleeWeapon(
                super::modifier::MeleeWeaponModifier::new(1, 1),
            )],
        );
        assert_eq!(model.get_stats(), copy);
    }
}
