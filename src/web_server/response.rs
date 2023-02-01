#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegimentResponse {
    nb_rows: usize,
    nb_cols: usize,
    nb_models: usize,
    points: usize,
}

impl RegimentResponse {
    /// Create a new RegimentResponse
    ///
    /// ## Parameters
    /// (usize) nb_rows: Number of rows in the regiment's formation
    ///
    /// (usize) nb_cols: Number of columns in the regiment's formation
    ///
    /// (usize) nb_models: Number of models in the regiment's formation
    ///
    /// (usize) points: Number of points the regiment carries
    ///
    /// ## Return
    /// RegimentResponse: The newly created RegimentResponse
    pub fn new(nb_rows: usize, nb_cols: usize, nb_models: usize, points: usize) -> Self {
        Self {
            nb_rows,
            nb_cols,
            nb_models,
            points,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PredictionResponse {
    attacking_regiment: RegimentResponse,
    defending_regiment: RegimentResponse,
    probability: f64,
}

impl PredictionResponse {
    /// Create a new PredictionResponse
    ///
    /// ## Parameters
    /// (RegimentResponse) attacking_regiment: The attacking regiment
    ///
    /// (RegimentResponse) defending_regiment: The defending regiment
    ///
    /// (f64) probability: The probability of the prediction
    ///
    /// ## Return
    /// PredictionResponse: The newly created PredictionResponse
    pub fn new(
        attacking_regiment: RegimentResponse,
        defending_regiment: RegimentResponse,
        probability: f64,
    ) -> Self {
        Self {
            attacking_regiment,
            defending_regiment,
            probability,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProphecyResponse {
    best_case: PredictionResponse,
    worst_case: PredictionResponse,
    mean_case: PredictionResponse,
}

impl ProphecyResponse {
    /// Create a new ProphecyResponse
    ///
    /// ## Parameters
    /// (PredictionResponse) best_case: The best case prediction
    ///
    /// (PredictionResponse) worst_case: The worst case prediction
    ///
    /// (PredictionResponse) average_case: The average case prediction
    ///
    /// ## Return
    /// ProphecyResponse: The newly created ProphecyResponse
    pub fn new(
        best_case: PredictionResponse,
        worst_case: PredictionResponse,
        mean_case: PredictionResponse,
    ) -> Self {
        Self {
            best_case,
            worst_case,
            mean_case,
        }
    }

    pub fn from_fight_prediction_result(
        fight_prediction_result: maths::fight::FightPredictionResult,
    ) -> ProphecyResponse {
        ProphecyResponse {
            best_case: PredictionResponse::new(
                RegimentResponse::new(
                    fight_prediction_result
                        .get_best_case()
                        .get_attacking_regiment()
                        .get_rows(),
                    fight_prediction_result
                        .get_best_case()
                        .get_attacking_regiment()
                        .get_cols(),
                    fight_prediction_result
                        .get_best_case()
                        .get_attacking_regiment()
                        .get_nb_models(),
                    fight_prediction_result
                        .get_best_case()
                        .get_attacking_regiment()
                        .get_points(),
                ),
                RegimentResponse::new(
                    fight_prediction_result
                        .get_best_case()
                        .get_defending_regiment()
                        .get_rows(),
                    fight_prediction_result
                        .get_best_case()
                        .get_defending_regiment()
                        .get_cols(),
                    fight_prediction_result
                        .get_best_case()
                        .get_defending_regiment()
                        .get_nb_models(),
                    fight_prediction_result
                        .get_best_case()
                        .get_defending_regiment()
                        .get_points(),
                ),
                fight_prediction_result.get_best_case().get_probability(),
            ),
            worst_case: PredictionResponse::new(
                RegimentResponse::new(
                    fight_prediction_result
                        .get_worst_case()
                        .get_attacking_regiment()
                        .get_rows(),
                    fight_prediction_result
                        .get_worst_case()
                        .get_attacking_regiment()
                        .get_cols(),
                    fight_prediction_result
                        .get_worst_case()
                        .get_attacking_regiment()
                        .get_nb_models(),
                    fight_prediction_result
                        .get_worst_case()
                        .get_attacking_regiment()
                        .get_points(),
                ),
                RegimentResponse::new(
                    fight_prediction_result
                        .get_worst_case()
                        .get_defending_regiment()
                        .get_rows(),
                    fight_prediction_result
                        .get_worst_case()
                        .get_defending_regiment()
                        .get_cols(),
                    fight_prediction_result
                        .get_worst_case()
                        .get_defending_regiment()
                        .get_nb_models(),
                    fight_prediction_result
                        .get_worst_case()
                        .get_defending_regiment()
                        .get_points(),
                ),
                fight_prediction_result.get_worst_case().get_probability(),
            ),
            mean_case: PredictionResponse::new(
                RegimentResponse::new(
                    fight_prediction_result
                        .get_mean_case()
                        .get_attacking_regiment()
                        .get_rows(),
                    fight_prediction_result
                        .get_mean_case()
                        .get_attacking_regiment()
                        .get_cols(),
                    fight_prediction_result
                        .get_mean_case()
                        .get_attacking_regiment()
                        .get_nb_models(),
                    fight_prediction_result
                        .get_mean_case()
                        .get_attacking_regiment()
                        .get_points(),
                ),
                RegimentResponse::new(
                    fight_prediction_result
                        .get_mean_case()
                        .get_defending_regiment()
                        .get_rows(),
                    fight_prediction_result
                        .get_mean_case()
                        .get_defending_regiment()
                        .get_cols(),
                    fight_prediction_result
                        .get_mean_case()
                        .get_defending_regiment()
                        .get_nb_models(),
                    fight_prediction_result
                        .get_mean_case()
                        .get_defending_regiment()
                        .get_points(),
                ),
                fight_prediction_result.get_mean_case().get_probability(),
            ),
        }
    }
}
