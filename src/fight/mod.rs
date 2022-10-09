//! # Fight Module
//!
//! This module contain all the functions to emulate a fight between 2 Regiment
//! This may evolve to multiple Regiment against multiple Regiment

mod computation_tools;
mod global_values;
use std::{cmp::Ordering, collections::HashMap};

use crate::{
    math_tools, model,
    prediction::{self, Prediction},
    regiment::{self, Regiment},
};

/// ## This describe the scenario we are computing when creating a prediction
///
/// BEST : Represent the best scenario for the first unit
///
/// WORST : Represent the worst scenario for the first unit
///
/// MEAN : Represent the scenario with the highest probability of occurence
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ComputeCase {
    BEST,
    WORST,
    MEAN,
}

/// ## Compute the average damage a unit would dealt to another
///
/// ### Paramaters
/// (&regiment::Regiment) attacking_regiment -> The attacker
///
/// (&regiment::Regiment) defending_regiment -> The defender
///
/// ### Return
/// (usize, f64) -> A tuple with first the damage computed and then the probability that it occurs
fn compute_mean_case(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
) -> (usize, f64) {
    let attacking_model: &model::Model = attacking_regiment.get_model();
    let defending_model: &model::Model = defending_regiment.get_model();
    let damage_probability: f64 =
        computation_tools::compute_damage_probability(attacking_model, defending_model);
    let nb_attacks = (attacking_model.get_stats().get_attack() as f64
        * 1.5
        * attacking_regiment.get_cols() as f64)
        .round();
    let damage = std::cmp::min(
        (nb_attacks * damage_probability).round() as usize,
        defending_model.get_stats().get_health_point() * defending_regiment.get_nb_models(),
    );

    (
        damage,
        math_tools::compute_bernoulli(nb_attacks as usize, damage, damage_probability),
    )
}

/// ## Compute the average damage dealt by a unit to another according to the requested scenario
///
/// ### Parameters
/// (&regiment::Regiment) attacking_regiment -> The attacker
///
/// (&regiment::Regiment) defending_regiment -> The defender
///
/// (ComputeCase) case -> The scenario from first_unit point of view
///
/// ### Return
/// (usize, f64) -> The average amount of damage dealt by first_unit and the probability for this scenario to occurs
fn compute_case(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
    case: ComputeCase,
) -> (usize, f64) {
    let attacking_model: &model::Model = attacking_regiment.get_model();
    let defending_model: &model::Model = defending_regiment.get_model();
    let nb_touch: usize = (attacking_model.get_stats().get_attack() as f64
        * 1.5
        * attacking_regiment.get_cols() as f64)
        .round() as usize;
    let touch_probability: f64 =
        computation_tools::compute_damage_probability(attacking_model, defending_model);
    let mut results: Vec<(usize, f64)> = Vec::new();
    let mut selection: Vec<(usize, f64)> = Vec::new();
    let res: (usize, f64);
    let mut bondaries: (isize, isize);
    let threshold: f64 = match case {
        ComputeCase::BEST => global_values::BEST_CASE_THRESHOLD,
        ComputeCase::WORST => global_values::WORST_CASE_THRESHOLD,
        _ => 0.0_f64,
    };
    let defender_hp = defending_model.get_stats().get_health_point();

    if let ComputeCase::MEAN = case {
        return compute_mean_case(attacking_regiment, defending_regiment);
    }
    for i in 0..nb_touch {
        bondaries = match case {
            ComputeCase::BEST => (i as isize, nb_touch as isize),
            ComputeCase::WORST => (0, i as isize),
            ComputeCase::MEAN => unreachable!("Code not supposed to be reached!"),
        };
        results.push((
            i,
            math_tools::sigma(
                bondaries.0,
                bondaries.1,
                |curr, param, _, _| {
                    math_tools::compute_bernoulli(
                        param.unwrap().1 as usize,
                        curr as usize,
                        param.unwrap().0,
                    )
                },
                Some((touch_probability, nb_touch)),
            ),
        ));
    }

    for i in results.iter() {
        if i.1 >= threshold {
            selection.push(i.clone());
        }
    }

    if selection.is_empty() {
        let size = (0.1 * results.len() as f64).floor() as usize;
        for i in results {
            if selection.len() < size {
                selection.push(i.clone());
            } else {
                match selection.iter().position(|e| e.1 < i.1) {
                    Some(x) => selection[x] = i,
                    None => (),
                }
            }
        }
    }

    res = match case {
        ComputeCase::BEST => selection.into_iter().max_by_key(|e| e.0).unwrap(),
        ComputeCase::WORST => selection.into_iter().min_by_key(|e| e.0).unwrap(),
        ComputeCase::MEAN => unreachable!("Code not supposed to be reached!"),
    };
    bondaries = match case {
        ComputeCase::BEST => (defender_hp as isize, nb_touch as isize),
        ComputeCase::WORST => (0, defender_hp as isize),
        ComputeCase::MEAN => unreachable!("Code not supposed to be reached!"),
    };
    match res.0.cmp(&defender_hp) {
        Ordering::Less => res,
        Ordering::Greater => (
            defender_hp,
            math_tools::sigma(
                bondaries.0,
                bondaries.1,
                |curr: isize, param: Option<f64>, _, fin: isize| {
                    math_tools::compute_bernoulli(fin as usize, curr as usize, param.unwrap())
                },
                Some(touch_probability),
            ),
        ),
        Ordering::Equal => res,
    }
}

/// ## Compute a full scenario
///
/// ### Parameters
/// (&regiment::Regiment) attacking_regiment -> The first unit
///
/// (&regiment::Regiment) defending_regiment -> The second unit
///
/// (ComputeCase) case -> The scenario from first unit point of view
///
/// ### Return
/// (usize, usize, f64) -> The average amount of damage dealt by the two units and the probability for this scenario to occurs
fn compute_turn(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
    case: ComputeCase,
) -> (usize, usize, f64) {
    let complementary: ComputeCase = match case {
        ComputeCase::BEST => ComputeCase::WORST,
        ComputeCase::WORST => ComputeCase::BEST,
        ComputeCase::MEAN => ComputeCase::MEAN,
    };
    let attacking_stats: &model::Stats = attacking_regiment.get_model().get_stats();
    let defending_stats: &model::Stats = defending_regiment.get_model().get_stats();
    let fastest: u8 = computation_tools::find_the_fastest(attacking_stats, defending_stats);
    let first_damages: (usize, f64) = compute_case(attacking_regiment, defending_regiment, case);
    let second_damages: (usize, f64) =
        compute_case(defending_regiment, attacking_regiment, complementary);

    match fastest {
        0 => (
            first_damages.0,
            second_damages.0,
            first_damages.1 * second_damages.1,
        ),
        1 => {
            if first_damages.0 > defending_stats.get_health_point() {
                (first_damages.0, 0, first_damages.1)
            } else {
                (
                    first_damages.0,
                    second_damages.0,
                    first_damages.1 * second_damages.1,
                )
            }
        }
        2 => {
            if second_damages.0 > attacking_stats.get_health_point() {
                (0, second_damages.0, second_damages.1)
            } else {
                (
                    first_damages.0,
                    second_damages.0,
                    first_damages.1 * second_damages.1,
                )
            }
        }
        _ => unreachable!("Code not supposed to be reached!"),
    }
}

/// ## Create a prediction from raw turn computation
///
/// ### Parameters
/// (&regiment::Regiment) attacking_regiment -> The attacking regiment
///
/// (&regiment::Regiment) defending_regiment -> The defending regiment
///
/// (usize, usize, f64) raw_data -> The raw scenario computation
///
/// ### Return
/// Prediction -> The scenario represented with the Prediction data structure
fn turn_result_to_prediction(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
    raw_data: (usize, usize, f64),
) -> prediction::Prediction {
    let mut res_data: (Regiment, Regiment) =
        (attacking_regiment.clone(), defending_regiment.clone());
    res_data.0.take_damage(raw_data.1);
    res_data.1.take_damage(raw_data.0);
    res_data.0.earn_points(raw_data.0);
    res_data.1.earn_points(raw_data.1);
    Prediction::new(res_data.0, res_data.1, raw_data.2)
}

/// ## Compute the 3 most important scenario while in melee phase
///
/// ### Parameters
/// (&regiment::Regiment) first_unit -> The first unit
///
/// (&regiment::Regiment) second_unit -> The second unit
///
/// ### Return
/// (HashMap<ComputeCase, Prediction>) -> \[The more realistic scenario, The best scenario for first unit, The worst scenario for first unit\]
pub fn create_prediction(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
) -> HashMap<ComputeCase, Prediction> {
    HashMap::from([
        (
            ComputeCase::BEST,
            turn_result_to_prediction(
                attacking_regiment,
                defending_regiment,
                compute_turn(attacking_regiment, defending_regiment, ComputeCase::BEST),
            ),
        ),
        (
            ComputeCase::WORST,
            turn_result_to_prediction(
                attacking_regiment,
                defending_regiment,
                compute_turn(attacking_regiment, defending_regiment, ComputeCase::WORST),
            ),
        ),
        (
            ComputeCase::MEAN,
            turn_result_to_prediction(
                attacking_regiment,
                defending_regiment,
                compute_turn(attacking_regiment, defending_regiment, ComputeCase::MEAN),
            ),
        ),
    ])
}

#[cfg(test)]
mod tests {
    use crate::{model, prediction, regiment};

    use super::{
        compute_case, compute_mean_case, compute_turn, create_prediction, turn_result_to_prediction,
    };

    fn initialize_chaos_warrior() -> regiment::Regiment {
        let chaos_warrior_stats: model::Stats = model::Stats::new(
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
        let chaos_warrior_modifier_stats: model::Stats = model::Stats::new(
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
        let chaos_warrior_modifier: model::Modifier =
            model::Modifier::new(chaos_warrior_modifier_stats, false, 0, vec![]);
        let model_chaos_warrior: model::Model =
            model::Model::new(chaos_warrior_stats, vec![chaos_warrior_modifier]);
        let chaos_warrior: regiment::Regiment =
            regiment::Regiment::new(model_chaos_warrior, 4, 5, 20, None);
        chaos_warrior
    }

    fn initialize_heavy_infantry() -> regiment::Regiment {
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

    fn initialize_two_units() -> (regiment::Regiment, regiment::Regiment) {
        (initialize_chaos_warrior(), initialize_heavy_infantry())
    }

    #[test]
    fn test_turn_result_to_prediction() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let prediction: prediction::Prediction =
            turn_result_to_prediction(&chaos_warrior, &heavy_infantry, (5, 10, 0.6_f64));
        assert_eq!(prediction.get_probability(), 0.6_f64);
        assert_eq!(prediction.get_attacking_regiment().get_points(), 5);
        assert_eq!(
            prediction
                .get_attacking_regiment()
                .get_regiment_health_points(),
            10
        );
        assert_eq!(
            prediction.get_attacking_regiment().get_model(),
            chaos_warrior.get_model()
        );
        assert_eq!(prediction.get_attacking_regiment().get_nb_models(), 10);

        assert_eq!(prediction.get_defending_regiment().get_points(), 10);
        assert_eq!(
            prediction
                .get_defending_regiment()
                .get_regiment_health_points(),
            15
        );
        assert_eq!(
            prediction.get_defending_regiment().get_model(),
            heavy_infantry.get_model()
        );
        assert_eq!(prediction.get_defending_regiment().get_nb_models(), 15);
    }

    /*#[test]
    fn test_compute_mean_case() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let res = compute_mean_case(&chaos_warrior, &heavy_infantry);
    }*/
}
