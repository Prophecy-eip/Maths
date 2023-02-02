/// The structure containing the information for a regiment in the response of the web server
///
/// # Attributes
///
/// nb_rows (usize): Number of rows in the regiment's formation
///
/// nb_cols (usize): Number of columns in the regiment's formation
///
/// nb_models (usize): Number of models in the regiment's formation
///
/// points (usize): Number of points the regiment carries
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
    /// # Parameters
    ///
    /// nb_rows (usize): Number of rows in the regiment's formation
    ///
    /// nb_cols (usize): Number of columns in the regiment's formation
    ///
    /// nb_models (usize): Number of models in the regiment's formation
    ///
    /// points (usize): Number of points the regiment carries
    ///
    /// # Return
    ///
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

/// The structure containing the information for a prediction in the response of the web server
///
/// # Attributes
///
/// attacking_regiment (RegimentResponse): The attacking regiment
///
/// defending_regiment (RegimentResponse): The defending regiment
///
/// probability (f64): The probability of the prediction
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PredictionResponse {
    attacking_regiment: RegimentResponse,
    defending_regiment: RegimentResponse,
    probability: f64,
}

impl PredictionResponse {
    /// Create a new PredictionResponse
    ///
    /// # Parameters
    ///
    /// attacking_regiment (RegimentResponse): The attacking regiment
    ///
    /// defending_regiment (RegimentResponse): The defending regiment
    ///
    /// probability (f64): The probability of the prediction
    ///
    /// # Return
    ///
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

/// The structure containing the information for a prophecy in the response of the web server
/// Each prophecy contains 3 predictions: the best case, the worst case and the average case of the same fight
///
/// # Attributes
///
/// best_case (PredictionResponse): The best case prediction
///
/// worst_case (PredictionResponse): The worst case prediction
///
/// mean_case (PredictionResponse): The average case prediction
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProphecyResponse {
    best_case: PredictionResponse,
    worst_case: PredictionResponse,
    mean_case: PredictionResponse,
}

impl ProphecyResponse {
    /// Create a new ProphecyResponse
    ///
    /// # Parameters
    ///
    /// best_case (PredictionResponse): The best case prediction
    ///
    /// worst_case (PredictionResponse): The worst case prediction
    ///
    /// average_case (PredictionResponse): The average case prediction
    ///
    /// # Return
    ///
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

    /// Create a new ProphecyResponse from a FightPredictionResult
    ///
    /// # Parameters
    ///
    /// fight_prediction_result (fight::FightPredictionResult): The FightPredictionResult to convert
    ///
    /// # Return
    ///
    /// ProphecyResponse: The newly created ProphecyResponse
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
