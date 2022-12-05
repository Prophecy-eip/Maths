//! # Model module
//!
//! This module contain all the structs and datas needed by a Model
//! A Model is one of the figurines in a Regiment
//! It holds Stats and Modifiers

/// Struct containing all the statistics in the game for a Model
///
/// # Attributes
/// advance (usize): The distance the Model can advance per turn
///
/// march (usize): The distance the Model can forcefully advance per turn
///
/// discipline (usize): The discipline of the Model
///
/// health_point (usize): The number of hit the Model can endure before being removed from the Regiment
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
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Stats {
    advance: usize,
    march: usize,
    discipline: usize,
    health_point: usize,
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

pub struct GlobalStats {
    pub advance: usize,
    pub march: usize,
    pub discipline: usize,
}

pub struct DefensiveStats {
    pub health_point: usize,
    pub defense: usize,
    pub resilience: usize,
    pub armour: usize,
    pub aegis: usize,
}

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
            health_point,
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
            health_point,
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
    /// # Returns
    /// advance (usize): The advance rate of the Model
    pub fn get_advance(&self) -> usize {
        self.advance
    }

    /// Get the march rate of the Model
    ///
    /// # Returns
    /// march (usize): The march rate of the Model
    pub fn get_march(&self) -> usize {
        self.march
    }

    /// Get the discipline of the Model
    ///
    /// # Returns
    /// discipline (usize): The discipline of the Model
    pub fn get_discipline(&self) -> usize {
        self.discipline
    }

    /// Get the health point of the Model
    ///
    /// # Returns
    /// health_point (usize): The health point of the Model
    pub fn get_health_point(&self) -> usize {
        self.health_point
    }

    /// Get the defense of the Model
    ///
    /// # Returns
    /// defense (usize): The defense of the Model
    pub fn get_defense(&self) -> usize {
        self.defense
    }

    /// Get the resilience of the Model
    ///
    /// # Returns
    /// resilience (usize): The resilience of the Model
    pub fn get_resilience(&self) -> usize {
        self.resilience
    }

    /// Get the armour of the Model
    ///
    /// # Returns
    /// armour (usize): The armour of the Model
    pub fn get_armour(&self) -> usize {
        self.armour
    }

    /// Get the aegis of the Model
    ///
    /// # Returns
    /// aegis (usize): The aegis of the Model
    pub fn get_aegis(&self) -> usize {
        self.aegis
    }

    /// Get the attack of the Model
    ///
    /// # Returns
    /// attack (usize): The attack of the Model
    pub fn get_attack(&self) -> usize {
        self.attack
    }

    /// Get the offensive of the Model
    ///
    /// # Returns
    /// offensive (usize): The offensive of the Model
    pub fn get_offensive(&self) -> usize {
        self.offensive
    }

    /// Get the strength of the Model
    ///
    /// # Returns
    /// strength (usize): The strength of the Model
    pub fn get_strength(&self) -> usize {
        self.strength
    }

    /// Get the armour penetration of the Model
    ///
    /// # Returns
    /// armour_penetration (usize): The armour penetration of the Model
    pub fn get_armour_penetration(&self) -> usize {
        self.armour_penetration
    }

    /// Get the agility of the Model
    ///
    /// # Returns
    /// agility (usize): The agility of the Model
    pub fn get_agility(&self) -> usize {
        self.agility
    }
}

/// Struct containing all the information about a single Modifier for a Model
///
/// # Attributes
/// stat (Stats): Stat modified by the Modifier
///
/// nb_dice (usize): The number of dice
///
/// requirements (Vec<String>): The requirements for the Modifier to apply (as an array of flags)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Modifier {
    stat: Stats,
    bonus: bool,
    nb_dice: usize,
    requirements: Vec<String>,
}

impl Modifier {
    /// Create a new Modifier using Stats, number of dice and the requirements to apply the Modifier
    ///
    /// # Return
    /// Modifier: The Modifier created
    pub fn new(stat: Stats, bonus: bool, nb_dice: usize, requirements: Vec<String>) -> Modifier {
        Modifier {
            stat,
            bonus,
            nb_dice,
            requirements,
        }
    }
}

/// Struct containing all the information about a Model
///
/// # Attributes
/// stats (Stats): The statistics of the Model
///
/// modifiers (Vec<Modifier>): The list of Modifier the Model have
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Model {
    stats: Stats,
    modifiers: Vec<Modifier>,
}

impl Model {
    /// # Create a new Model using Stats and list of Modifiers
    ///
    /// # Return
    /// Model: The Model created
    pub fn new(stats: Stats, modifiers: Vec<Modifier>) -> Model {
        Model { stats, modifiers }
    }
    /// Get the Stats of the Model
    ///
    /// # Returns
    /// Stats: The Stats of the Model
    pub fn get_stats(&self) -> &Stats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::{Stats};
    use crate::{global_test};

    #[test]
    fn stat_get_advance() {
        let stats: Stats = global_test::initialize_stats_with_1();
        assert_eq!(stats.get_advance(), 1);
    }

    #[test]
    fn stat_get_march() {
        let stats: Stats = global_test::initialize_stats_with_1();
        assert_eq!(stats.get_march(), 1);
    }

    #[test]
    fn stat_get_discipline() {
        let stats: Stats = global_test::initialize_stats_with_1();
        assert_eq!(stats.get_discipline(), 1);
    }

    #[test]
    fn stat_get_health_point() {
        let stats: Stats = global_test::initialize_stats_with_1();
        assert_eq!(stats.get_health_point(), 1);
    }

    #[test]
    fn stat_get_defense() {
        let stats: Stats = global_test::initialize_stats_with_1();
        assert_eq!(stats.get_defense(), 1);
    }

    #[test]
    fn stat_get_resilience() {
        let stats: Stats = global_test::initialize_stats_with_1();
        assert_eq!(stats.get_resilience(), 1);
    }

    #[test]
    fn stat_get_armour() {
        let stats: Stats = global_test::initialize_stats_with_1();
        assert_eq!(stats.get_armour(), 1);
    }

    #[test]
    fn stat_get_aegis() {
        let stats: Stats = global_test::initialize_stats_with_1();
        assert_eq!(stats.get_aegis(), 1);
    }

    #[test]
    fn stat_get_attack() {
        let stats: Stats = global_test::initialize_stats_with_1();
        assert_eq!(stats.get_attack(), 1);
    }

    #[test]
    fn stat_get_offensive() {
        let stats: Stats = global_test::initialize_stats_with_1();
        assert_eq!(stats.get_offensive(), 1);
    }

    #[test]
    fn stat_get_strength() {
        let stats: Stats = global_test::initialize_stats_with_1();
        assert_eq!(stats.get_strength(), 1);
    }

    #[test]
    fn stat_get_armour_penetration() {
        let stats: Stats = global_test::initialize_stats_with_1();
        assert_eq!(stats.get_armour_penetration(), 1);
    }

    #[test]
    fn stat_get_agility() {
        let stats: Stats = global_test::initialize_stats_with_1();
        assert_eq!(stats.get_agility(), 1);
    }
}
