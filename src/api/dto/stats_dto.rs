use crate::model;

#[repr(C)]
#[derive(Clone)]
pub struct StatsDto {
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

impl StatsDto {
    pub fn dehydrate(stats: &model::Stats) -> StatsDto {
        StatsDto {
            advance: stats.get_advance(),
            march: stats.get_march(),
            discipline: stats.get_discipline(),
            health_point: stats.get_health_point(),
            defense: stats.get_defense(),
            resilience: stats.get_resilience(),
            armour: stats.get_armour(),
            aegis: stats.get_aegis(),
            attack: stats.get_attack(),
            offensive: stats.get_offensive(),
            strength: stats.get_strength(),
            armour_penetration: stats.get_armour_penetration(),
            agility: stats.get_agility(),
        }
    }

    pub fn hydrate(stats_dto: &StatsDto) -> model::Stats {
        model::Stats::new(
            model::GlobalStats {
                advance: stats_dto.advance,
                march: stats_dto.march,
                discipline: stats_dto.discipline,
            },
            model::DefensiveStats {
                health_point: stats_dto.health_point,
                defense: stats_dto.defense,
                resilience: stats_dto.resilience,
                armour: stats_dto.armour,
                aegis: stats_dto.aegis,
            },
            model::OffensiveStats {
                attack: stats_dto.attack,
                offensive: stats_dto.offensive,
                strength: stats_dto.strength,
                armour_penetration: stats_dto.armour_penetration,
                agility: stats_dto.agility,
            },
        )
    }
}
