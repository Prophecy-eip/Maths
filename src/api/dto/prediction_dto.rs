use crate::api::dto::regiment_dto::RegimentDto;
use crate::prediction;

#[repr(C)]
pub struct PredictionDto {
    attacking_regiment: RegimentDto,
    defending_regiment: RegimentDto,
    occurence_probability: f64,
}

impl PredictionDto {
    pub fn dehydrate(prediction: &prediction::Prediction) -> PredictionDto {
        PredictionDto {
            attacking_regiment: RegimentDto::dehydrate(prediction.get_attacking_regiment()),
            defending_regiment: RegimentDto::dehydrate(prediction.get_defending_regiment()),
            occurence_probability: prediction.get_probability(),
        }
    }
}
