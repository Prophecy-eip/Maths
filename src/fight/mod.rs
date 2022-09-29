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

/// Compute the number of wound the defending Regiment will suffer after armor save
///
/// # Parameters
/// nb_wounds (usize): The number of wounds the defending Regiment will suffer
///
/// regiment_defending_armor (usize): The armor of the defending Regiment
///
/// regiment_attacking_armor_penetration (usize): The armor penetration of the attacking Regiment
///
/// # Return
/// usize: The number of wounds the defending Regiment will suffer after armor save
fn get_final_result(
    nb_wounds: usize,
    regiment_defending_armor: usize,
    regiment_attacking_armor_penetration: usize,
) -> usize {
    let to_save: i8 = regiment_defending_armor as i8 - regiment_attacking_armor_penetration as i8;
    let throw_to_save: i8 = match to_save {
        i8::MIN..=0 => return nb_wounds,
        1 => 6,
        2 => 5,
        3 => 4,
        4 => 3,
        5..=i8::MAX => 2,
    };
    let nb_save: usize = ((nb_wounds) * (global_values::DEFAULT_DICE - throw_to_save as usize + 1))
        / global_values::DEFAULT_DICE;
    nb_wounds - nb_save
}

/// Compute the first round of a fight between 2 Regiment
///
/// # Parameters
/// regiment_attacking (&regiment::Regiment): The Regiment attacking
///
/// regiment_defending (&regiment::Regiment): The Regiment defending
///
/// # Return
/// usize: The number of wounds the defending Regiment will suffer
fn fight_first_turn(
    regiment_attacking: &regiment::Regiment,
    regiment_defending: &regiment::Regiment,
) -> usize {
    let regiment_attacking_stats: &model::Stats = regiment_attacking.get_model().get_stats();
    let regiment_defending_stats: &model::Stats = regiment_defending.get_model().get_stats();
    let to_hit: usize = computation_tools::compute_roll_to_hit(
        regiment_attacking_stats.get_offensive(),
        regiment_defending_stats.get_defense(),
    );
    let to_wound: usize = computation_tools::compute_roll_to_wound(
        regiment_attacking_stats.get_strength(),
        regiment_defending_stats.get_resilience(),
    );
    let nb_attacks: usize = computation_tools::get_nb_attacks(regiment_attacking);
    let nb_wounds: usize = computation_tools::compute_nb_wounds(nb_attacks, to_hit, to_wound);
    let final_result: usize = get_final_result(
        nb_wounds,
        regiment_defending_stats.get_armour(),
        regiment_attacking_stats.get_armour_penetration(),
    );
    final_result
}

/// Resolve a fight between 2 Regiment
/// !! Only the first round is resolved !!
///
/// # Parameters
/// regiment_1 (regiment::Regiment): The first Regiment
///
/// regiment_2 (regiment::Regiment): The second Regiment
///
/// # Return
/// usize: The number of wounds the second Regiment will suffer !! Temporary !!
pub fn resolve_fight(regiment_1: regiment::Regiment, regiment_2: regiment::Regiment) -> usize {
    let fastest: u8 =
        computation_tools::find_the_fastest(regiment_1.get_model(), regiment_2.get_model());

    match fastest {
        1 => fight_first_turn(&regiment_1, &regiment_2),
        2 => fight_first_turn(&regiment_2, &regiment_1),
        0 => unimplemented!("They will fight at the same time !"),
        _ => unreachable!("This should never happen"),
    }
}

/// ## Compute the average damage a unit would dealt to another
///
/// ### Paramaters
/// (&Regiment) first_unit -> The attacker
///
/// (&Regiment) second_unit -> The defender
///
/// ### Return
/// (usize, f64) -> A tuple with first the damage computed and then the probability that it occurs
pub fn compute_mean_case(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
) -> (usize, f64) {
    let damage_probability: f64 = computation_tools::compute_damage_probability(
        attacking_regiment.get_model(),
        defending_regiment.get_model(),
    );
    let nb_attacks = (attacking_regiment.get_model().get_stats().get_attack() as f64
        * 1.5
        * attacking_regiment.get_cols() as f64)
        .round();
    let damage = std::cmp::min(
        (nb_attacks * damage_probability).round() as usize,
        defending_regiment
            .get_model()
            .get_stats()
            .get_health_point()
            * defending_regiment.get_nb_models(),
    );

    (
        damage,
        math_tools::compute_bernoulli(nb_attacks as usize, damage, damage_probability),
    )
}

/// ## Compute the average damage dealt by a unit to another according to the requested scenario
///
/// ### Parameters
/// (&Regiment) first_unit -> The attacker
///
/// (&Regiment) second_unit -> The defender
///
/// (ComputeCase) case -> The scenario from first_unit point of view
///
/// ### Return
/// (usize, f64) -> The average amount of damage dealt by first_unit and the probability for this scenario to occurs
pub fn compute_case(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
    case: ComputeCase,
) -> (usize, f64) {
    let nb_touch: usize = (attacking_regiment.get_model().get_stats().get_attack() as f64
        * 1.5
        * attacking_regiment.get_cols() as f64)
        .round() as usize;
    let touch_probability: f64 = computation_tools::compute_damage_probability(
        attacking_regiment.get_model(),
        defending_regiment.get_model(),
    );
    let mut results: Vec<(usize, f64)> = Vec::new();
    let mut selection: Vec<(usize, f64)> = Vec::new();
    let res: (usize, f64);
    let mut bondaries: (isize, isize);
    let threshold: f64 = match case {
        ComputeCase::BEST => global_values::BEST_CASE_THRESHOLD,
        ComputeCase::WORST => global_values::WORST_CASE_THRESHOLD,
        _ => 0.0_f64,
    };

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
        ComputeCase::BEST => (
            defending_regiment
                .get_model()
                .get_stats()
                .get_health_point() as isize,
            nb_touch as isize,
        ),
        ComputeCase::WORST => (
            0,
            defending_regiment
                .get_model()
                .get_stats()
                .get_health_point() as isize,
        ),
        ComputeCase::MEAN => unreachable!("Code not supposed to be reached!"),
    };
    match res.0.cmp(
        &defending_regiment
            .get_model()
            .get_stats()
            .get_health_point(),
    ) {
        Ordering::Less => res,
        Ordering::Greater => (
            defending_regiment
                .get_model()
                .get_stats()
                .get_health_point(),
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
/// (&Regiment) first_unit -> The first unit
///
/// (&Regiment) second_unit -> The second unit
///
/// (ComputeCase) case -> The scenario from first unit point of view
///
/// ### Return
/// (usize, usize, f64) -> The average amount of damage dealt by the two units and the probability for this scenario to occurs
pub fn compute_turn(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
    case: ComputeCase,
) -> (usize, usize, f64) {
    let complementary: ComputeCase = match case {
        ComputeCase::BEST => ComputeCase::WORST,
        ComputeCase::WORST => ComputeCase::BEST,
        ComputeCase::MEAN => ComputeCase::MEAN,
    };
    let fastest: u8 = computation_tools::find_the_fastest(
        attacking_regiment.get_model(),
        defending_regiment.get_model(),
    );
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
            if first_damages.0
                > defending_regiment
                    .get_model()
                    .get_stats()
                    .get_health_point()
            {
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
            if second_damages.0
                > attacking_regiment
                    .get_model()
                    .get_stats()
                    .get_health_point()
            {
                (0, second_damages.0, second_damages.1)
            } else {
                (
                    first_damages.0,
                    second_damages.0,
                    first_damages.1 * second_damages.1,
                )
            }
        }
        _ => panic!("Code not supposed to be reached!"),
    }
}

/// ## Create a prediction from raw turn computation
///
/// ### Parameters
/// (&regiment::Regiment) first_unit -> The first unit
///
/// (&regiment::Regiment) second_unit -> The second unit
///
/// (usize, usize, f64) raw -> The raw scenario computation
///
/// ### Return
/// Prediction -> The scenarion represented with the Prediction data structure
fn turn_result_to_prediction(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
    raw: (usize, usize, f64),
) -> prediction::Prediction {
    let mut first: regiment::Regiment = attacking_regiment.clone();
    let mut second: regiment::Regiment = defending_regiment.clone();
    first.take_damage(raw.1);
    second.take_damage(raw.0);
    prediction::Prediction::new(
        prediction::RegimentResult::new(first, raw.0),
        prediction::RegimentResult::new(second, raw.1),
        raw.2,
    )
}

/// ## Compute the 3 most important scenario while in melee phase
///
/// ### Parameters
/// (&Unit) first_unit -> The first unit
///
/// (&Unit) second_unit -> The second unit
///
/// ### Return
/// (\[Prediction; 3\]) -> \[The more realistic scenario, The best scenario for first unit, The worst scenario for first unit\]
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
    use crate::{model, regiment};

    use super::{computation_tools, fight_first_turn, get_final_result, resolve_fight};

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
    fn test_final_result_1() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let regiment_attacking_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let to_hit: usize = computation_tools::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = computation_tools::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = computation_tools::get_nb_attacks(&chaos_warrior);
        let nb_wounds: usize = computation_tools::compute_nb_wounds(nb_attacks, to_hit, to_wound);
        assert_eq!(
            get_final_result(
                nb_wounds,
                regiment_defending_stats.get_armour(),
                regiment_attacking_stats.get_armour_penetration()
            ),
            8
        )
    }

    #[test]
    fn test_final_result_2() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let regiment_attacking_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let to_hit: usize = computation_tools::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = computation_tools::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = computation_tools::get_nb_attacks(&chaos_warrior);
        let nb_wounds: usize = computation_tools::compute_nb_wounds(nb_attacks, to_hit, to_wound);
        assert_eq!(
            get_final_result(
                nb_wounds,
                regiment_defending_stats.get_armour(),
                regiment_attacking_stats.get_armour_penetration()
            ),
            2
        )
    }

    #[test]
    fn test_final_result_3() {
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
                offensive: 5,
                strength: 4,
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
                offensive: 0,
                strength: 0,
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
                armour: 5,
                aegis: 0,
            },
            model::OffensiveStats {
                attack: 1,
                offensive: 3,
                strength: 3,
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
                offensive: 0,
                strength: 0,
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
        let regiment_attacking_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let to_hit: usize = computation_tools::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = computation_tools::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = computation_tools::get_nb_attacks(&chaos_warrior);
        let nb_wounds: usize = computation_tools::compute_nb_wounds(nb_attacks, to_hit, to_wound);
        assert_eq!(
            get_final_result(
                nb_wounds,
                regiment_defending_stats.get_armour(),
                regiment_attacking_stats.get_armour_penetration()
            ),
            2
        )
    }

    #[test]
    fn test_final_result_4() {
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
                offensive: 5,
                strength: 4,
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
                offensive: 0,
                strength: 0,
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
                armour: 4,
                aegis: 0,
            },
            model::OffensiveStats {
                attack: 1,
                offensive: 3,
                strength: 3,
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
                offensive: 0,
                strength: 0,
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
        let regiment_attacking_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let to_hit: usize = computation_tools::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = computation_tools::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = computation_tools::get_nb_attacks(&chaos_warrior);
        let nb_wounds: usize = computation_tools::compute_nb_wounds(nb_attacks, to_hit, to_wound);
        assert_eq!(
            get_final_result(
                nb_wounds,
                regiment_defending_stats.get_armour(),
                regiment_attacking_stats.get_armour_penetration()
            ),
            3
        )
    }

    #[test]
    fn test_final_result_5() {
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
                offensive: 5,
                strength: 4,
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
                offensive: 0,
                strength: 0,
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
                armour: 3,
                aegis: 0,
            },
            model::OffensiveStats {
                attack: 1,
                offensive: 3,
                strength: 3,
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
                offensive: 0,
                strength: 0,
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
        let regiment_attacking_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let to_hit: usize = computation_tools::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = computation_tools::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = computation_tools::get_nb_attacks(&chaos_warrior);
        let nb_wounds: usize = computation_tools::compute_nb_wounds(nb_attacks, to_hit, to_wound);
        assert_eq!(
            get_final_result(
                nb_wounds,
                regiment_defending_stats.get_armour(),
                regiment_attacking_stats.get_armour_penetration()
            ),
            4
        )
    }

    #[test]
    fn test_final_result_6() {
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
                offensive: 5,
                strength: 4,
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
                offensive: 0,
                strength: 0,
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
                armour: 2,
                aegis: 0,
            },
            model::OffensiveStats {
                attack: 1,
                offensive: 3,
                strength: 3,
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
                offensive: 0,
                strength: 0,
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
        let regiment_attacking_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let to_hit: usize = computation_tools::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = computation_tools::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = computation_tools::get_nb_attacks(&chaos_warrior);
        let nb_wounds: usize = computation_tools::compute_nb_wounds(nb_attacks, to_hit, to_wound);
        assert_eq!(
            get_final_result(
                nb_wounds,
                regiment_defending_stats.get_armour(),
                regiment_attacking_stats.get_armour_penetration()
            ),
            5
        )
    }

    #[test]
    fn test_final_result_7() {
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
                offensive: 5,
                strength: 4,
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
                offensive: 0,
                strength: 0,
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
                armour: 6,
                aegis: 0,
            },
            model::OffensiveStats {
                attack: 1,
                offensive: 3,
                strength: 3,
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
                offensive: 0,
                strength: 0,
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
        let regiment_attacking_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let to_hit: usize = computation_tools::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = computation_tools::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = computation_tools::get_nb_attacks(&chaos_warrior);
        let nb_wounds: usize = computation_tools::compute_nb_wounds(nb_attacks, to_hit, to_wound);
        assert_eq!(
            get_final_result(
                nb_wounds,
                regiment_defending_stats.get_armour(),
                regiment_attacking_stats.get_armour_penetration()
            ),
            1
        )
    }

    #[test]
    fn test_fight_first_turn_1() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        assert_eq!(fight_first_turn(&chaos_warrior, &heavy_infantry), 8)
    }

    #[test]
    fn test_fight_first_turn_2() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        assert_eq!(fight_first_turn(&heavy_infantry, &chaos_warrior), 1)
    }

    #[test]
    fn test_resolve_fight_1() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        assert_eq!(resolve_fight(chaos_warrior, heavy_infantry), 8)
    }

    #[test]
    fn test_resolve_fight_2() {
        let chaos_warrior: regiment::Regiment = initialize_chaos_warrior();
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
                agility: 2,
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
        assert_eq!(resolve_fight(heavy_infantry, chaos_warrior), 8)
    }
}
