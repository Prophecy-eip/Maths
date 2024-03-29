use crate::fight::global_values;
use crate::fight::ComputeCase;
use crate::stat;

/// Compute the value to hit the opponent
///
/// # Parameters
/// offensive (usize): The offensive Stats of the attacking Regiment
///
/// defense (usize): The defense Stats of the defending Regiment
///
/// # Return
/// usize: The minimum roll to hit the opponent
pub fn compute_roll_to_hit(offensive: usize, defense: usize) -> usize {
    let difference: i8 = offensive as i8 - defense as i8;

    match difference {
        i8::MIN..=-8 => 6,
        -7..=-4 => 5,
        -3..=0 => 4,
        1..=3 => 3,
        4..=i8::MAX => 2,
    }
}

/// Compute the value to wound the opponent
///
/// # Parameters
/// strength (usize): The strength Stats of the attacking Regiment
///
/// resilience (usize): The resilience Stats of the defending Regiment
///
/// # Return
/// usize: The minimum roll to wound the opponent
pub fn compute_roll_to_wound(strength: usize, resilience: usize) -> usize {
    let difference: i8 = strength as i8 - resilience as i8;

    match difference {
        i8::MIN..=-2 => 6,
        -1 => 5,
        0 => 4,
        1 => 3,
        2..=i8::MAX => 2,
    }
}

/// Compute the probability for a model to wound an another
///
/// # Parameters
/// attacking_stats (&model::Stats): The attacker stats
///
/// defending_stats (&model::Stats): The defender stats
///
/// # Return
/// f64: The probability for attacking_stats to wound target
fn compute_wound_probability(attacking_stats: &stat::Stats, defending_stats: &stat::Stats) -> f64 {
    let minimum_to_hit: usize = compute_roll_to_hit(
        attacking_stats.get_offensive(),
        defending_stats.get_defense(),
    );
    let minimum_to_wound: usize = compute_roll_to_wound(
        attacking_stats.get_strength(),
        defending_stats.get_resilience(),
    );
    let hit_probability: f64 = (global_values::DEFAULT_DICE - (minimum_to_hit as f64) + 1.0_f64)
        / (global_values::DEFAULT_DICE);
    let wound_probability: f64 = (global_values::DEFAULT_DICE - (minimum_to_wound as f64)
        + 1.0_f64)
        / (global_values::DEFAULT_DICE);
    match hit_probability * wound_probability {
        x if x <= 0.0 => 0.0,
        y => y,
    }
}

/// Return the fastest Model between 2 Model
/// If the first Model is faster than the second Model, return 1
/// If the second Model is faster than the first Model, return 2
/// If the Models have the same speed, return 0
///
/// # Parameters
/// attacking_stats (&model::Stats): The attacker stats
///
/// defending_stats (&model::Stats): The defender stats
///
/// # Return
/// u8: The fastest model
///
/// 0 if the two units have the same speed
///
/// 1 if the attacking is the fastest
///
/// 2 if the defending is the fastest)
pub fn find_the_fastest(attacking_stats: &stat::Stats, defending_stats: &stat::Stats) -> u8 {
    let attacking_model_agility: usize = attacking_stats.get_agility() + 1;
    let defending_model_agility: usize = defending_stats.get_agility();

    match attacking_model_agility.cmp(&defending_model_agility) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => 2,
        std::cmp::Ordering::Equal => 0,
    }
}

/// Compute the probability for a model to protect itself from another model damages
///
/// # Parameters
/// defending_stats (&model::Stats): The defender stats
///
/// attacking_stats (&model::Stats): The attacker stats
///
/// # Return
/// f64: The probability to save a damage dealt by attacker
fn compute_save_probability(defending_stats: &stat::Stats, attacking_stats: &stat::Stats) -> f64 {
    let armour_save: usize = match global_values::ARMOUR_SAVE_THRESHOLD as isize
        - (defending_stats.get_armour() as isize
            - attacking_stats.get_armour_penetration() as isize)
    {
        x if x < 0 => 0usize,
        y => y as usize,
    };
    let save_proba: f64 = ((global_values::DEFAULT_DICE - armour_save as f64 + 1.0)
        / global_values::DEFAULT_DICE)
        .clamp(0.0, 1.0);
    let aegis: usize = defending_stats.get_aegis();
    let aegis_proba: f64 = ((global_values::DEFAULT_DICE - aegis as f64 + 1.0)
        / global_values::DEFAULT_DICE)
        .clamp(0.0, 1.0);
    if aegis == 0 {
        save_proba
    } else {
        save_proba + aegis_proba - (aegis_proba * save_proba)
    }
}

/// Compute the probability for a model to damage another
///
/// This function take account of the defender defensive stats
///
/// # Parameters
/// attacking_stats (&model::Stats): The attacker stats
///
/// defending_stats (&model::Stats): The defender stats
///
/// # Return
/// f64: The probability that a hit wound the defender
pub fn compute_damage_probability(
    attacking_stats: &stat::Stats,
    defending_stats: &stat::Stats,
) -> f64 {
    compute_wound_probability(attacking_stats, defending_stats)
        * (1.0_f64 - compute_save_probability(defending_stats, attacking_stats))
}

/// Compute the average damage a unit would dealt to another
///
/// # Paramaters
/// attacking_regiment (&regiment::Regiment): The attacker
///
/// defending_regiment (&regiment::Regiment): The defender
///
/// # Return
/// (usize, f64): A tuple with first the damage computed and then the probability that it occurs
fn compute_mean_case(
    attacking_regiment: &crate::regiment::Regiment,
    defending_regiment: &crate::regiment::Regiment,
) -> (usize, f64) {
    let attacking_stats: &stat::Stats = attacking_regiment.get_model().get_boosted_stats();
    let defending_stats: &stat::Stats = defending_regiment.get_model().get_boosted_stats();
    let damage_probability: f64 = compute_damage_probability(attacking_stats, defending_stats);
    let nb_attacks: f64 =
        (attacking_stats.get_attack() as f64 * 1.5 * attacking_regiment.get_cols() as f64).round();
    let damage: usize = std::cmp::min(
        (nb_attacks * damage_probability).round() as usize,
        defending_stats.get_health_points() * defending_regiment.get_nb_models(),
    );

    (
        damage,
        crate::math_tools::compute_bernoulli(nb_attacks as usize, damage, damage_probability),
    )
}

/// Compute the average damage dealt by a unit to another according to the requested scenario
///
/// # Parameters
/// attacking_regiment (&regiment::Regiment): The attacker
///
/// defending_regiment (&regiment::Regiment): The defender
///
/// case (&ComputeCase): The scenario from first_unit point of view
///
/// # Return
/// (usize, f64): The average amount of damage dealt by first_unit and the probability for this scenario to occurs
pub fn compute_case(
    attacking_regiment: &crate::regiment::Regiment,
    defending_regiment: &crate::regiment::Regiment,
    case: &ComputeCase,
) -> (usize, f64) {
    let attacking_stats: &stat::Stats = attacking_regiment.get_model().get_boosted_stats();
    let nb_touch: usize =
        (attacking_stats.get_attack() as f64 * 1.5 * attacking_regiment.get_cols() as f64).round()
            as usize;
    let wound_probability: f64 = compute_damage_probability(
        attacking_stats,
        defending_regiment.get_model().get_boosted_stats(),
    );
    let defender_hp: usize = defending_regiment.get_regiment_health_points();
    let max_hit: usize = std::cmp::min(nb_touch, defender_hp);
    let checkpoints: (usize, usize) =
        crate::math_tools::find_great_gauss_checkpoints(nb_touch, wound_probability, defender_hp);

    if let ComputeCase::MEAN = case {
        return compute_mean_case(attacking_regiment, defending_regiment);
    }
    match case {
        ComputeCase::BEST => (
            checkpoints.1,
            crate::math_tools::evaluate_gauss_interval(
                checkpoints.1,
                max_hit,
                nb_touch,
                wound_probability,
            ),
        ),
        ComputeCase::WORST => (
            checkpoints.0,
            crate::math_tools::evaluate_gauss_interval(
                0,
                checkpoints.0,
                nb_touch,
                wound_probability,
            ),
        ),
        ComputeCase::MEAN => unreachable!("Code not supposed to be reached!"),
    }
}

#[cfg(test)]
mod tests {
    use super::compute_mean_case;
    use crate::{
        fight::computation_tools::{
            compute_damage_probability, compute_roll_to_hit, compute_roll_to_wound,
            compute_save_probability, compute_wound_probability, find_the_fastest,
        },
        model, modifier, regiment, stat,
    };

    #[test]
    fn test_compute_mean_case() {
        let (attacking, defending): (crate::regiment::Regiment, crate::regiment::Regiment) =
            crate::fight::tests::initialize_two_units();
        let mut res: (usize, f64) = compute_mean_case(&attacking, &defending);
        assert_eq!(8, res.0);
        res = compute_mean_case(&defending, &attacking);
        assert_eq!(1, res.0);
    }

    fn initialize_chaos_warrior() -> regiment::Regiment {
        let chaos_warrior_stats: stat::Stats = stat::Stats::new(
            stat::GlobalStats {
                advance: 4,
                march: 8,
                discipline: 8,
            },
            stat::DefensiveStats {
                health_points: 1,
                defense: 5,
                resilience: 4,
                armour: 0,
                aegis: 0,
            },
            stat::OffensiveStats {
                attack: 2,
                strength: 5,
                offensive: 4,
                armour_penetration: 1,
                agility: 5,
            },
        );
        let chaos_warrior_modifier: modifier::Modifier =
            modifier::Modifier::new_weapon(Some(3), None, 0, 0);
        let model_chaos_warrior: model::Model =
            model::Model::new(chaos_warrior_stats, vec![chaos_warrior_modifier], false);
        let chaos_warrior: regiment::Regiment =
            regiment::Regiment::new(model_chaos_warrior, 4, 5, 20, None);
        chaos_warrior
    }

    fn initialize_heavy_infantry() -> regiment::Regiment {
        let heavy_infantry_stats: stat::Stats = stat::Stats::new(
            stat::GlobalStats {
                advance: 4,
                march: 8,
                discipline: 7,
            },
            stat::DefensiveStats {
                health_points: 1,
                defense: 3,
                resilience: 3,
                armour: 0,
                aegis: 0,
            },
            stat::OffensiveStats {
                attack: 1,
                strength: 3,
                offensive: 3,
                armour_penetration: 0,
                agility: 3,
            },
        );
        let heavy_infantry_modifier: modifier::Modifier =
            modifier::Modifier::new_weapon(Some(3), None, 0, 0);
        let model_heavy_infantry: model::Model =
            model::Model::new(heavy_infantry_stats, vec![heavy_infantry_modifier], false);
        let heavy_infantry: regiment::Regiment =
            regiment::Regiment::new(model_heavy_infantry, 4, 5, 20, None);
        heavy_infantry
    }

    fn initialize_buffed_heavy_infantry() -> regiment::Regiment {
        let heavy_infantry_stats: stat::Stats = stat::Stats::new(
            stat::GlobalStats {
                advance: 4,
                march: 8,
                discipline: 7,
            },
            stat::DefensiveStats {
                health_points: 1,
                defense: 3,
                resilience: 3,
                armour: 2,
                aegis: 0,
            },
            stat::OffensiveStats {
                attack: 1,
                strength: 3,
                offensive: 3,
                armour_penetration: 0,
                agility: 4,
            },
        );
        let heavy_infantry_modifier: modifier::Modifier =
            modifier::Modifier::new_weapon(Some(3), None, 0, 0);
        let model_heavy_infantry: model::Model =
            model::Model::new(heavy_infantry_stats, vec![heavy_infantry_modifier], false);
        let heavy_infantry: regiment::Regiment =
            regiment::Regiment::new(model_heavy_infantry, 4, 5, 20, None);
        heavy_infantry
    }

    fn initialize_aegis_heavy_infantry() -> regiment::Regiment {
        let heavy_infantry_stats: stat::Stats = stat::Stats::new(
            stat::GlobalStats {
                advance: 4,
                march: 8,
                discipline: 7,
            },
            stat::DefensiveStats {
                health_points: 1,
                defense: 3,
                resilience: 3,
                armour: 0,
                aegis: 3,
            },
            stat::OffensiveStats {
                attack: 1,
                strength: 3,
                offensive: 3,
                armour_penetration: 0,
                agility: 3,
            },
        );
        let heavy_infantry_modifier: modifier::Modifier =
            modifier::Modifier::new_weapon(Some(3), None, 0, 0);
        let model_heavy_infantry: model::Model =
            model::Model::new(heavy_infantry_stats, vec![heavy_infantry_modifier], false);
        let heavy_infantry: regiment::Regiment =
            regiment::Regiment::new(model_heavy_infantry, 4, 5, 20, None);
        heavy_infantry
    }

    fn initialize_two_units() -> (regiment::Regiment, regiment::Regiment) {
        (initialize_chaos_warrior(), initialize_heavy_infantry())
    }

    #[test]
    fn test_hit() {
        assert_eq!(compute_roll_to_hit(1, 1), 4);
        assert_eq!(compute_roll_to_hit(1, 5), 5);
        assert_eq!(compute_roll_to_hit(1, 9), 6);
        assert_eq!(compute_roll_to_hit(6, 4), 3);
        assert_eq!(compute_roll_to_hit(8, 3), 2);
    }

    #[test]
    fn test_wound() {
        assert_eq!(compute_roll_to_wound(1, 1), 4);
        assert_eq!(compute_roll_to_wound(1, 2), 5);
        assert_eq!(compute_roll_to_wound(1, 3), 6);
        assert_eq!(compute_roll_to_wound(3, 2), 3);
        assert_eq!(compute_roll_to_wound(4, 2), 2);
    }

    #[test]
    fn test_fastest_is_one() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        assert_eq!(
            find_the_fastest(
                &chaos_warrior.get_model().get_boosted_stats(),
                &heavy_infantry.get_model().get_boosted_stats()
            ),
            1
        );
    }

    #[test]
    fn test_fastest_is_two() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        assert_eq!(
            find_the_fastest(
                &heavy_infantry.get_model().get_boosted_stats(),
                &chaos_warrior.get_model().get_boosted_stats()
            ),
            2
        );
    }

    #[test]
    fn test_fastest_is_none() {
        let first_unit = initialize_chaos_warrior();
        let second_unit = initialize_buffed_heavy_infantry();
        assert_eq!(
            find_the_fastest(
                &second_unit.get_model().get_boosted_stats(),
                &first_unit.get_model().get_boosted_stats()
            ),
            0
        );
    }

    #[test]
    fn test_compute_wound_probability_strongest() {
        let first_unit = initialize_chaos_warrior();
        let second_unit = initialize_heavy_infantry();

        let wound_probability: f64 = compute_wound_probability(
            &first_unit.get_model().get_boosted_stats(),
            &second_unit.get_model().get_boosted_stats(),
        );
        assert_eq!(wound_probability - 0.555 < 0.001, true);
    }

    #[test]
    fn test_compute_wound_probability_weakest() {
        let first_unit = initialize_chaos_warrior();
        let second_unit = initialize_heavy_infantry();

        let wound_probability: f64 = compute_wound_probability(
            &second_unit.get_model().get_boosted_stats(),
            &first_unit.get_model().get_boosted_stats(),
        );
        assert_eq!(wound_probability - 0.166 < 0.001, true);
    }

    #[test]
    fn test_compute_save_probability_aegis() {
        let first_unit = initialize_chaos_warrior();
        let second_unit = initialize_aegis_heavy_infantry();

        let save_probability: f64 = compute_save_probability(
            &second_unit.get_model().get_boosted_stats(),
            &first_unit.get_model().get_boosted_stats(),
        );
        assert_eq!(save_probability - 0.667 < 0.001, true);
    }

    #[test]
    fn test_compute_save_probability_strongest() {
        let first_unit = initialize_chaos_warrior();
        let second_unit = initialize_heavy_infantry();

        let save_probability: f64 = compute_save_probability(
            &first_unit.get_model().get_boosted_stats(),
            &second_unit.get_model().get_boosted_stats(),
        );
        assert_eq!(save_probability, 0.0_f64);
    }

    #[test]
    fn test_compute_save_probability_weakest() {
        let first_unit = initialize_chaos_warrior();
        let second_unit = initialize_heavy_infantry();

        let save_probability: f64 = compute_save_probability(
            &second_unit.get_model().get_boosted_stats(),
            &first_unit.get_model().get_boosted_stats(),
        );
        assert_eq!(save_probability, 0.0_f64);
    }

    #[test]
    fn test_compute_save_probability() {
        let first_unit = initialize_buffed_heavy_infantry();
        let second_unit = initialize_chaos_warrior();

        let save_probability: f64 = compute_save_probability(
            &first_unit.get_model().get_boosted_stats(),
            &second_unit.get_model().get_boosted_stats(),
        );
        assert_eq!(save_probability - 0.166 < 0.001, true);
    }

    #[test]
    fn test_compute_damage_probability_strongest() {
        let first_unit = initialize_buffed_heavy_infantry();
        let second_unit = initialize_chaos_warrior();

        let damage_probability: f64 = compute_damage_probability(
            &first_unit.get_model().get_boosted_stats(),
            &second_unit.get_model().get_boosted_stats(),
        );
        assert_eq!(damage_probability - 0.555 < 0.001, true);
    }

    #[test]
    fn test_compute_damage_probability_weakest() {
        let first_unit = initialize_buffed_heavy_infantry();
        let second_unit = initialize_chaos_warrior();

        let damage_probability: f64 = compute_damage_probability(
            &second_unit.get_model().get_boosted_stats(),
            &first_unit.get_model().get_boosted_stats(),
        );
        assert_eq!(damage_probability - 0.462 < 0.001, true);
    }
}
