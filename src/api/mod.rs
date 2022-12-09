use crate::fight::compute_turn;

use self::dto::{prediction_dto::PredictionDto, regiment_dto::RegimentDto};

pub mod dto;

#[repr(C)]
pub struct Result {
    best_prediction: PredictionDto,
    worst_prediction: PredictionDto,
    mean_prediction: PredictionDto,
}

#[no_mangle]
pub extern "C" fn compute_fight(
    attacking_regiment: RegimentDto,
    defending_regiment: RegimentDto,
) -> Result {
    let attacking_regiment = RegimentDto::hydrate(&attacking_regiment);
    let defending_regiment = RegimentDto::hydrate(&defending_regiment);
    let result = compute_turn(&attacking_regiment, &defending_regiment);

    println!(
        "{:?}",
        result
            .get(&crate::fight::ComputeCase::BEST)
            .unwrap()
            .get_attacking_regiment()
            .get_points()
    );
    println!("@@@@@@@@@@@@");
    println!(
        "{:?}",
        result
            .get(&crate::fight::ComputeCase::WORST)
            .unwrap()
            .get_attacking_regiment()
            .get_points()
    );
    println!("@@@@@@@@@@@@");
    println!(
        "{:?}",
        result
            .get(&crate::fight::ComputeCase::MEAN)
            .unwrap()
            .get_attacking_regiment()
            .get_points()
    );
    Result {
        /*best_prediction: PredictionDto {
            attacking_regiment: RegimentDto {
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
            },
            defending_regiment: RegimentDto {
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
            },
            occurence_probability: 0.0,
        },
        worst_prediction: PredictionDto {
            attacking_regiment: RegimentDto {
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
            },
            defending_regiment: RegimentDto {
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
            },
            occurence_probability: 0.0,
        },
        mean_prediction: PredictionDto {
            attacking_regiment: RegimentDto {
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
            },
            defending_regiment: RegimentDto {
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
            },
            occurence_probability: 0.0,
        },*/
        best_prediction: PredictionDto::dehydrate(
            result.get(&crate::fight::ComputeCase::BEST).unwrap(),
        ),
        worst_prediction: PredictionDto::dehydrate(
            result.get(&crate::fight::ComputeCase::WORST).unwrap(),
        ),
        mean_prediction: PredictionDto::dehydrate(
            result.get(&crate::fight::ComputeCase::MEAN).unwrap(),
        ),
    }
}
