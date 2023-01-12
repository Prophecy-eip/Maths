//! # Web_server make_prophecy_response module
//!
//! This module contains all the data structures needed by the make_prophecy response

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct RegimentData {
    pub nb_rows: usize,
    pub nb_cols: usize,
    pub nb_models: usize,
    pub points: usize,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct Prediction {
    pub attacking_regiment: RegimentData,
    pub defending_regiment: RegimentData,
    pub occurrence_probability: f64,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct MakeProphecyResponse {
    best_case: Prediction,
    worst_case: Prediction,
    average_case: Prediction,
}

impl RegimentData {
    pub fn new(nb_rows: usize, nb_cols: usize, nb_models: usize, points: usize) -> Self {
        Self {
            nb_rows,
            nb_cols,
            nb_models,
            points,
        }
    }
}

impl Prediction {
    pub fn new(
        attacking_regiment: RegimentData,
        defending_regiment: RegimentData,
        occurrence_probability: f64,
    ) -> Self {
        Self {
            attacking_regiment,
            defending_regiment,
            occurrence_probability,
        }
    }
}

impl MakeProphecyResponse {
    pub fn new(best_case: Prediction, worst_case: Prediction, average_case: Prediction) -> Self {
        Self {
            best_case,
            worst_case,
            average_case,
        }
    }

    pub fn from_dict(
        prophecies: std::collections::HashMap<
            crate::fight::ComputeCase,
            crate::prediction::Prediction,
        >,
    ) -> Self {
        let mut best_case: Option<Prediction> = None;
        let mut worst_case: Option<Prediction> = None;
        let mut average_case: Option<Prediction> = None;

        for (compute_case, prediction) in prophecies {
            let prediction: Prediction = Prediction::new(
                RegimentData::new(
                    prediction.get_attacking_regiment().get_rows(),
                    prediction.get_attacking_regiment().get_cols(),
                    prediction.get_attacking_regiment().get_nb_models(),
                    prediction.get_attacking_regiment().get_points(),
                ),
                RegimentData::new(
                    prediction.get_defending_regiment().get_rows(),
                    prediction.get_defending_regiment().get_cols(),
                    prediction.get_defending_regiment().get_nb_models(),
                    prediction.get_defending_regiment().get_points(),
                ),
                prediction.get_probability(),
            );

            match compute_case {
                crate::fight::ComputeCase::BEST => {
                    best_case = Some(prediction);
                }
                crate::fight::ComputeCase::WORST => {
                    worst_case = Some(prediction);
                }
                crate::fight::ComputeCase::MEAN => {
                    average_case = Some(prediction);
                }
            }
        }

        Self::new(
            best_case.unwrap(),
            worst_case.unwrap(),
            average_case.unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regiment_data_new() {
        let regiment_data: RegimentData = RegimentData::new(1, 2, 3, 4);
        assert_eq!(regiment_data.nb_rows, 1);
        assert_eq!(regiment_data.nb_cols, 2);
        assert_eq!(regiment_data.nb_models, 3);
        assert_eq!(regiment_data.points, 4);
    }

    #[test]
    fn test_prediction_new() {
        let prediction = Prediction::new(
            RegimentData::new(1, 2, 3, 4),
            RegimentData::new(5, 6, 7, 8),
            0.5,
        );
        assert_eq!(prediction.attacking_regiment.nb_rows, 1);
        assert_eq!(prediction.attacking_regiment.nb_cols, 2);
        assert_eq!(prediction.attacking_regiment.nb_models, 3);
        assert_eq!(prediction.attacking_regiment.points, 4);
        assert_eq!(prediction.defending_regiment.nb_rows, 5);
        assert_eq!(prediction.defending_regiment.nb_cols, 6);
        assert_eq!(prediction.defending_regiment.nb_models, 7);
        assert_eq!(prediction.defending_regiment.points, 8);
        assert_eq!(prediction.occurrence_probability, 0.5);
    }

    #[test]
    fn test_make_prophecy_response_new() {
        let best_prediction: Prediction = Prediction::new(
            RegimentData::new(1, 2, 3, 4),
            RegimentData::new(5, 6, 7, 8),
            0.5,
        );
        let worst_prediction: Prediction = Prediction::new(
            RegimentData::new(9, 10, 11, 12),
            RegimentData::new(13, 14, 15, 16),
            0.6,
        );
        let average_prediction: Prediction = Prediction::new(
            RegimentData::new(17, 18, 19, 20),
            RegimentData::new(21, 22, 23, 24),
            0.7,
        );
        let copies: (Prediction, Prediction, Prediction) = (
            best_prediction.clone(),
            worst_prediction.clone(),
            average_prediction.clone(),
        );
        let make_prophecy_response =
            MakeProphecyResponse::new(best_prediction, worst_prediction, average_prediction);
        assert_eq!(make_prophecy_response.best_case == copies.0, true);
        assert_eq!(make_prophecy_response.worst_case == copies.1, true);
        assert_eq!(make_prophecy_response.average_case == copies.2, true);
    }

    #[test]
    fn test_make_prophecy_response_from_dict() {
        let mut prophecies: std::collections::HashMap<
            crate::fight::ComputeCase,
            crate::prediction::Prediction,
        > = std::collections::HashMap::new();
        prophecies.insert(
            crate::fight::ComputeCase::BEST,
            crate::prediction::Prediction::new(
                crate::global_test::tests::initialize_mock_regiment(),
                crate::global_test::tests::initialize_mock_regiment(),
                0.5,
            ),
        );
        prophecies.insert(
            crate::fight::ComputeCase::WORST,
            crate::prediction::Prediction::new(
                crate::global_test::tests::initialize_mock_regiment(),
                crate::global_test::tests::initialize_mock_regiment(),
                0.6,
            ),
        );
        prophecies.insert(
            crate::fight::ComputeCase::MEAN,
            crate::prediction::Prediction::new(
                crate::global_test::tests::initialize_mock_regiment(),
                crate::global_test::tests::initialize_mock_regiment(),
                0.7,
            ),
        );

        let make_prophecy_response = MakeProphecyResponse::from_dict(prophecies);
        assert_eq!(make_prophecy_response.best_case.occurrence_probability, 0.5);
        assert_eq!(
            make_prophecy_response.worst_case.occurrence_probability,
            0.6
        );
        assert_eq!(
            make_prophecy_response.average_case.occurrence_probability,
            0.7
        );
    }
}
