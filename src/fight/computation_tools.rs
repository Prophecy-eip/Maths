use crate::fight::global_values;
use crate::{model, regiment};

/// Compute the value to hit the opponent
///
/// # Parameters
/// offensive (usize): The offensive Stats of the attacking Regiment
///
/// defense (usize): The defense Stats of the defending Regiment
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

/// Compute the number of attacks of a Regiment
///
/// # Parameters
/// regiment (regiment::Regiment): The Regiment attacking
///
/// # Return
/// usize: The number of attacks of the Regiment
pub fn get_nb_attacks(regiment_attacking: &regiment::Regiment) -> usize {
    (regiment_attacking.get_model().get_stats().get_attack() as f64
        * regiment_attacking.get_cols() as f64
        * 1.5)
        .floor() as usize
}

/// ## Compute the probability for a model to wound an another
///
/// ### Parameters
/// (&Unit) main -> The attacker
///
/// (&Unit) target -> The defender
///
/// ### Return
/// isize -> The probability for main to wound target
pub fn compute_wound_probability(attacking: &model::Model, defending: &model::Model) -> f64 {
    let minimum_to_hit: usize = compute_roll_to_hit(
        attacking.get_stats().get_offensive(),
        defending.get_stats().get_defense(),
    );

    let minimum_to_wound: usize = compute_roll_to_wound(
        attacking.get_stats().get_strength(),
        defending.get_stats().get_resilience(),
    );
    let hit_probability: f64 = (global_values::DEFAULT_DICE as f64 - (minimum_to_hit as f64)
        + 1.0_f64)
        / (global_values::DEFAULT_DICE as f64);
    let wound_probability: f64 = (global_values::DEFAULT_DICE as f64 - (minimum_to_wound as f64)
        + 1.0_f64)
        / (global_values::DEFAULT_DICE as f64);
    match hit_probability * wound_probability {
        x if x <= 0.0 => 0.0,
        y => y,
    }
}

/// Compute the number of wounds a Regiment will suffer
///
/// # Parameters
/// nb_attacks (usize): The number of attacks of the attacking Regiment
///
/// to_hit (usize): The value to hit the defending Regiment
///
/// to_wound (usize): The value to wound the defending Regiment
///
/// # Return
/// usize: The number of wounds the defending Regiment will suffer
pub fn compute_nb_wounds(nb_attacks: usize, to_hit: usize, to_wound: usize) -> usize {
    let nb_hit: usize = (((nb_attacks * (global_values::DEFAULT_DICE - to_hit + 1))
        / global_values::DEFAULT_DICE) as f64)
        .round() as usize;
    (((nb_hit * (global_values::DEFAULT_DICE - to_wound + 1)) / global_values::DEFAULT_DICE) as f64)
        .round() as usize
}

/// Return the fastest Model between 2 Model
/// If the first Model is faster than the second Model, return 1
/// If the second Model is faster than the first Model, return 2
/// If the Models have the same speed, return 0
///
/// # Parameters
/// attacking_model (&model::Model): The first Model
///
/// defending_model (&model::Model): The second Model
///
/// # Return
/// usize: The fastest Model
pub fn find_the_fastest(attacking_model: &model::Model, defending_model: &model::Model) -> u8 {
    let attacking_model_agility: usize = attacking_model.get_stats().get_agility() + 1;
    let defending_model_agility: usize = defending_model.get_stats().get_agility();

    match attacking_model_agility.cmp(&defending_model_agility) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => 2,
        std::cmp::Ordering::Equal => 0,
    }
}

/// ## Compute the probability for a model to protect itself from another model damages
///
/// ### Parameters
/// (&Model) main -> The defending unit
///
/// (&Model) attacker -> The attacking unit
///
/// ### Return
/// f64 -> The probability to save a damage inflited by attacker
pub fn compute_save_probability(defending: &model::Model, attacking: &model::Model) -> f64 {
    let armour_save: usize = match global_values::ARMOUR_SAVE_THRESHOLD as isize
        - (defending.get_stats().get_armour() as isize - attacking.get_stats().get_armour_penetration() as isize)
    {
        x if x < 0 => 0usize,
        y => y as usize,
    };
    let save_proba: f64 = match (global_values::DEFAULT_DICE as f64 - armour_save as f64 + 1.0)
        / global_values::DEFAULT_DICE as f64
    {
        x if x <= 0.0 => 0.0,
        x if x > 1.0 => 1.0,
        y => y,
    };
    let aegis_proba: f64 = match (global_values::DEFAULT_DICE as f64 - defending.get_stats().get_aegis() as f64
        + 1.0)
        / global_values::DEFAULT_DICE as f64
    {
        x if x <= 0.0 => 0.0,
        x if x > 1.0 => 1.0,
        y => y,
    };
    if defending.get_stats().get_aegis() == 0 {
        save_proba
    } else {
        save_proba + aegis_proba - (aegis_proba * save_proba)
    }
}

/// ## Compute the probability for a model to damage another
///
/// ### Parameters
/// (&Model) first_unit -> The attacker
///
/// (&Model) second_unit -> The defender
///
/// ### Return
/// f64 -> The probability that first_unit inflict 1 damage to second_unit
pub fn compute_damage_probability(first_unit: &model::Model, second_unit: &model::Model) -> f64 {
    compute_wound_probability(first_unit, second_unit)
        * (1.0_f64 - compute_save_probability(second_unit, first_unit))
}


#[cfg(test)]
mod tests {
    use super::{
        compute_nb_wounds, compute_roll_to_hit, compute_roll_to_wound, find_the_fastest,
        get_nb_attacks,
    };
    use crate::{model, regiment};

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
                agility: 5,
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

    // get_nb_attacks

    #[test]
    fn test_nb_attacks_1() {
        let chaos_warrior: regiment::Regiment = initialize_chaos_warrior();
        assert_eq!(get_nb_attacks(&chaos_warrior), 15);
    }

    #[test]
    fn test_nb_attacks_2() {
        let heavy_infantry: regiment::Regiment = initialize_heavy_infantry();
        assert_eq!(get_nb_attacks(&heavy_infantry), 7);
    }

    // compute_nb_wounds

    #[test]
    fn test_nb_wounds_1() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let regiment_attacking_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let to_hit: usize = compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = get_nb_attacks(&chaos_warrior);
        assert_eq!(compute_nb_wounds(nb_attacks, to_hit, to_wound), 8);
    }

    #[test]
    fn test_nb_wounds_2() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let regiment_attacking_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let to_hit: usize = compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = get_nb_attacks(&heavy_infantry);
        assert_eq!(compute_nb_wounds(nb_attacks, to_hit, to_wound), 1);
    }

    #[test]
    fn test_fastest_is_one() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        assert_eq!(
            find_the_fastest(chaos_warrior.get_model(), heavy_infantry.get_model()),
            1
        );
    }

    #[test]
    fn test_fastest_is_two() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        assert_eq!(
            find_the_fastest(heavy_infantry.get_model(), chaos_warrior.get_model()),
            2
        );
    }

    /*#[test]
    fn test_fastest_is_none() {
        let first_unit = initialize_chaos_warrior();
        let second_unit = initialize_chaos_warrior();
        assert_eq!(
            find_the_fastest(first_unit.get_model(), second_unit.get_model()),
            0
        );
    }*/
}
