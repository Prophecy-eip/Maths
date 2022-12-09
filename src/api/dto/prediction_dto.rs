use crate::api::dto::regiment_dto::RegimentDto;
use crate::prediction;

#[repr(C)]
pub struct PredictionDto {
    pub attacking_regiment: RegimentDto,
    pub defending_regiment: RegimentDto,
    pub occurence_probability: f64,
}

impl PredictionDto {
    pub fn dehydrate(prediction: &prediction::Prediction) -> PredictionDto {
        println!("{:?}", prediction.get_attacking_regiment());
        PredictionDto {
            attacking_regiment: RegimentDto::dehydrate(prediction.get_attacking_regiment()),
            defending_regiment: RegimentDto::dehydrate(prediction.get_defending_regiment()),
            occurence_probability: prediction.get_probability(),
        }
    }
}
