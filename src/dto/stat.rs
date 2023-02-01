
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
