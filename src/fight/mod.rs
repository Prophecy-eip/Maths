//! # Fight Module
//!
//! This module contain all the functions to emulate a fight between 2 Regiment
//! This may evolve to multiple Regiment against multiple Regiment

mod computation_tools;
mod global_values;

use crate::{
    math_tools, model,
    prediction::{self, Prediction},
    regiment,
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
    case: &ComputeCase,
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
            selection.push(*i);
        }
    }

    if selection.is_empty() {
        let size = (0.1 * results.len() as f64).floor() as usize;
        for i in results {
            if selection.len() < size {
                selection.push(i);
            } else {
                match selection.iter().position(|e| e.1 < i.1) {
                    Some(x) => selection[x] = i,
                    None => (),
                }
            }
        }
    }

    let res: (usize, f64) = match case {
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
        std::cmp::Ordering::Less => res,
        std::cmp::Ordering::Greater => (
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
        std::cmp::Ordering::Equal => res,
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
/// Prediction -> The prediction computed according to the specified Compute Case
fn create_prediction(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
    case: ComputeCase,
) -> prediction::Prediction {
    let complementary: ComputeCase = match case {
        ComputeCase::BEST => ComputeCase::WORST,
        ComputeCase::WORST => ComputeCase::BEST,
        ComputeCase::MEAN => ComputeCase::MEAN,
    };
    let fastest: u8 = computation_tools::find_the_fastest(
        attacking_regiment.get_model().get_stats(),
        defending_regiment.get_model().get_stats(),
    );
    let mut first_damages: (usize, f64) =
        compute_case(attacking_regiment, defending_regiment, &case);
    let mut second_damages: (usize, f64) =
        compute_case(defending_regiment, attacking_regiment, &complementary);
    let mut final_defending = defending_regiment.clone();
    let mut final_attacking = attacking_regiment.clone();
    match fastest {
        0 => {
            final_attacking.take_damage(second_damages.0);
            final_attacking.earn_points(first_damages.0);
            final_defending.take_damage(first_damages.0);
            final_defending.earn_points(second_damages.0);
            prediction::Prediction::new(
                final_attacking,
                final_defending,
                first_damages.1 * second_damages.1,
            )
        }
        1 => {
            final_defending.take_damage(first_damages.0);
            final_attacking.earn_points(first_damages.0);
            if first_damages.0 < defending_regiment.get_regiment_health_points() {
                second_damages = compute_case(&final_defending, &final_attacking, &complementary);
                final_attacking.take_damage(second_damages.0);
                final_defending.earn_points(second_damages.0);
                prediction::Prediction::new(
                    final_attacking,
                    final_defending,
                    first_damages.1 * second_damages.1,
                )
            } else {
                prediction::Prediction::new(final_attacking, final_defending, first_damages.1)
            }
        }
        2 => {
            final_attacking.take_damage(second_damages.0);
            final_defending.earn_points(second_damages.0);
            if second_damages.0 < attacking_regiment.get_regiment_health_points() {
                first_damages = compute_case(&final_attacking, &final_defending, &case);
                final_defending.take_damage(first_damages.0);
                final_attacking.earn_points(first_damages.0);
                prediction::Prediction::new(
                    final_attacking,
                    final_defending,
                    first_damages.1 * second_damages.1,
                )
            } else {
                prediction::Prediction::new(final_attacking, final_defending, first_damages.1)
            }
        }
        _ => unreachable!("Code not supposed to be reached!"),
    }
}

/// ## Compute the 3 most important scenario while in melee phase
///
/// ### Parameters
/// (&regiment::Regiment) first_unit -> The first unit
///
/// (&regiment::Regiment) second_unit -> The second unit
///
/// ### Return
/// (HashMap<ComputeCase, Prediction>) -> The more realistic scenario, The best scenario for first unit, The worst scenario for first unit
pub fn compute_turn(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
) -> std::collections::HashMap<ComputeCase, Prediction> {
    std::collections::HashMap::from([
        (
            ComputeCase::BEST,
            create_prediction(attacking_regiment, defending_regiment, ComputeCase::BEST),
        ),
        (
            ComputeCase::WORST,
            create_prediction(attacking_regiment, defending_regiment, ComputeCase::WORST),
        ),
        (
            ComputeCase::MEAN,
            create_prediction(attacking_regiment, defending_regiment, ComputeCase::MEAN),
        ),
    ])
}

#[cfg(test)]
mod tests {
    use crate::{model, prediction, regiment};

    use super::{compute_case, compute_mean_case, compute_turn, create_prediction, ComputeCase};

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
    fn test_compute_mean_case() {
        let (attacking, defending): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let mut res: (usize, f64) = compute_mean_case(&attacking, &defending);
        assert_eq!(8, res.0);
        res = compute_mean_case(&defending, &attacking);
        assert_eq!(1, res.0);
    }

    #[test]
    fn test_compute_case() {
        let (attacking, defending): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let mut res: (usize, f64) = compute_case(&attacking, &defending, &ComputeCase::MEAN);
        assert_eq!(res.0, 8);
        res = compute_case(&attacking, &defending, &ComputeCase::WORST);
        assert_eq!(res.0, 1);
        res = compute_case(&attacking, &defending, &ComputeCase::BEST);
        assert_eq!(res.0, 1);
        res = compute_case(&defending, &attacking, &ComputeCase::MEAN);
        assert_eq!(res.0, 1);
        res = compute_case(&defending, &attacking, &ComputeCase::WORST);
        assert_eq!(res.0, 0);
        res = compute_case(&defending, &attacking, &ComputeCase::BEST);
        assert_eq!(res.0, 1);
    }

    #[test]
    fn test_create_prediction() {
        let (attacking, defending): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let res: prediction::Prediction =
            create_prediction(&attacking, &defending, ComputeCase::MEAN);
        assert_eq!(1, res.get_defending_regiment().get_points());
        assert_eq!(8, res.get_attacking_regiment().get_points());

        let res: prediction::Prediction =
            create_prediction(&attacking, &defending, ComputeCase::BEST);
        assert_eq!(0, res.get_defending_regiment().get_points());
        assert_eq!(1, res.get_attacking_regiment().get_points());

        let res: prediction::Prediction =
            create_prediction(&attacking, &defending, ComputeCase::WORST);
        assert_eq!(1, res.get_defending_regiment().get_points());
        assert_eq!(1, res.get_attacking_regiment().get_points());
    }

    #[test]
    fn test_compute_turn() {
        let (attacking, defending): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let res = compute_turn(&attacking, &defending);
        assert_eq!(1, 1);
        assert_eq!(
            res.get(&ComputeCase::MEAN)
                .unwrap()
                .get_attacking_regiment()
                .get_points(),
            8
        );
        assert_eq!(
            res.get(&ComputeCase::BEST)
                .unwrap()
                .get_attacking_regiment()
                .get_points(),
            1
        );
        assert_eq!(
            res.get(&ComputeCase::WORST)
                .unwrap()
                .get_attacking_regiment()
                .get_points(),
            1
        );
        assert_eq!(
            res.get(&ComputeCase::MEAN)
                .unwrap()
                .get_defending_regiment()
                .get_points(),
            1
        );
        assert_eq!(
            res.get(&ComputeCase::BEST)
                .unwrap()
                .get_defending_regiment()
                .get_points(),
            0
        );
        assert_eq!(
            res.get(&ComputeCase::WORST)
                .unwrap()
                .get_defending_regiment()
                .get_points(),
            1
        );
    }
}
