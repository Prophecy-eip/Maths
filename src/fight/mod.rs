//! # Fight Module
//!
//! This module contain all the functions to emulate a fight between 2 Regiment
//! This may evolve to multiple Regiment against multiple Regiment

mod global_values;
use crate::{model, regiment, roll};

fn get_nb_attacks(regiment_attacking: &regiment::Regiment) -> usize {
    regiment_attacking.get_model().get_stats().get_attack() * regiment_attacking.get_cols()
        + ((regiment_attacking.get_model().get_stats().get_attack() * regiment_attacking.get_cols()
            / 2) as usize)
}

fn get_nb_wounds(nb_attacks: usize, to_hit: usize, to_wound: usize) -> usize {
    let nb_hit: usize = (((nb_attacks * (global_values::DEFAULT_DICE - to_hit + 1))
        / global_values::DEFAULT_DICE) as f64)
        .ceil() as usize;
    (((nb_hit * (global_values::DEFAULT_DICE - to_wound + 1)) / global_values::DEFAULT_DICE) as f64)
        .ceil() as usize
}

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

fn fight_first_turn(
    regiment_attacking: &regiment::Regiment,
    regiment_defending: &regiment::Regiment,
) -> usize {
    let regiment_attacking_stats: &model::Stats = regiment_attacking.get_model().get_stats();
    let regiment_defending_stats: &model::Stats = regiment_defending.get_model().get_stats();
    let to_hit: usize = roll::compute_roll_to_hit(
        regiment_attacking_stats.get_offensive(),
        regiment_defending_stats.get_defense(),
    );
    let to_wound: usize = roll::compute_roll_to_wound(
        regiment_attacking_stats.get_strength(),
        regiment_defending_stats.get_resilience(),
    );
    let nb_attacks: usize = get_nb_attacks(regiment_attacking);
    let nb_wounds: usize = get_nb_wounds(nb_attacks, to_hit, to_wound);
    let final_result: usize = get_final_result(
        nb_wounds,
        regiment_defending_stats.get_armour(),
        regiment_attacking_stats.get_armour_penetration(),
    );
    final_result
}

fn find_the_fastest(model_1: &model::Model, model_2: &model::Model) -> usize {
    let model_1_agility: usize = model_1.get_stats().get_agility();
    let model_2_agility: usize = model_2.get_stats().get_agility();

    match model_1_agility.cmp(&model_2_agility) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => 2,
        std::cmp::Ordering::Equal => 0,
    }
}

pub fn resolve_fight(regiment_1: regiment::Regiment, regiment_2: regiment::Regiment) -> usize {
    let fastest: usize = find_the_fastest(regiment_1.get_model(), regiment_2.get_model());

    match fastest {
        1 => fight_first_turn(&regiment_1, &regiment_2),
        2 => fight_first_turn(&regiment_2, &regiment_1),
        0 => unimplemented!("They will fight at the same time !"),
        _ => unreachable!("This should never happen"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{model, regiment, roll};

    use super::{
        fight_first_turn, find_the_fastest, get_final_result, get_nb_attacks, get_nb_wounds,
        resolve_fight,
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
            model::Modifier::new(chaos_warrior_modifier_stats, 0, vec![]);
        let model_chaos_warrior: model::Model =
            model::Model::new(chaos_warrior_stats, vec![chaos_warrior_modifier]);
        let chaos_warrior: regiment::Regiment =
            regiment::Regiment::new(model_chaos_warrior.clone(), 4, 5, 20);
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
            model::Modifier::new(heavy_infantry_modifier_stats, 0, vec![]);
        let model_heavy_infantry: model::Model =
            model::Model::new(heavy_infantry_stats, vec![heavy_infantry_modifier]);
        let heavy_infantry: regiment::Regiment =
            regiment::Regiment::new(model_heavy_infantry.clone(), 4, 5, 20);
        heavy_infantry
    }

    fn initialize_two_units() -> (regiment::Regiment, regiment::Regiment) {
        (initialize_chaos_warrior(), initialize_heavy_infantry())
    }

    #[test]
    fn test_fastest_is_one() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        assert_eq!(
            find_the_fastest(&chaos_warrior.get_model(), &heavy_infantry.get_model()),
            1
        );
    }

    #[test]
    fn test_fastest_is_two() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        assert_eq!(
            find_the_fastest(&heavy_infantry.get_model(), &chaos_warrior.get_model()),
            2
        );
    }

    #[test]
    fn test_fastest_is_none() {
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
            model::Modifier::new(chaos_warrior_modifier_stats, 0, vec![]);
        let model_chaos_warrior: model::Model =
            model::Model::new(chaos_warrior_stats, vec![chaos_warrior_modifier]);
        let chaos_warrior: regiment::Regiment =
            regiment::Regiment::new(model_chaos_warrior.clone(), 4, 5, 20);
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
                agility: 4,
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
            model::Modifier::new(heavy_infantry_modifier_stats, 0, vec![]);
        let model_heavy_infantry: model::Model =
            model::Model::new(heavy_infantry_stats, vec![heavy_infantry_modifier]);
        let heavy_infantry: regiment::Regiment =
            regiment::Regiment::new(model_heavy_infantry.clone(), 4, 5, 20);
        assert_eq!(
            find_the_fastest(&model_heavy_infantry, &model_chaos_warrior),
            0
        );
    }

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

    #[test]
    fn test_nb_wounds_1() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let regiment_attacking_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let to_hit: usize = roll::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = roll::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = get_nb_attacks(&chaos_warrior);
        assert_eq!(get_nb_wounds(nb_attacks, to_hit, to_wound), 8);
    }

    #[test]
    fn test_nb_wounds_2() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let regiment_attacking_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let to_hit: usize = roll::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = roll::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        println!("to_hit: {}", to_hit);
        println!("to_wound: {}", to_wound);
        let nb_attacks: usize = get_nb_attacks(&heavy_infantry);
        assert_eq!(get_nb_wounds(nb_attacks, to_hit, to_wound), 1);
    }

    #[test]
    fn test_final_result_1() {
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        let regiment_attacking_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let to_hit: usize = roll::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = roll::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = get_nb_attacks(&chaos_warrior);
        let nb_wounds: usize = get_nb_wounds(nb_attacks, to_hit, to_wound);
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
        let to_hit: usize = roll::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = roll::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = get_nb_attacks(&chaos_warrior);
        let nb_wounds: usize = get_nb_wounds(nb_attacks, to_hit, to_wound);
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
            model::Modifier::new(chaos_warrior_modifier_stats, 0, vec![]);
        let model_chaos_warrior: model::Model =
            model::Model::new(chaos_warrior_stats, vec![chaos_warrior_modifier]);
        let chaos_warrior: regiment::Regiment =
            regiment::Regiment::new(model_chaos_warrior.clone(), 4, 5, 20);
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
            model::Modifier::new(heavy_infantry_modifier_stats, 0, vec![]);
        let model_heavy_infantry: model::Model =
            model::Model::new(heavy_infantry_stats, vec![heavy_infantry_modifier]);
        let heavy_infantry: regiment::Regiment =
            regiment::Regiment::new(model_heavy_infantry.clone(), 4, 5, 20);
        let regiment_attacking_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let to_hit: usize = roll::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = roll::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = get_nb_attacks(&chaos_warrior);
        let nb_wounds: usize = get_nb_wounds(nb_attacks, to_hit, to_wound);
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
            model::Modifier::new(chaos_warrior_modifier_stats, 0, vec![]);
        let model_chaos_warrior: model::Model =
            model::Model::new(chaos_warrior_stats, vec![chaos_warrior_modifier]);
        let chaos_warrior: regiment::Regiment =
            regiment::Regiment::new(model_chaos_warrior.clone(), 4, 5, 20);
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
            model::Modifier::new(heavy_infantry_modifier_stats, 0, vec![]);
        let model_heavy_infantry: model::Model =
            model::Model::new(heavy_infantry_stats, vec![heavy_infantry_modifier]);
        let heavy_infantry: regiment::Regiment =
            regiment::Regiment::new(model_heavy_infantry.clone(), 4, 5, 20);
        let regiment_attacking_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let to_hit: usize = roll::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = roll::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = get_nb_attacks(&chaos_warrior);
        let nb_wounds: usize = get_nb_wounds(nb_attacks, to_hit, to_wound);
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
            model::Modifier::new(chaos_warrior_modifier_stats, 0, vec![]);
        let model_chaos_warrior: model::Model =
            model::Model::new(chaos_warrior_stats, vec![chaos_warrior_modifier]);
        let chaos_warrior: regiment::Regiment =
            regiment::Regiment::new(model_chaos_warrior.clone(), 4, 5, 20);
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
            model::Modifier::new(heavy_infantry_modifier_stats, 0, vec![]);
        let model_heavy_infantry: model::Model =
            model::Model::new(heavy_infantry_stats, vec![heavy_infantry_modifier]);
        let heavy_infantry: regiment::Regiment =
            regiment::Regiment::new(model_heavy_infantry.clone(), 4, 5, 20);
        let regiment_attacking_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let to_hit: usize = roll::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = roll::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = get_nb_attacks(&chaos_warrior);
        let nb_wounds: usize = get_nb_wounds(nb_attacks, to_hit, to_wound);
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
            model::Modifier::new(chaos_warrior_modifier_stats, 0, vec![]);
        let model_chaos_warrior: model::Model =
            model::Model::new(chaos_warrior_stats, vec![chaos_warrior_modifier]);
        let chaos_warrior: regiment::Regiment =
            regiment::Regiment::new(model_chaos_warrior.clone(), 4, 5, 20);
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
            model::Modifier::new(heavy_infantry_modifier_stats, 0, vec![]);
        let model_heavy_infantry: model::Model =
            model::Model::new(heavy_infantry_stats, vec![heavy_infantry_modifier]);
        let heavy_infantry: regiment::Regiment =
            regiment::Regiment::new(model_heavy_infantry.clone(), 4, 5, 20);
        let regiment_attacking_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let to_hit: usize = roll::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = roll::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = get_nb_attacks(&chaos_warrior);
        let nb_wounds: usize = get_nb_wounds(nb_attacks, to_hit, to_wound);
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
            model::Modifier::new(chaos_warrior_modifier_stats, 0, vec![]);
        let model_chaos_warrior: model::Model =
            model::Model::new(chaos_warrior_stats, vec![chaos_warrior_modifier]);
        let chaos_warrior: regiment::Regiment =
            regiment::Regiment::new(model_chaos_warrior.clone(), 4, 5, 20);
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
            model::Modifier::new(heavy_infantry_modifier_stats, 0, vec![]);
        let model_heavy_infantry: model::Model =
            model::Model::new(heavy_infantry_stats, vec![heavy_infantry_modifier]);
        let heavy_infantry: regiment::Regiment =
            regiment::Regiment::new(model_heavy_infantry.clone(), 4, 5, 20);
        let regiment_attacking_stats: &model::Stats = chaos_warrior.get_model().get_stats();
        let regiment_defending_stats: &model::Stats = heavy_infantry.get_model().get_stats();
        let to_hit: usize = roll::compute_roll_to_hit(
            regiment_attacking_stats.get_offensive(),
            regiment_defending_stats.get_defense(),
        );
        let to_wound: usize = roll::compute_roll_to_wound(
            regiment_attacking_stats.get_strength(),
            regiment_defending_stats.get_resilience(),
        );
        let nb_attacks: usize = get_nb_attacks(&chaos_warrior);
        let nb_wounds: usize = get_nb_wounds(nb_attacks, to_hit, to_wound);
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
        let (chaos_warrior, heavy_infantry): (regiment::Regiment, regiment::Regiment) =
            initialize_two_units();
        assert_eq!(resolve_fight(heavy_infantry, chaos_warrior), 8)
    }
}
