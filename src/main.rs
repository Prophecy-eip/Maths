use maths::api::dto::{
    model_dto::ModelDto, modifiers_dto::ModifierDto, raw_array::RawArray,
    regiment_dto::RegimentDto, stats_dto::StatsDto,
};

fn main() {
    let attacking_regiment = RegimentDto {
        model: ModelDto {
            stats: StatsDto {
                advance: 4,
                march: 8,
                discipline: 8,
                health_point: 1,
                defense: 5,
                resilience: 4,
                armour: 0,
                aegis: 0,
                attack: 2,
                offensive: 4,
                strength: 5,
                armour_penetration: 1,
                agility: 4,
            },
            modifiers: RawArray {
                size: 1,
                items: Box::leak(Box::new(ModifierDto {
                    stat: StatsDto {
                        advance: 0,
                        march: 0,
                        discipline: 0,
                        health_point: 0,
                        defense: 0,
                        resilience: 0,
                        armour: 0,
                        aegis: 0,
                        attack: 0,
                        offensive: 0,
                        strength: 0,
                        armour_penetration: 0,
                        agility: 0,
                    },
                    bonus: false,
                    nb_dice: 0,
                    requirements: RawArray {
                        size: 0,
                        items: Box::leak(Box::new(0)),
                    },
                })),
            },
        },
        nb_rows: 4,
        nb_cols: 5,
        nb_models: 20,
        regiment_health_point: 1,
        points: 1,
    };
    let defending_regiment = RegimentDto {
        model: ModelDto {
            stats: StatsDto {
                advance: 4,
                march: 8,
                discipline: 8,
                health_point: 1,
                defense: 5,
                resilience: 4,
                armour: 0,
                aegis: 0,
                attack: 2,
                offensive: 4,
                strength: 5,
                armour_penetration: 1,
                agility: 4,
            },
            modifiers: RawArray {
                size: 1,
                items: Box::leak(Box::new(ModifierDto {
                    stat: StatsDto {
                        advance: 0,
                        march: 0,
                        discipline: 0,
                        health_point: 0,
                        defense: 0,
                        resilience: 0,
                        armour: 0,
                        aegis: 0,
                        attack: 0,
                        offensive: 0,
                        strength: 0,
                        armour_penetration: 0,
                        agility: 0,
                    },
                    bonus: false,
                    nb_dice: 0,
                    requirements: RawArray {
                        size: 0,
                        items: Box::leak(Box::new(0)),
                    },
                })),
            },
        },
        nb_rows: 4,
        nb_cols: 5,
        nb_models: 20,
        regiment_health_point: 1,
        points: 1,
    };
    maths::api::compute_fight(attacking_regiment, defending_regiment);
}
