use crate::fight::computation_tools;
use crate::fight::global_values;
use crate::fight::ComputeCase;
use crate::modifier::StatsModifier;
use crate::stat;

/// Compute the value to hit the opponent in the shooting phase
///
/// # Parameters
/// aim (isize): The aim Stats of the weapon modifier within the attacking Regiment
///
/// resilience (usize): The resilience Stats of the defending Regiment
///
/// # Return
/// usize: The minimum roll to hit the opponent
pub fn compute_roll_to_hit_shoot(aim: isize, resilience: usize) -> usize {
    let difference: i8 = aim as i8 - resilience as i8;

    match difference {
        i8::MIN..=-8 => 6,
        -7..=-4 => 5,
        -3..=0 => 4,
        1..=3 => 3,
        4..=i8::MAX => 2,
    }
}

/// Compute the probability for a model to wound an another
///
/// # Parameters
///
/// defending_stats (&stat::Stats): The defender stats
///
/// weapon_aim (isize): The aim of the weapon
///
/// weapon_strength (isize): The strength of the weapon
///
/// # Return
/// f64: The probability for attacking_stats to wound target
fn compute_wound_probability_shooting(
    defending_stats: &stat::Stats,
    weapon_aim: isize,
    weapon_strength: isize,
) -> f64 {
    let minimum_to_hit: usize =
        compute_roll_to_hit_shoot(weapon_aim, defending_stats.get_resilience());
    let minimum_to_wound: usize = computation_tools::compute_roll_to_wound(
        weapon_strength as usize,
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

/// Compute the probability for a model to damage another
///
/// This function take account of the defender defensive stats
///
/// # Parameters
///
/// defending_stats (&stat::Stats): The defender stats
///
/// weapon_aim (isize): The aim of the weapon
///
/// weapon_strength (isize): The strength of the weapon
///
/// weapon_ap (isize): The armour penetration of the weapon
///
/// # Return
/// f64: The probability that a hit wound the defender
pub fn compute_damage_probability_shooting(
    defending_stats: &stat::Stats,
    weapon_aim: isize,
    weapon_strength: isize,
    weapon_ap: isize,
) -> f64 {
    compute_wound_probability_shooting(defending_stats, weapon_aim, weapon_strength)
        * (1.0_f64 - compute_save_probability_shooting(defending_stats, weapon_ap))
}

/// Compute the probability for a model to protect itself from another model damages
///
/// # Parameters
/// defending_stats (&stat::Stats): The defender stats
///
/// weapon_ap (isize): The weapon armour penetration attached to the attacking unit weapon modifier
///
/// # Return
/// f64: The probability to save a damage dealt by attacker
fn compute_save_probability_shooting(defending_stats: &stat::Stats, weapon_ap: isize) -> f64 {
    let armour_save: usize = match global_values::ARMOUR_SAVE_THRESHOLD as isize
        - (defending_stats.get_armour() as isize - weapon_ap)
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

/// Compute the average damage a unit would dealt to another
///
/// # Paramaters
/// attacking_regiment (&regiment::Regiment): The attacker
///
/// defending_regiment (&regiment::Regiment): The defender
///
/// weapon_aim (isize): The aim of the weapon
///
/// weapon_strength (isize): The strength of the weapon
///
/// weapon_ap (isize): The armour penetration of the weapon
///
/// # Return
/// (usize, f64): A tuple with first the damage computed and then the probability that it occurs
fn compute_mean_case_shooting(
    attacking_regiment: &crate::regiment::Regiment,
    defending_regiment: &crate::regiment::Regiment,
    weapon_aim: isize,
    weapon_strength: isize,
    weapon_ap: isize,
) -> (usize, f64) {
    let defending_stats: &stat::Stats = defending_regiment.get_model().get_boosted_stats();

    let damage_probability: f64 = compute_damage_probability_shooting(
        defending_stats,
        weapon_aim,
        weapon_strength,
        weapon_ap,
    );

    let nb_attacks: f64 = (weapon_aim as f64 * 1.5 * attacking_regiment.get_cols() as f64).round();
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
pub fn compute_case_shooting(
    attacking_regiment: &crate::regiment::Regiment,
    defending_regiment: &crate::regiment::Regiment,
    case: &ComputeCase,
) -> (usize, f64) {
    let mut weapon_aim: isize = 0;
    let mut weapon_strength: isize = 0;
    let mut weapon_ap: isize = 0;
    let attacking_modifiers_weapon = attacking_regiment.get_model().get_modifiers();

    if let StatsModifier::Weapon(weapon_stats) = &attacking_modifiers_weapon[0].get_stats_modifier()
    {
        weapon_aim = weapon_stats.get_aim().unwrap_or(0);
        weapon_strength = weapon_stats.get_strength();
        weapon_ap = weapon_stats.get_armour_penetration();
    }

    let nb_touch: usize =
        (weapon_aim as f64 * 1.5 * attacking_regiment.get_cols() as f64).round() as usize;
    let wound_probability: f64 = compute_damage_probability_shooting(
        defending_regiment.get_model().get_boosted_stats(),
        weapon_aim,
        weapon_strength,
        weapon_ap,
    );
    let defender_hp: usize = defending_regiment.get_regiment_health_points();
    let max_hit: usize = std::cmp::min(nb_touch, defender_hp);
    let checkpoints: (usize, usize) =
        crate::math_tools::find_great_gauss_checkpoints(nb_touch, wound_probability, defender_hp);

    if let ComputeCase::MEAN = case {
        return compute_mean_case_shooting(
            attacking_regiment,
            defending_regiment,
            weapon_aim,
            weapon_strength,
            weapon_ap,
        );
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
    use super::*;
    use crate::{model, modifier, regiment, stat};

    #[test]
    fn test_hit_shoot() {
        assert_eq!(compute_roll_to_hit_shoot(1, 1), 4);
        assert_eq!(compute_roll_to_hit_shoot(1, 5), 5);
        assert_eq!(compute_roll_to_hit_shoot(1, 9), 6);
        assert_eq!(compute_roll_to_hit_shoot(6, 4), 3);
        assert_eq!(compute_roll_to_hit_shoot(8, 3), 2);
    }

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
    ) -> stat::Stats {
        let stats: stat::Stats = stat::Stats::new(
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
            },
        );
        stats
    }

    fn initialize_heavy_infantry_with_weapon() -> regiment::Regiment {
        let h_stats = initialize_stats_w_ballistic(4, 8, 7, 1, 3, 3, 0, 0, 1, 3, 3, 0, 3);
        let h_modifiers: modifier::Modifier =
            modifier::Modifier::new_weapon(Some(1), None, 3, 0, None, false);
        let h_model: model::Model = model::Model::new(h_stats, vec![h_modifiers], false);

        let h_infantry: regiment::Regiment = regiment::Regiment::new(h_model, 4, 5, 20, None);
        h_infantry
    }

    fn initialize_chaos_warrior_with_weapon() -> regiment::Regiment {
        let c_stats = initialize_stats_w_ballistic(4, 8, 8, 1, 5, 4, 0, 0, 2, 4, 5, 1, 4);
        let c_modifiers: modifier::Modifier =
            modifier::Modifier::new_weapon(Some(1), None, 4, 1, None, false);
        let c_model: model::Model = model::Model::new(c_stats, vec![c_modifiers], false);

        let c_warrior: regiment::Regiment = regiment::Regiment::new(c_model, 4, 5, 20, None);
        c_warrior
    }

    pub fn initialize_two_units_with_weapon() -> (regiment::Regiment, regiment::Regiment) {
        (
            initialize_chaos_warrior_with_weapon(),
            initialize_heavy_infantry_with_weapon(),
        )
    }

    #[test]
    fn test_compute_case_shoot() {
        let (attacking, defending): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units_with_weapon();
        let mut res: (usize, f64) =
            compute_case_shooting(&attacking, &defending, &ComputeCase::MEAN);
        assert_eq!(res.0, 3);
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
}
