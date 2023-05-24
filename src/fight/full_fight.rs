use crate::fight::*;

/// Make two units fight and return the probability that the fight occurs during the game
///
/// # Parameters
/// fastest (&mut regiment::Regiment, &ComputeCase): The fastest regiment
///
/// slowest (&mut regiment::Regiment, &ComputeCase): The slowest regiment
///
/// speed_equality (bool): A boolean value to specify if the regiment have the same speed or not
///
/// # Return
/// f64: The probability that the fight computed occurs
fn apply_fight_all(
    fastest: (&mut regiment::Regiment, &ComputeCase),
    slowest: (&mut regiment::Regiment, &ComputeCase),
    speed_equality: bool,
) -> f64 {
    let first_damages: (usize, f64) =
        computation_tools::compute_case(fastest.0, slowest.0, fastest.1);
    let first_damages_shoot: (usize, f64) =
        shooting_tools::compute_case_shooting(fastest.0, slowest.0, fastest.1);
    let mut second_damages: (usize, f64) =
        computation_tools::compute_case(slowest.0, fastest.0, slowest.1);
    let mut second_damages_shoot: (usize, f64) =
        shooting_tools::compute_case_shooting(slowest.0, fastest.0, slowest.1);

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
        (first_damages.1 + first_damages_shoot.1) * (second_damages.1 + second_damages_shoot.1)
    } else if slowest.0.get_regiment_health_points() > 0 {
        second_damages = computation_tools::compute_case(slowest.0, fastest.0, slowest.1);
        second_damages_shoot =
            shooting_tools::compute_case_shooting(slowest.0, fastest.0, slowest.1);
        fastest.0.take_damage(second_damages.0);
        slowest.0.earn_points(second_damages.0);
        first_damages.1 + first_damages_shoot.1 * second_damages.1 + second_damages_shoot.1
    } else {
        first_damages.1 + first_damages_shoot.1
    }
}

/// Create a prediction for a fight between two units taking all the phases in consideration
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
pub fn create_prediction_all(
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
        apply_fight_all(
            (&mut final_attacking, case),
            (&mut final_defending, &complementary),
            fastest == 0,
        )
    } else {
        apply_fight_all(
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
