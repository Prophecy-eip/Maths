// use maths::fight;
// use maths::prediction;
// use maths::regiment;
// mod initialize_units;

// #[test]
// fn test_warriors_against_heavy_infantry() {
//     let attacking_position: maths::web_server::converter::web_objects::AttackPosition = maths::web_server::converter::web_objects::AttackPosition::FRONT;
//     let attacking: regiment::Regiment = initialize_units::initialize_warriors();
//     let defending: regiment::Regiment = initialize_units::initialize_heavy_infantry();
//     let res: std::collections::HashMap<fight::ComputeCase, prediction::Prediction> =
//         fight::compute_turn(attacking_position, &attacking, &defending);
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         9
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         11
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         6
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         1
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
// }

// #[test]
// fn test_warriors_against_wildhorn_herd() {
//     let attacking_position: maths::web_server::converter::web_objects::AttackPosition = maths::web_server::converter::web_objects::AttackPosition::FRONT;
//     let attacking: regiment::Regiment = initialize_units::initialize_warriors();
//     let defending: regiment::Regiment = initialize_units::initialize_wildhorn_herd();
//     let res: std::collections::HashMap<fight::ComputeCase, prediction::Prediction> =
//         fight::compute_turn(attacking_position, &attacking, &defending);
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         6
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         9
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         6
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         1
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
// }

// #[test]
// fn test_imps_against_wildhorn_herd() {
//     let attacking_position: maths::web_server::converter::web_objects::AttackPosition = maths::web_server::converter::web_objects::AttackPosition::FRONT;
//     let attacking: regiment::Regiment = initialize_units::initialize_imps();
//     let defending: regiment::Regiment = initialize_units::initialize_wildhorn_herd();
//     let res: std::collections::HashMap<fight::ComputeCase, prediction::Prediction> =
//         fight::compute_turn(attacking_position, &attacking, &defending);
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         2
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         4
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         6
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         2
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         4
//     );
// }

// #[test]
// fn test_imps_against_heavy_infantry() {
//     let attacking_position: maths::web_server::converter::web_objects::AttackPosition = maths::web_server::converter::web_objects::AttackPosition::FRONT;
//     let attacking: regiment::Regiment = initialize_units::initialize_imps();
//     let defending: regiment::Regiment = initialize_units::initialize_heavy_infantry();
//     let res: std::collections::HashMap<fight::ComputeCase, prediction::Prediction> =
//         fight::compute_turn(attacking_position, &attacking, &defending);
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         2
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         4
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         4
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         2
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         4
//     );
// }

// #[test]
// fn test_warriors_against_warriors() {
//     let attacking_position: maths::web_server::converter::web_objects::AttackPosition = maths::web_server::converter::web_objects::AttackPosition::FRONT;
//     let attacking: regiment::Regiment = initialize_units::initialize_warriors();
//     let defending: regiment::Regiment = initialize_units::initialize_warriors();
//     let res: std::collections::HashMap<fight::ComputeCase, prediction::Prediction> =
//         fight::compute_turn(attacking_position, &attacking, &defending);
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         6
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         9
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         6
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         5
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         5
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         8
//     );
// }

// #[test]
// fn test_imps_against_imps() {
//     let attacking_position: maths::web_server::converter::web_objects::AttackPosition = maths::web_server::converter::web_objects::AttackPosition::FRONT;
//     let attacking: regiment::Regiment = initialize_units::initialize_imps();
//     let defending: regiment::Regiment = initialize_units::initialize_imps();
//     let res: std::collections::HashMap<fight::ComputeCase, prediction::Prediction> =
//         fight::compute_turn(attacking_position, &attacking, &defending);
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         2
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         4
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         4
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         1
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
// }

// #[test]
// fn test_heavy_infantry_against_heavy_infantry() {
//     let attacking_position: maths::web_server::converter::web_objects::AttackPosition = maths::web_server::converter::web_objects::AttackPosition::FRONT;
//     let attacking: regiment::Regiment = initialize_units::initialize_heavy_infantry();
//     let defending: regiment::Regiment = initialize_units::initialize_heavy_infantry();
//     let res: std::collections::HashMap<fight::ComputeCase, prediction::Prediction> =
//         fight::compute_turn(attacking_position, &attacking, &defending);
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         5
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         4
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         2
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         4
//     );
// }

// #[test]
// fn test_silexian_spears_against_imps() {
//     let attacking_position: maths::web_server::converter::web_objects::AttackPosition = maths::web_server::converter::web_objects::AttackPosition::FRONT;
//     let attacking: regiment::Regiment = initialize_units::initialize_silexian_spears();
//     let defending: regiment::Regiment = initialize_units::initialize_imps();
//     let res: std::collections::HashMap<fight::ComputeCase, prediction::Prediction> =
//         fight::compute_turn(attacking_position, &attacking, &defending);
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         5
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         4
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         1
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
// }

// #[test]
// fn test_silexian_spears_against_warriors() {
//     let attacking_position: maths::web_server::converter::web_objects::AttackPosition = maths::web_server::converter::web_objects::AttackPosition::FRONT;
//     let attacking: regiment::Regiment = initialize_units::initialize_silexian_spears();
//     let defending: regiment::Regiment = initialize_units::initialize_warriors();
//     let res: std::collections::HashMap<fight::ComputeCase, prediction::Prediction> =
//         fight::compute_turn(attacking_position, &attacking, &defending);
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         2
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         4
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         4
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         6
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         5
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         10
//     );
// }

// #[test]
// fn test_clan_warriors_against_citizen_spears() {
//     let attacking_position: maths::web_server::converter::web_objects::AttackPosition = maths::web_server::converter::web_objects::AttackPosition::FRONT;
//     let attacking: regiment::Regiment = initialize_units::initialize_clan_warriors();
//     let defending: regiment::Regiment = initialize_units::initialize_citizen_spears();
//     let res: std::collections::HashMap<fight::ComputeCase, prediction::Prediction> =
//         fight::compute_turn(attacking_position, &attacking, &defending);
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         5
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         4
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         1
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
// }

// #[test]
// fn test_clan_warriors_against_silexian_spears() {
//     let attacking_position: maths::web_server::converter::web_objects::AttackPosition = maths::web_server::converter::web_objects::AttackPosition::FRONT;
//     let attacking: regiment::Regiment = initialize_units::initialize_clan_warriors();
//     let defending: regiment::Regiment = initialize_units::initialize_silexian_spears();
//     let res: std::collections::HashMap<fight::ComputeCase, prediction::Prediction> =
//         fight::compute_turn(attacking_position, &attacking, &defending);
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         5
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         4
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         1
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
// }

// #[test]
// fn test_infernal_warriors_against_clan_warriors() {
//     let attacking_position: maths::web_server::converter::web_objects::AttackPosition = maths::web_server::converter::web_objects::AttackPosition::FRONT;
//     let attacking: regiment::Regiment = initialize_units::initialize_infernal_warriors();
//     let defending: regiment::Regiment = initialize_units::initialize_clan_warriors();
//     let res: std::collections::HashMap<fight::ComputeCase, prediction::Prediction> =
//         fight::compute_turn(attacking_position, &attacking, &defending);
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         2
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         4
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         4
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         1
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
// }

// #[test]
// fn test_zombies_against_imps() {
//     let attacking_position: maths::web_server::converter::web_objects::AttackPosition = maths::web_server::converter::web_objects::AttackPosition::FRONT;
//     let attacking: regiment::Regiment = initialize_units::initialize_zombies();
//     let defending: regiment::Regiment = initialize_units::initialize_imps();
//     let res: std::collections::HashMap<fight::ComputeCase, prediction::Prediction> =
//         fight::compute_turn(attacking_position, &attacking, &defending);
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         6
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_attacking_regiment()
//             .get_points(),
//         5
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::MEAN)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         2
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::BEST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         3
//     );
//     assert_eq!(
//         res.get(&fight::ComputeCase::WORST)
//             .unwrap()
//             .get_defending_regiment()
//             .get_points(),
//         4
//     );
// }
