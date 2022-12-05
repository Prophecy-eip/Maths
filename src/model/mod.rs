//! # Model module
//!
//! This module contain all the structs and datas needed by a Model
//! A Model is one of the figurines in a Regiment
//! It holds Stats and Modifiers

use crate::modifier;

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
    /// Get the Stats of the Model
    ///
    /// ## Return
    /// Stats: The Stats of the Model
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
}
