use crate::fight::compute_turn;

use self::dto::{prediction_dto::PredictionDto, regiment_dto::RegimentDto};

pub mod dto;

#[repr(C)]
pub struct Result {
    best_prediction: PredictionDto,
    worst_prediction: PredictionDto,
    mean_prediction: PredictionDto,
}

#[no_mangle]
pub extern "C" fn compute_fight(
    attacking_regiment: RegimentDto,
    defending_regiment: RegimentDto,
) -> Result {
    let attacking_regiment = RegimentDto::hydrate(&attacking_regiment);
    let defending_regiment = RegimentDto::hydrate(&defending_regiment);
    let result = compute_turn(&attacking_regiment, &defending_regiment);

    Result {
        best_prediction: PredictionDto::dehydrate(
            result.get(&crate::fight::ComputeCase::BEST).unwrap(),
        ),
        worst_prediction: PredictionDto::dehydrate(
            result.get(&crate::fight::ComputeCase::WORST).unwrap(),
        ),
        mean_prediction: PredictionDto::dehydrate(
            result.get(&crate::fight::ComputeCase::MEAN).unwrap(),
        ),
    }
}
