
pub struct GlobalStatsDto {
    advance: usize,
    march: usize,
    discipline: usize,
}

impl GlobalStatsDto {
    pub fn new(advance: usize, march: usize, discipline: usize) -> GlobalStatsDto {
        GlobalStatsDto {
            advance,
            march,
            discipline,
        }
    }
}

pub struct DefensiveStatsDto {
    health_points: usize,
    defense: usize,
    resilience: usize,
    armour: usize,
    aegis: usize,
}

impl DefensiveStatsDto {
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

pub struct OffensiveStatsDto {
    attack: usize,
    offensive: usize,
    strength: usize,
    armour_penetration: usize,
    agility: usize,
}

impl OffensiveStatsDto {
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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
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
    pub fn new(
        GlobalStatsDto { advance, march, discipline }: GlobalStatsDto,
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