//! # Fight Module
//!
//! This module contain all the functions to emulate a fight between 2 Regiment
//! This may evolve to multiple Regiment against multiple Regiment

mod computation_tools;
mod global_values;

use crate::regiment;

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

/// This is the enumeration of the differents attacking positions
///
/// FRONT: The attacking regiment is charging from the front of the defending unit
/// FLANK: The attacking regiment is charging from the left or the right flank of the defending unit
/// BACK: The attacking regiment is charging from the back of the defending unit
#[derive(Clone, Copy)]
pub enum AttackPosition {
    FRONT,
    FLANK,
    BACK,
}

/// Struct containing a fight case, a probability of that case to happen and the two regiments
///
/// ## Attributes
///
///
/// probability (f64): The probability of that case to happen
///
/// attacking_regiment (regiment::Regiment): The attacking regiment
///
/// defending_regiment (regiment::Regiment): The defending regiment
///
///
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct FightCase {
    probability: f64,
    attacking_regiment: regiment::Regiment,
    defending_regiment: regiment::Regiment,
}

impl FightCase {
    pub fn new(
        probability: f64,
        attacking_regiment: regiment::Regiment,
        defending_regiment: regiment::Regiment,
    ) -> FightCase {
        FightCase {
            probability,
            attacking_regiment,
            defending_regiment,
        }
    }

    pub fn get_attacking_regiment(&self) -> regiment::Regiment {
        self.attacking_regiment.clone()
    }

    pub fn get_defending_regiment(&self) -> regiment::Regiment {
        self.defending_regiment.clone()
    }

    pub fn get_probability(&self) -> f64 {
        self.probability
    }
}

pub struct FightPredictionResult {
    best_case: FightCase,
    worst_case: FightCase,
    mean_case: FightCase,
}

impl FightPredictionResult {
    pub fn new(
        best_case: FightCase,
        worst_case: FightCase,
        mean_case: FightCase,
    ) -> FightPredictionResult {
        FightPredictionResult {
            best_case,
            worst_case,
            mean_case,
        }
    }

    pub fn get_best_case(&self) -> FightCase {
        self.best_case.clone()
    }

    pub fn get_worst_case(&self) -> FightCase {
        self.worst_case.clone()
    }

    pub fn get_mean_case(&self) -> FightCase {
        self.mean_case.clone()
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
    let first_damages: (usize, f64) =
        computation_tools::compute_case(fastest.0, slowest.0, fastest.1);
    let mut second_damages: (usize, f64) =
        computation_tools::compute_case(slowest.0, fastest.0, slowest.1);

    if fastest.0.get_model().is_banner_bearer() {
        fastest.0.earn_points(1);
    }
    if slowest.0.get_model().is_banner_bearer() {
        slowest.0.earn_points(1);
    }
    slowest.0.take_damage(first_damages.0);
    fastest.0.earn_points(first_damages.0);
    if speed_equality {
        fastest.0.take_damage(second_damages.0);
        slowest.0.earn_points(second_damages.0);
        first_damages.1 * second_damages.1
    } else if slowest.0.get_regiment_health_points() > 0 {
        second_damages = computation_tools::compute_case(slowest.0, fastest.0, slowest.1);
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
/// (crate::web_server::converter::web_objects::AttackPosition) attacking_position: The position of the attacking Regiment
///
/// (&regiment::Regiment) attacking_regiment: The first unit
///
/// (&regiment::Regiment) defending_regiment: The second unit
///
/// (&ComputeCase) case: The scenario from first unit point of view
///
/// ## Return
/// Prediction: The prediction computed according to the specified Compute Case
fn create_prediction(
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
    // as we consider the attacking regiment's charging. According to the rules, they get one bonus point
    final_attacking.earn_points(1);
    match attacking_position {
        AttackPosition::FRONT => final_attacking.earn_points(0),
        AttackPosition::FLANK => final_attacking.earn_points(1),
        AttackPosition::BACK => final_attacking.earn_points(2),
    };
    FightCase::new(probability, final_attacking, final_defending)
}

/// Compute the 3 most important scenario while in melee phase
///
/// ## Parameters
/// (crate::web_server::converter::web_objects::AttackPosition) attacking_position: The position of the attacking Regiment
///
/// (&regiment::Regiment) attacking_regiment: The attacking regiment
///
/// (&regiment::Regiment) defending_regiment: The defending regiment
///
/// ## Return
/// (HashMap<ComputeCase, Prediction>): The more realistic scenario, The best scenario for first unit, The worst scenario for first unit
pub fn compute_turn(
    attacking_position: AttackPosition,
    attacking_regiment: &regiment::Regiment,
    defending_regiment: &regiment::Regiment,
) -> FightPredictionResult {
    let best_case: FightCase = create_prediction(
        attacking_position,
        attacking_regiment,
        defending_regiment,
        &ComputeCase::BEST,
    );
    let worst_case: FightCase = create_prediction(
        attacking_position,
        attacking_regiment,
        defending_regiment,
        &ComputeCase::WORST,
    );
    let mean_case: FightCase = create_prediction(
        attacking_position,
        attacking_regiment,
        defending_regiment,
        &ComputeCase::MEAN,
    );
    FightPredictionResult::new(best_case, worst_case, mean_case)
}

#[cfg(test)]
mod tests {
    use crate::fight::{create_prediction, compute_turn};
use crate::fight::computation_tools::compute_case;
    use crate::{global_test, regiment};

    use super::ComputeCase;

    fn initialize_chaos_warrior() -> regiment::Regiment {
        let chaos_warrior: regiment::Regiment = global_test::tests::initialize_regiment(
            global_test::tests::initialize_stats(4, 8, 8, 1, 5, 4, 0, 0, 2, 4, 5, 1, 4),
            4,
            5,
            20,
            false,
        );
        chaos_warrior
    }

    fn initialize_chaos_warrior_with_banner() -> regiment::Regiment {
        let chaos_warrior: regiment::Regiment = global_test::tests::initialize_regiment(
            global_test::tests::initialize_stats(4, 8, 8, 1, 5, 4, 0, 0, 2, 4, 5, 1, 4),
            4,
            5,
            20,
            true,
        );
        chaos_warrior
    }

    fn initialize_heavy_infantry() -> regiment::Regiment {
        let heavy_infantry: regiment::Regiment = global_test::tests::initialize_regiment(
            global_test::tests::initialize_stats(4, 8, 7, 1, 3, 3, 0, 0, 1, 3, 3, 0, 3),
            4,
            5,
            20,
            false,
        );
        heavy_infantry
    }

    fn initialize_heavy_infantry_with_banner() -> regiment::Regiment {
        let heavy_infantry: regiment::Regiment = global_test::tests::initialize_regiment(
            global_test::tests::initialize_stats(4, 8, 7, 1, 3, 3, 0, 0, 1, 3, 3, 0, 3),
            4,
            5,
            20,
            true,
        );
        heavy_infantry
    }

    pub fn initialize_two_units() -> (regiment::Regiment, regiment::Regiment) {
        (initialize_chaos_warrior(), initialize_heavy_infantry())
    }

    fn initialize_two_units_with_banner() -> (regiment::Regiment, regiment::Regiment) {
        (
            initialize_chaos_warrior_with_banner(),
            initialize_heavy_infantry_with_banner(),
        )
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
        let attacking_position: crate::fight::AttackPosition = crate::fight::AttackPosition::FRONT;
        let (attacking, defending): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let res: crate::fight::FightCase = create_prediction(
            attacking_position,
            &attacking,
            &defending,
            &ComputeCase::MEAN,
        );
        assert_eq!(1, res.get_defending_regiment().get_points());
        assert_eq!(9, res.get_attacking_regiment().get_points());
        let res: crate::fight::FightCase = create_prediction(
            attacking_position,
            &attacking,
            &defending,
            &ComputeCase::BEST,
        );
        assert_eq!(3, res.get_defending_regiment().get_points());
        assert_eq!(11, res.get_attacking_regiment().get_points());

        let res: crate::fight::FightCase = create_prediction(
            attacking_position,
            &attacking,
            &defending,
            &ComputeCase::WORST,
        );
        assert_eq!(3, res.get_defending_regiment().get_points());
        assert_eq!(6, res.get_attacking_regiment().get_points());
    }

    #[test]
    fn test_compute_turn() {
        let (attacking, defending): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let res: super::FightPredictionResult = compute_turn(crate::fight::AttackPosition::FRONT, &attacking, &defending);
        assert_eq!(
            res.get_mean_case().get_attacking_regiment().get_points(),
            9
        );
        assert_eq!(
            res.get_best_case().get_attacking_regiment().get_points(),
            11
        );
        assert_eq!(
            res.get_worst_case().get_attacking_regiment().get_points(),
            6
        );
    }

    #[test]
    fn test_compute_turn_with_banner() {
        let (attacking, defending): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units_with_banner();
        let res: crate::fight::FightPredictionResult = compute_turn(crate::fight::AttackPosition::FRONT, &attacking, &defending);
        assert_eq!(
            res.get_mean_case().get_attacking_regiment().get_points(),
            10
        );
        assert_eq!(
            res.get_best_case().get_attacking_regiment().get_points(),
            12
        );
        assert_eq!(
            res.get_worst_case().get_attacking_regiment().get_points(),
            7
        );
    }
}
