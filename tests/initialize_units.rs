use maths::{model, regiment};

pub fn initialize_warriors() -> regiment::Regiment {
    let warriors_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 4,
            march: 8,
            discipline: 8,
        },
        model::DefensiveStats {
            health_point: 1,
            defense: 5,
            resilience: 4,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 2,
            strength: 5,
            offensive: 4,
            armour_penetration: 1,
            agility: 4,
        },
    );
    let warriors_modifier_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 0,
            march: 0,
            discipline: 0,
        },
        model::DefensiveStats {
            health_point: 0,
            defense: 0,
            resilience: 0,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 0,
            strength: 0,
            offensive: 0,
            armour_penetration: 0,
            agility: 0,
        },
    );
    let warriors_modifier: model::Modifier =
        model::Modifier::new(warriors_modifier_stats, false, 0, vec![]);
    let model_warriors: model::Model = model::Model::new(warriors_stats, vec![warriors_modifier]);
    let warriors: regiment::Regiment = regiment::Regiment::new(model_warriors, 4, 5, 20, None);
    warriors
}

pub fn initialize_heavy_infantry() -> regiment::Regiment {
    let heavy_infantry_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 4,
            march: 8,
            discipline: 7,
        },
        model::DefensiveStats {
            health_point: 1,
            defense: 3,
            resilience: 3,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 1,
            strength: 3,
            offensive: 3,
            armour_penetration: 0,
            agility: 3,
        },
    );
    let heavy_infantry_modifier_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 0,
            march: 0,
            discipline: 0,
        },
        model::DefensiveStats {
            health_point: 0,
            defense: 0,
            resilience: 0,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 0,
            strength: 0,
            offensive: 0,
            armour_penetration: 0,
            agility: 0,
        },
    );
    let heavy_infantry_modifier: model::Modifier =
        model::Modifier::new(heavy_infantry_modifier_stats, false, 0, vec![]);
    let model_heavy_infantry: model::Model =
        model::Model::new(heavy_infantry_stats, vec![heavy_infantry_modifier]);
    let heavy_infantry: regiment::Regiment =
        regiment::Regiment::new(model_heavy_infantry, 4, 5, 20, None);
    heavy_infantry
}

pub fn initialize_wildhorn_herd() -> regiment::Regiment {
    let wildhorn_herd_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 5,
            march: 10,
            discipline: 7,
        },
        model::DefensiveStats {
            health_point: 1,
            defense: 4,
            resilience: 4,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 1,
            offensive: 4,
            strength: 3,
            armour_penetration: 0,
            agility: 3,
        },
    );
    let wildhorn_herd_modifier_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 0,
            march: 0,
            discipline: 0,
        },
        model::DefensiveStats {
            health_point: 0,
            defense: 0,
            resilience: 0,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 0,
            strength: 0,
            offensive: 0,
            armour_penetration: 0,
            agility: 0,
        },
    );
    let wildhorn_herd_modifier: model::Modifier =
        model::Modifier::new(wildhorn_herd_modifier_stats, false, 0, vec![]);
    let model_wildhorn_herd: model::Model =
        model::Model::new(wildhorn_herd_stats, vec![wildhorn_herd_modifier]);
    let wildhorn_herd: regiment::Regiment =
        regiment::Regiment::new(model_wildhorn_herd, 4, 5, 20, None);
    return wildhorn_herd;
}

pub fn initialize_imps() -> regiment::Regiment {
    let imps_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 5,
            march: 10,
            discipline: 6,
        },
        model::DefensiveStats {
            health_point: 1,
            defense: 2,
            resilience: 3,
            armour: 0,
            aegis: 5,
        },
        model::OffensiveStats {
            attack: 1,
            offensive: 2,
            strength: 2,
            armour_penetration: 0,
            agility: 3,
        },
    );
    let imps_modifier_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 0,
            march: 0,
            discipline: 0,
        },
        model::DefensiveStats {
            health_point: 0,
            defense: 0,
            resilience: 0,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 0,
            offensive: 0,
            strength: 0,
            armour_penetration: 0,
            agility: 0,
        },
    );
    let imps_modifier: model::Modifier =
        model::Modifier::new(imps_modifier_stats, false, 0, vec![]);
    let model_imps: model::Model = model::Model::new(imps_stats, vec![imps_modifier]);
    let imps: regiment::Regiment = regiment::Regiment::new(model_imps, 4, 5, 20, None);
    return imps;
}

pub fn initialize_silexian_spears() -> regiment::Regiment {
    let silexian_spears_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 5,
            march: 10,
            discipline: 8,
        },
        model::DefensiveStats {
            health_point: 1,
            defense: 4,
            resilience: 3,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 1,
            offensive: 4,
            strength: 3,
            armour_penetration: 0,
            agility: 5,
        },
    );
    let silexian_spears_modifier_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 0,
            march: 0,
            discipline: 0,
        },
        model::DefensiveStats {
            health_point: 0,
            defense: 0,
            resilience: 0,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 0,
            offensive: 0,
            strength: 0,
            armour_penetration: 0,
            agility: 0,
        },
    );
    let silexian_spears_modifier: model::Modifier =
        model::Modifier::new(silexian_spears_modifier_stats, false, 0, vec![]);
    let model_silexian_spears: model::Model =
        model::Model::new(silexian_spears_stats, vec![silexian_spears_modifier]);
    let silexian_spears: regiment::Regiment =
        regiment::Regiment::new(model_silexian_spears, 4, 5, 20, None);
    return silexian_spears;
}

pub fn initialize_clan_warriors() -> regiment::Regiment {
    let clan_warriors_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 3,
            march: 9,
            discipline: 9,
        },
        model::DefensiveStats {
            health_point: 1,
            defense: 4,
            resilience: 4,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 1,
            offensive: 4,
            strength: 3,
            armour_penetration: 0,
            agility: 2,
        },
    );
    let clan_warriors_modifier_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 0,
            march: 0,
            discipline: 0,
        },
        model::DefensiveStats {
            health_point: 0,
            defense: 0,
            resilience: 0,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 0,
            offensive: 0,
            strength: 0,
            armour_penetration: 0,
            agility: 0,
        },
    );
    let clan_warriors_modifier: model::Modifier =
        model::Modifier::new(clan_warriors_modifier_stats, false, 0, vec![]);
    let model_clan_warriors: model::Model =
        model::Model::new(clan_warriors_stats, vec![clan_warriors_modifier]);
    let clan_warriors: regiment::Regiment =
        regiment::Regiment::new(model_clan_warriors, 4, 5, 20, None);
    return clan_warriors;
}

pub fn initialize_citizen_spears() -> regiment::Regiment {
    let citizen_spears_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 5,
            march: 10,
            discipline: 8,
        },
        model::DefensiveStats {
            health_point: 1,
            defense: 4,
            resilience: 3,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 1,
            offensive: 4,
            strength: 3,
            armour_penetration: 0,
            agility: 5,
        },
    );
    let citizen_spears_modifier_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 0,
            march: 0,
            discipline: 0,
        },
        model::DefensiveStats {
            health_point: 0,
            defense: 0,
            resilience: 0,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 0,
            offensive: 0,
            strength: 0,
            armour_penetration: 0,
            agility: 0,
        },
    );
    let citizen_spears_modifier: model::Modifier =
        model::Modifier::new(citizen_spears_modifier_stats, false, 0, vec![]);
    let model_citizen_spears: model::Model =
        model::Model::new(citizen_spears_stats, vec![citizen_spears_modifier]);
    let citizen_spears: regiment::Regiment =
        regiment::Regiment::new(model_citizen_spears, 4, 5, 20, None);
    return citizen_spears;
}

pub fn initialize_infernal_warriors() -> regiment::Regiment {
    let infernal_warriors_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 3,
            march: 9,
            discipline: 9,
        },
        model::DefensiveStats {
            health_point: 1,
            defense: 4,
            resilience: 4,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 1,
            offensive: 4,
            strength: 3,
            armour_penetration: 0,
            agility: 2,
        },
    );
    let infernal_warriors_modifier_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 0,
            march: 0,
            discipline: 0,
        },
        model::DefensiveStats {
            health_point: 0,
            defense: 0,
            resilience: 0,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 0,
            offensive: 0,
            strength: 0,
            armour_penetration: 0,
            agility: 0,
        },
    );
    let infernal_warriors_modifier: model::Modifier =
        model::Modifier::new(infernal_warriors_modifier_stats, false, 0, vec![]);
    let model_infernal_warriors: model::Model =
        model::Model::new(infernal_warriors_stats, vec![infernal_warriors_modifier]);
    let infernal_warriors: regiment::Regiment =
        regiment::Regiment::new(model_infernal_warriors, 4, 5, 20, None);
    return infernal_warriors;
}

pub fn initialize_zombies() -> regiment::Regiment {
    let zombies_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 4,
            march: 8,
            discipline: 2,
        },
        model::DefensiveStats {
            health_point: 1,
            defense: 1,
            resilience: 3,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 1,
            offensive: 1,
            strength: 3,
            armour_penetration: 0,
            agility: 1,
        },
    );
    let zombies_modifier_stats: model::Stats = model::Stats::new(
        model::GlobalStats {
            advance: 0,
            march: 0,
            discipline: 0,
        },
        model::DefensiveStats {
            health_point: 0,
            defense: 0,
            resilience: 0,
            armour: 0,
            aegis: 0,
        },
        model::OffensiveStats {
            attack: 0,
            offensive: 0,
            strength: 0,
            armour_penetration: 0,
            agility: 0,
        },
    );
    let zombies_modifier: model::Modifier =
        model::Modifier::new(zombies_modifier_stats, false, 0, vec![]);
    let model_zombies: model::Model = model::Model::new(zombies_stats, vec![zombies_modifier]);
    let zombies: regiment::Regiment = regiment::Regiment::new(model_zombies, 10, 8, 80, None);
    return zombies;
}
