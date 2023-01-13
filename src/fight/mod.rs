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
#[derive(Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ComputeCase {
    BEST,
    WORST,
    MEAN,
}

/// Compute the average damage a unit would dealt to another
///
/// ## Paramaters
/// (&regiment::Regiment) attacking_regiment: The attacker
///
/// (&regiment::Regiment) defending_regiment: The defender
///
/// ## Return
/// (usize, f64): A tuple with first the damage computed and then the probability that it occurs
fn compute_mean_case(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
) -> (usize, f64) {
    let attacking_stats: &model::Stats = attacking_regiment.get_model().get_boosted_stats();
    let defending_stats: &model::Stats = defending_regiment.get_model().get_boosted_stats();
    let damage_probability: f64 =
        computation_tools::compute_damage_probability(attacking_stats, defending_stats);
    let nb_attacks: f64 =
        (attacking_stats.get_attack() as f64 * 1.5 * attacking_regiment.get_cols() as f64).round();
    let damage: usize = std::cmp::min(
        (nb_attacks * damage_probability).round() as usize,
        defending_stats.get_health_points() * defending_regiment.get_nb_models(),
    );

    (
        damage,
        math_tools::compute_bernoulli(nb_attacks as usize, damage, damage_probability),
    )
}

/// Find two thresholds for a gaussien assuring that the area covered reprenset at least 0.06% of the curve
///
/// ## Parameters
/// (usize) nb_touch: The number of hit that the attacker can assume
///
/// (f64) wound_probability: The probability that a hit wound the enemy
///
/// (usize) defender_hp: The amount of health point of the defender
///
/// ## Return
/// (usize, usize): Our two gaussian threshold
fn find_great_gauss_checkpoints(
    nb_touch: usize,
    wound_probability: f64,
    defender_hp: usize,
) -> (usize, usize) {
    let max_hit: usize = std::cmp::min(nb_touch, defender_hp);
    let mut low_checkpoint: usize = (max_hit as f64 * (1.0_f64 / 3.0_f64)).round() as usize;
    let mut high_checkpoint: usize = (max_hit as f64 * (2.0_f64 / 3.0_f64)).round() as usize;
    let compute_proba = |e: usize| math_tools::compute_bernoulli(nb_touch, e, wound_probability);

    while compute_proba(low_checkpoint) < 0.03 && low_checkpoint <= max_hit / 2 {
        low_checkpoint += 1;
    }
    while compute_proba(high_checkpoint) < 0.03 && high_checkpoint >= max_hit / 2 {
        high_checkpoint -= 1;
    }
    (low_checkpoint, high_checkpoint)
}

/// Evaluate the area covered on a gaussian curve
///
/// ## Parameters
/// (usize) start: The start of the inverval
///
/// (usize) end: The end of the interval
///
/// (usize) gauss_len: The lenght of the X axis of the curve
///
/// (f64) success_probability: The probability that one success occurs
///
/// ## Return
/// f64: The percentage of the curve covered by the interval
fn evaluate_gauss_interval(
    start: usize,
    end: usize,
    gauss_len: usize,
    success_probability: f64,
) -> f64 {
    math_tools::sigma(
        start as isize,
        end as isize,
        |current, params, _, _| {
            math_tools::compute_bernoulli(params.unwrap().0, current as usize, params.unwrap().1)
        },
        Some((gauss_len, success_probability)),
    )
}

/// Compute the average damage dealt by a unit to another according to the requested scenario
///
/// ## Parameters
/// (&regiment::Regiment) attacking_regiment: The attacker
///
/// (&regiment::Regiment) defending_regiment: The defender
///
/// (&ComputeCase) case: The scenario from first_unit point of view
///
/// ## Return
/// (usize, f64): The average amount of damage dealt by first_unit and the probability for this scenario to occurs
fn compute_case(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
    case: &ComputeCase,
) -> (usize, f64) {
    let attacking_stats: &model::Stats = attacking_regiment.get_model().get_boosted_stats();
    let nb_touch: usize =
        (attacking_stats.get_attack() as f64 * 1.5 * attacking_regiment.get_cols() as f64).round()
            as usize;
    let wound_probability: f64 = computation_tools::compute_damage_probability(
        attacking_stats,
        defending_regiment.get_model().get_boosted_stats(),
    );
    let defender_hp: usize = defending_regiment.get_regiment_health_points();
    let max_hit: usize = std::cmp::min(nb_touch, defender_hp);
    let checkpoints: (usize, usize) =
        find_great_gauss_checkpoints(nb_touch, wound_probability, defender_hp);

    if let ComputeCase::MEAN = case {
        return compute_mean_case(attacking_regiment, defending_regiment);
    }
    match case {
        ComputeCase::BEST => (
            checkpoints.1,
            evaluate_gauss_interval(checkpoints.1, max_hit, nb_touch, wound_probability),
        ),
        ComputeCase::WORST => (
            checkpoints.0,
            evaluate_gauss_interval(0, checkpoints.0, nb_touch, wound_probability),
        ),
        ComputeCase::MEAN => unreachable!("Code not supposed to be reached!"),
    }
}

/// Make two units fight and return the probability that the fight occurs during the game
///
/// ## Parameters
/// (&mut regiment::Regiment, &ComputeCase) fastest: The fastest regiment
///
/// (&mut regiment::Regiment, &ComputeCase) slowest: The slowest regiment
///
/// (bool) speed_equality: A boolean value to specify if the regiment have the same speed or not
///
/// ## Return
/// f64: The probability that the fight computed occurs
fn apply_fight(
    fastest: (&mut regiment::Regiment, &ComputeCase),
    slowest: (&mut regiment::Regiment, &ComputeCase),
    speed_equality: bool,
) -> f64 {
    let first_damages: (usize, f64) = compute_case(fastest.0, slowest.0, fastest.1);
    let mut second_damages: (usize, f64) = compute_case(slowest.0, fastest.0, slowest.1);

    slowest.0.take_damage(first_damages.0);
    fastest.0.earn_points(first_damages.0);
    if speed_equality {
        fastest.0.take_damage(second_damages.0);
        slowest.0.earn_points(second_damages.0);
        first_damages.1 * second_damages.1
    } else if slowest.0.get_regiment_health_points() > 0 {
        second_damages = compute_case(slowest.0, fastest.0, slowest.1);
        fastest.0.take_damage(second_damages.0);
        slowest.0.earn_points(second_damages.0);
        first_damages.1 * second_damages.1
    } else {
        first_damages.1
    }
}

/// Compute a full scenario
///
/// ## Parameters
/// (&regiment::Regiment) attacking_regiment: The first unit
///
/// (&regiment::Regiment) defending_regiment: The second unit
///
/// (&ComputeCase) case: The scenario from first unit point of view
///
/// ## Return
/// Prediction: The prediction computed according to the specified Compute Case
fn create_prediction(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
    case: &ComputeCase,
) -> prediction::Prediction {
    let complementary: ComputeCase = match case {
        ComputeCase::BEST => ComputeCase::WORST,
        ComputeCase::WORST => ComputeCase::BEST,
        ComputeCase::MEAN => ComputeCase::MEAN,
    };
    let fastest: u8 = computation_tools::find_the_fastest(
        attacking_regiment.get_model().get_boosted_stats(),
        defending_regiment.get_model().get_boosted_stats(),
    );

    let mut final_defending: regiment::Regiment = defending_regiment.clone();
    let mut final_attacking: regiment::Regiment = attacking_regiment.clone();
    let probability: f64 = if fastest < 2 {
        apply_fight(
            (&mut final_attacking, case),
            (&mut final_defending, &complementary),
            fastest == 0,
        )
    } else {
        apply_fight(
            (&mut final_defending, &complementary),
            (&mut final_attacking, case),
            false,
        )
    };
    prediction::Prediction::new(final_attacking, final_defending, probability)
}

/// Compute the 3 most important scenario while in melee phase
///
/// ## Parameters
/// (&regiment::Regiment) attacking_regiment: The attacking regiment
///
/// (&regiment::Regiment) defending_regiment: The defending regiment
///
/// ## Return
/// (HashMap<ComputeCase, Prediction>): The more realistic scenario, The best scenario for first unit, The worst scenario for first unit
pub fn compute_turn(
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
) -> std::collections::HashMap<ComputeCase, Prediction> {
    std::collections::HashMap::from([
        (
            ComputeCase::BEST,
            create_prediction(attacking_regiment, defending_regiment, &ComputeCase::BEST),
        ),
        (
            ComputeCase::WORST,
            create_prediction(attacking_regiment, defending_regiment, &ComputeCase::WORST),
        ),
        (
            ComputeCase::MEAN,
            create_prediction(attacking_regiment, defending_regiment, &ComputeCase::MEAN),
        ),
    ])
}

#[cfg(test)]
mod tests {
    use crate::{global_test, prediction, regiment};

    use super::{compute_case, compute_mean_case, compute_turn, create_prediction, ComputeCase};

    fn initialize_chaos_warrior() -> regiment::Regiment {
        let chaos_warrior: regiment::Regiment = global_test::tests::initialize_regiment(
            global_test::tests::initialize_stats(4, 8, 8, 1, 5, 4, 0, 0, 2, 4, 5, 1, 4),
            4,
            5,
            20,
        );
        chaos_warrior
    }

    fn initialize_heavy_infantry() -> regiment::Regiment {
        let heavy_infantry: regiment::Regiment = global_test::tests::initialize_regiment(
            global_test::tests::initialize_stats(4, 8, 7, 1, 3, 3, 0, 0, 1, 3, 3, 0, 3),
            4,
            5,
            20,
        );
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
        assert_eq!(res.0, 5);
        res = compute_case(&attacking, &defending, &ComputeCase::BEST);
        assert_eq!(res.0, 10);
        res = compute_case(&defending, &attacking, &ComputeCase::MEAN);
        assert_eq!(res.0, 1);
        res = compute_case(&defending, &attacking, &ComputeCase::WORST);
        assert_eq!(res.0, 3);
        res = compute_case(&defending, &attacking, &ComputeCase::BEST);
        assert_eq!(res.0, 3);
    }

    #[test]
    fn test_create_prediction() {
        let (attacking, defending): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let res: prediction::Prediction =
            create_prediction(&attacking, &defending, &ComputeCase::MEAN);
        assert_eq!(1, res.get_defending_regiment().get_points());
        assert_eq!(8, res.get_attacking_regiment().get_points());
        let res: prediction::Prediction =
            create_prediction(&attacking, &defending, &ComputeCase::BEST);
        assert_eq!(3, res.get_defending_regiment().get_points());
        assert_eq!(10, res.get_attacking_regiment().get_points());

        let res: prediction::Prediction =
            create_prediction(&attacking, &defending, &ComputeCase::WORST);
        assert_eq!(3, res.get_defending_regiment().get_points());
        assert_eq!(5, res.get_attacking_regiment().get_points());
    }

    #[test]
    fn test_compute_turn() {
        let (attacking, defending): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let res = compute_turn(&attacking, &defending);
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
            10
        );
    }
}
