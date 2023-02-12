//! # Shooting phase Module
//!
//! This module contain all the functions to emulate the shooting between 2 Regiment
//! This may evolve to multiple Regiment against multiple Regiment

use crate::{
    fight::shooting_tools, fight::AttackPosition, fight::ComputeCase, fight::FightCase,
    fight::FightPredictionResult, regiment,
};

/// Make two units shot and return the probability that the fight occurs during the game
///
/// # Parameters
/// attackers (&mut regiment::Regiment, &ComputeCase): The attacking regiment
///
/// defender (&mut regiment::Regiment, &ComputeCase): The defending regiment
///
/// # Return
/// f64: The probability that the shoot computed occurs
fn apply_shoot(
    attacker: (&mut regiment::Regiment, &ComputeCase),
    defender: (&mut regiment::Regiment, &ComputeCase),
) -> f64 {
    let first_damages: (usize, f64) =
        shooting_tools::compute_case_shooting(attacker.0, defender.0, attacker.1);
    let second_damages: (usize, f64) =
        shooting_tools::compute_case_shooting(defender.0, attacker.0, defender.1);

    if attacker.0.get_model().is_banner_bearer() {
        attacker.0.earn_points(1);
    }
    if defender.0.get_model().is_banner_bearer() {
        defender.0.earn_points(1);
    }
    defender.0.take_damage(first_damages.0);
    attacker.0.earn_points(first_damages.0);

    if defender.0.get_regiment_health_points() > 0 {
        attacker.0.take_damage(second_damages.0);
        defender.0.earn_points(second_damages.0);
        first_damages.1 * second_damages.1
    } else {
        first_damages.1
    }
}

/// Create a prediction for a fight between two units
///
/// # Parameters
/// attacking_position (AttackPosition): The position of the attacking Regiment
///
/// attacking_regiment (&regiment::Regiment): The first unit
///
/// defending_regiment (&regiment::Regiment): The second unit
///
/// case (&ComputeCase): The scenario from first unit point of view
///
/// # Return
/// FightCase: The case of the fight
fn create_prediction_shoot(
    attacking_position: AttackPosition,
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
    case: &ComputeCase,
) -> FightCase {
    let complementary: ComputeCase = match case {
        ComputeCase::BEST => ComputeCase::WORST,
        ComputeCase::WORST => ComputeCase::BEST,
        ComputeCase::MEAN => ComputeCase::MEAN,
    };

    let mut final_defending: regiment::Regiment = defending_regiment.clone();
    let mut final_attacking: regiment::Regiment = attacking_regiment.clone();
    let probability_attack: f64 = apply_shoot(
        (&mut final_attacking, case),
        (&mut final_defending, &complementary),
    );
    // as we consider the attacking regiment's charging. According to the rules, they get one bonus point
    final_attacking.earn_points(1);
    match attacking_position {
        AttackPosition::FRONT => final_attacking.earn_points(0),
        AttackPosition::FLANK => final_attacking.earn_points(1),
        AttackPosition::BACK => final_attacking.earn_points(2),
    };
    FightCase::new(probability_attack, final_attacking, final_defending)
}

/// Compute the 3 most important scenario while in melee phase
///
/// # Parameters
/// attacking_position (AttackPosition): The position of the attacking Regiment
///
/// attacking_regiment (&regiment::Regiment): The attacking regiment
///
/// defending_regiment (&regiment::Regiment): The defending regiment
///
/// # Return
/// FightPredictionResult: The best scenario, the worst scenario and the mean scenario for the attacking unit
pub fn compute_turn_shoot(
    attacking_position: AttackPosition,
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
) -> FightPredictionResult {
    let best_case: FightCase = create_prediction_shoot(
        attacking_position,
        attacking_regiment,
        defending_regiment,
        &ComputeCase::BEST,
    );
    let worst_case: FightCase = create_prediction_shoot(
        attacking_position,
        attacking_regiment,
        defending_regiment,
        &ComputeCase::WORST,
    );
    let mean_case: FightCase = create_prediction_shoot(
        attacking_position,
        attacking_regiment,
        defending_regiment,
        &ComputeCase::MEAN,
    );
    FightPredictionResult::new(best_case, worst_case, mean_case)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fight::shooting_tools::compute_case_shooting;
    use crate::{global_test, regiment, stat};

    use super::ComputeCase;

    pub fn initialize_stats_w_ballistic(
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
        ballistic_skill: usize,
    ) -> stat::Stats {
        let stats: stat::Stats = stat::Stats::new_ballistic(
            stat::GlobalStats {
                advance,
                march,
                discipline,
            },
            stat::DefensiveStats {
                health_points,
                defense,
                resilience,
                armour,
                aegis,
            },
            stat::OffensiveStats {
                attack,
                offensive,
                strength,
                armour_penetration,
                agility,
                ballistic_skill: Some(ballistic_skill),
            },
        );
        stats
    }

    fn initialize_chaos_warrior() -> regiment::Regiment {
        let chaos_warrior: regiment::Regiment = global_test::tests::initialize_regiment(
            initialize_stats_w_ballistic(4, 8, 8, 1, 5, 4, 0, 0, 2, 4, 5, 1, 4, 4),
            4,
            5,
            20,
            false,
        );
        chaos_warrior
    }

    fn initialize_heavy_infantry() -> regiment::Regiment {
        let heavy_infantry: regiment::Regiment = global_test::tests::initialize_regiment(
            initialize_stats_w_ballistic(4, 8, 7, 1, 3, 3, 0, 0, 1, 3, 3, 0, 3, 3),
            4,
            5,
            20,
            false,
        );
        heavy_infantry
    }

    pub fn initialize_two_units() -> (regiment::Regiment, regiment::Regiment) {
        (initialize_chaos_warrior(), initialize_heavy_infantry())
    }

    #[test]
    fn test_compute_case_shoot() {
        let (attacking, defending): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let mut res: (usize, f64) =
            compute_case_shooting(&attacking, &defending, &ComputeCase::MEAN);
        assert_eq!(res.0, 4);
        res = compute_case_shooting(&attacking, &defending, &ComputeCase::WORST);
        assert_eq!(res.0, 3);
        res = compute_case_shooting(&attacking, &defending, &ComputeCase::BEST);
        assert_eq!(res.0, 5);
        res = compute_case_shooting(&defending, &attacking, &ComputeCase::MEAN);
        assert_eq!(res.0, 1);
        res = compute_case_shooting(&defending, &attacking, &ComputeCase::WORST);
        assert_eq!(res.0, 3);
        res = compute_case_shooting(&defending, &attacking, &ComputeCase::BEST);
        assert_eq!(res.0, 3);
    }

    #[test]
    fn test_create_prediction() {
        let attacking_position: crate::fight::AttackPosition = crate::fight::AttackPosition::FRONT;
        let (attacking, defending): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let res: crate::fight::FightCase = create_prediction_shoot(
            attacking_position,
            &attacking,
            &defending,
            &ComputeCase::MEAN,
        );
        assert_eq!(1, res.get_defending_regiment().get_points());
        assert_eq!(5, res.get_attacking_regiment().get_points());
        let res: crate::fight::FightCase = create_prediction_shoot(
            attacking_position,
            &attacking,
            &defending,
            &ComputeCase::BEST,
        );
        assert_eq!(3, res.get_defending_regiment().get_points());
        assert_eq!(6, res.get_attacking_regiment().get_points());

        let res: crate::fight::FightCase = create_prediction_shoot(
            attacking_position,
            &attacking,
            &defending,
            &ComputeCase::WORST,
        );
        assert_eq!(3, res.get_defending_regiment().get_points());
        assert_eq!(4, res.get_attacking_regiment().get_points());
    }

    #[test]
    fn test_compute_turn_shoot() {
        let (attacking, defending): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let res: super::FightPredictionResult =
            compute_turn_shoot(crate::fight::AttackPosition::FRONT, &attacking, &defending);
        assert_eq!(res.get_mean_case().get_attacking_regiment().get_points(), 5);
        assert_eq!(res.get_best_case().get_attacking_regiment().get_points(), 6);
        assert_eq!(
            res.get_worst_case().get_attacking_regiment().get_points(),
            4
        );
    }
}
