//! StatsDto module
//!
//! This module contains the StatsDto struct and its implementation.
//! The goal is to be able to communicate easily with outside of the library.

/// Struct used to represent the GlobalStats of a model outside of rust code
///
/// # Attributes
///
/// advance (usize): The advance of the model
///
/// march (usize): The march of the model
///
/// discipline (usize): The discipline of the model
#[derive(Clone, Copy, Debug)]
pub struct GlobalStatsDto {
    advance: usize,
    march: usize,
    discipline: usize,
}

impl GlobalStatsDto {
    /// Creates a new GlobalStatsDto
    ///
    /// # Arguments
    ///
    /// advance (usize): The advance of the model
    ///
    /// march (usize): The march of the model
    ///
    /// discipline (usize): The discipline of the model
    pub fn new(advance: usize, march: usize, discipline: usize) -> GlobalStatsDto {
        GlobalStatsDto {
            advance,
            march,
            discipline,
        }
    }
}

/// Struct used to represent the DefensiveStats of a model outside of rust code
///
/// # Attributes
///
/// health_points (usize): The health points of the model
///
/// defense (usize): The defense of the model
///
/// resilience (usize): The resilience of the model
///
/// armour (usize): The armour of the model
///
/// aegis (usize): The aegis of the model
#[derive(Clone, Copy, Debug)]
pub struct DefensiveStatsDto {
    health_points: usize,
    defense: usize,
    resilience: usize,
    armour: usize,
    aegis: usize,
}

impl DefensiveStatsDto {
    /// Creates a new DefensiveStatsDto
    ///
    /// # Arguments
    ///
    /// health_points (usize): The health points of the model
    ///
    /// defense (usize): The defense of the model
    ///
    /// resilience (usize): The resilience of the model
    ///
    /// armour (usize): The armour of the model
    ///
    /// aegis (usize): The aegis of the model
    ///
    /// # Return
    ///
    /// DefensiveStatsDto: The newly created DefensiveStatsDto
    pub fn new(
        health_points: usize,
        defense: usize,
        resilience: usize,
        armour: usize,
        aegis: usize,
    ) -> DefensiveStatsDto {
        DefensiveStatsDto {
            health_points,
            defense,
            resilience,
            armour,
            aegis,
        }
    }
}

/// Struct used to represent the OffensiveStats of a model outside of rust code
///
/// # Attributes
///
/// attack (usize): The attack of the model
///
/// offensive (usize): The offensive of the model
///
/// strength (usize): The strength of the model
///
/// armour_penetration (usize): The armour penetration of the model
///
/// agility (usize): The agility of the model
#[derive(Clone, Copy, Debug)]
pub struct OffensiveStatsDto {
    attack: usize,
    offensive: usize,
    strength: usize,
    armour_penetration: usize,
    agility: usize,
}

impl OffensiveStatsDto {
    /// Creates a new OffensiveStatsDto
    ///
    /// # Arguments
    ///
    /// attack (usize): The attack of the model
    ///
    /// offensive (usize): The offensive of the model
    ///
    /// strength (usize): The strength of the model
    ///
    /// armour_penetration (usize): The armour penetration of the model
    ///
    /// agility (usize): The agility of the model
    ///
    /// # Return
    ///
    /// OffensiveStatsDto: The newly created OffensiveStatsDto
    pub fn new(
        attack: usize,
        offensive: usize,
        strength: usize,
        armour_penetration: usize,
        agility: usize,
    ) -> OffensiveStatsDto {
        OffensiveStatsDto {
            attack,
            offensive,
            strength,
            armour_penetration,
            agility,
        }
    }
}

/// Struct used to represent the Stats of a model outside of rust code
///
/// # Attributes
///
/// advance (usize): The advance of the model
///
/// march (usize): The march of the model
///
/// discipline (usize): The discipline of the model
///
/// health_points (usize): The health points of the model
///
/// defense (usize): The defense of the model
///
/// resilience (usize): The resilience of the model
///
/// armour (usize): The armour of the model
///
/// aegis (usize): The aegis of the model
///
/// attack (usize): The attack of the model
///
/// offensive (usize): The offensive of the model
///
/// strength (usize): The strength of the model
///
/// armour_penetration (usize): The armour penetration of the model
///
/// agility (usize): The agility of the model
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
pub struct StatsDto {
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

impl StatsDto {
    /// Creates a new StatsDto
    ///
    /// # Arguments
    ///
    /// GlobalStatsDto { advance, march, discipline }: The global stats of the model
    ///
    /// DefensiveStatsDto { health_points, defense, resilience, armour, aegis }: The defensive stats of the model
    ///
    /// OffensiveStatsDto { attack, offensive, strength, armour_penetration, agility }: The offensive stats of the model
    ///
    /// # Return
    ///
    /// StatsDto: The newly created StatsDto
    pub fn new(
        GlobalStatsDto {
            advance,
            march,
            discipline,
        }: GlobalStatsDto,
        DefensiveStatsDto {
            health_points,
            defense,
            resilience,
            armour,
            aegis,
        }: DefensiveStatsDto,
        OffensiveStatsDto {
            attack,
            offensive,
            strength,
            armour_penetration,
            agility,
        }: OffensiveStatsDto,
    ) -> StatsDto {
        StatsDto {
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

    /// Hydrates a StatsDto into a Stats
    ///
    /// # Return
    ///
    /// Stats: The hydrated Stats
    pub fn hydrate(&self) -> crate::stat::Stats {
        crate::stat::Stats::new(
            crate::stat::GlobalStats {
                advance: self.advance,
                march: self.march,
                discipline: self.discipline,
            },
            crate::stat::DefensiveStats {
                health_points: self.health_points,
                defense: self.defense,
                resilience: self.resilience,
                armour: self.armour,
                aegis: self.aegis,
            },
            crate::stat::OffensiveStats {
                attack: self.attack,
                offensive: self.offensive,
                strength: self.strength,
                armour_penetration: self.armour_penetration,
                agility: self.agility,
            },
        )
    }

    /// Dehydrates a Stats into a StatsDto
    ///
    /// # Arguments
    ///
    /// stat (Stats): The Stats to dehydrate
    ///
    /// # Return
    ///
    /// StatsDto: The dehydrated StatsDto
    pub fn dehydrate(stat: &crate::stat::Stats) -> StatsDto {
        StatsDto {
            advance: stat.get_advance(),
            march: stat.get_march(),
            discipline: stat.get_discipline(),
            health_points: stat.get_health_points(),
            defense: stat.get_defense(),
            resilience: stat.get_resilience(),
            armour: stat.get_armour(),
            aegis: stat.get_aegis(),
            attack: stat.get_attack(),
            offensive: stat.get_offensive(),
            strength: stat.get_strength(),
            armour_penetration: stat.get_armour_penetration(),
            agility: stat.get_agility(),
        }
    }

    /// Gets the advance of the model
    ///
    /// # Return
    ///
    /// usize: The advance of the model
    pub fn get_advance(&self) -> usize {
        self.advance
    }

    /// Gets the march of the model
    ///
    /// # Return
    ///
    /// usize: The march of the model
    pub fn get_march(&self) -> usize {
        self.march
    }

    /// Gets the discipline of the model
    ///
    /// # Return
    ///
    /// usize: The discipline of the model
    pub fn get_discipline(&self) -> usize {
        self.discipline
    }

    /// Gets the health points of the model
    ///
    /// # Return
    ///
    /// usize: The health points of the model
    pub fn get_health_points(&self) -> usize {
        self.health_points
    }

    /// Gets the defense of the model
    ///
    /// # Return
    ///
    /// usize: The defense of the model
    pub fn get_defense(&self) -> usize {
        self.defense
    }

    /// Gets the resilience of the model
    ///
    /// # Return
    ///
    /// usize: The resilience of the model
    pub fn get_resilience(&self) -> usize {
        self.resilience
    }

    /// Gets the armour of the model
    ///
    /// # Return
    ///
    /// usize: The armour of the model
    pub fn get_armour(&self) -> usize {
        self.armour
    }

    /// Gets the aegis of the model
    ///
    /// # Return
    ///
    /// usize: The aegis of the model
    pub fn get_aegis(&self) -> usize {
        self.aegis
    }

    /// Gets the attack of the model
    ///
    /// # Return
    ///
    /// usize: The attack of the model
    pub fn get_attack(&self) -> usize {
        self.attack
    }

    /// Gets the offensive of the model
    ///
    /// # Return
    ///
    /// usize: The offensive of the model
    pub fn get_offensive(&self) -> usize {
        self.offensive
    }

    /// Gets the strength of the model
    ///
    /// # Return
    ///
    /// usize: The strength of the model
    pub fn get_strength(&self) -> usize {
        self.strength
    }

    /// Gets the armour penetration of the model
    ///
    /// # Return
    ///
    /// usize: The armour penetration of the model
    pub fn get_armour_penetration(&self) -> usize {
        self.armour_penetration
    }

    /// Gets the agility of the model
    ///
    /// # Return
    ///
    /// usize: The agility of the model
    pub fn get_agility(&self) -> usize {
        self.agility
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_dto() {
        let stats = StatsDto::new(
            GlobalStatsDto::new(1, 2, 3),
            DefensiveStatsDto::new(4, 5, 6, 7, 8),
            OffensiveStatsDto::new(9, 10, 11, 12, 13),
        );

        assert_eq!(stats.advance, 1);
        assert_eq!(stats.march, 2);
        assert_eq!(stats.discipline, 3);
        assert_eq!(stats.health_points, 4);
        assert_eq!(stats.defense, 5);
        assert_eq!(stats.resilience, 6);
        assert_eq!(stats.armour, 7);
        assert_eq!(stats.aegis, 8);
        assert_eq!(stats.attack, 9);
        assert_eq!(stats.offensive, 10);
        assert_eq!(stats.strength, 11);
        assert_eq!(stats.armour_penetration, 12);
        assert_eq!(stats.agility, 13);
    }

    #[test]
    fn test_stats_dto_hydrate() {
        let stats = StatsDto::new(
            GlobalStatsDto::new(1, 2, 3),
            DefensiveStatsDto::new(4, 5, 6, 7, 8),
            OffensiveStatsDto::new(9, 10, 11, 12, 13),
        )
        .hydrate();

        assert_eq!(stats.get_advance(), 1);
        assert_eq!(stats.get_march(), 2);
        assert_eq!(stats.get_discipline(), 3);
        assert_eq!(stats.get_health_points(), 4);
        assert_eq!(stats.get_defense(), 5);
        assert_eq!(stats.get_resilience(), 6);
        assert_eq!(stats.get_armour(), 7);
        assert_eq!(stats.get_aegis(), 8);
        assert_eq!(stats.get_attack(), 9);
        assert_eq!(stats.get_offensive(), 10);
        assert_eq!(stats.get_strength(), 11);
        assert_eq!(stats.get_armour_penetration(), 12);
        assert_eq!(stats.get_agility(), 13);
    }

    #[test]
    fn test_stats_dto_dehydrate() {
        let stats = StatsDto::dehydrate(&crate::stat::Stats::new(
            crate::stat::GlobalStats {
                advance: 1,
                march: 2,
                discipline: 3,
            },
            crate::stat::DefensiveStats {
                health_points: 4,
                defense: 5,
                resilience: 6,
                armour: 7,
                aegis: 8,
            },
            crate::stat::OffensiveStats {
                attack: 9,
                offensive: 10,
                strength: 11,
                armour_penetration: 12,
                agility: 13,
            },
        ));

        assert_eq!(stats.advance, 1);
        assert_eq!(stats.march, 2);
        assert_eq!(stats.discipline, 3);
        assert_eq!(stats.health_points, 4);
        assert_eq!(stats.defense, 5);
        assert_eq!(stats.resilience, 6);
        assert_eq!(stats.armour, 7);
        assert_eq!(stats.aegis, 8);
        assert_eq!(stats.attack, 9);
        assert_eq!(stats.offensive, 10);
        assert_eq!(stats.strength, 11);
        assert_eq!(stats.armour_penetration, 12);
        assert_eq!(stats.agility, 13);
    }
}
