//! # Web_server make_prophecy_response module
//!
//! This module contains all the data structures needed by the make_prophecy response

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegimentData {
    pub nb_rows: usize,
    pub nb_cols: usize,
    pub nb_models: usize,
    pub points: usize,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Prediction {
    pub attacking_regiment: RegimentData,
    pub defending_regiment: RegimentData,
    pub occurrence_probability: f64,
}

#[derive(serde::Serialize, serde::Deserialize)]
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
