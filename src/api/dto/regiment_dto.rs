use crate::api::dto::model_dto::ModelDto;
use crate::regiment;

#[repr(C)]
#[derive(Debug)]
pub struct RegimentDto {
    pub model: ModelDto,
    pub nb_rows: usize,
    pub nb_cols: usize,
    pub nb_models: usize,
    pub regiment_health_point: usize,
    pub points: usize,
}

impl RegimentDto {
    pub fn dehydrate(regiment: regiment::Regiment) -> RegimentDto {
        RegimentDto {
            model: ModelDto::dehydrate(regiment.get_model().clone()),
            nb_rows: regiment.get_rows(),
            nb_cols: regiment.get_cols(),
            nb_models: regiment.get_nb_models(),
            regiment_health_point: regiment.get_regiment_health_points(),
            points: regiment.get_points(),
        }
    }

    pub fn hydrate(regiment_dto: &RegimentDto) -> regiment::Regiment {
        println!(
            "RegimentDto::hydrate {}, {}, {}, {}, {}",
            regiment_dto.nb_cols,
            regiment_dto.nb_rows,
            regiment_dto.nb_models,
            regiment_dto.regiment_health_point * regiment_dto.nb_models,
            regiment_dto.points
        );
        regiment::Regiment::new(
            ModelDto::hydrate(&regiment_dto.model),
            regiment_dto.nb_rows,
            regiment_dto.nb_cols,
            regiment_dto.nb_models,
            Option::from(regiment_dto.regiment_health_point * regiment_dto.nb_models),
        )
    }
}
