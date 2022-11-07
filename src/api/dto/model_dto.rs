use crate::api::dto::modifiers_dto::ModifierDto;
use crate::api::dto::raw_array::RawArray;
use crate::api::dto::stats_dto::StatsDto;
use crate::model;

#[repr(C)]
#[derive(Clone)]
pub struct ModelDto {
    stats: StatsDto,
    modifiers: RawArray<ModifierDto>,
}

impl ModelDto {
    pub fn dehydrate(model: model::Model) -> ModelDto {
        ModelDto {
            stats: StatsDto::dehydrate(model.get_stats()),
            modifiers: RawArray::dehydrate(
                model
                    .get_modifiers()
                    .iter()
                    .map(|m| ModifierDto::dehydrate(m.to_owned()))
                    .collect(),
            ),
        }
    }

    pub fn hydrate(model_dto: &ModelDto) -> model::Model {
        model::Model::new(
            StatsDto::hydrate(&model_dto.stats),
            RawArray::hydrate(&model_dto.modifiers)
                .iter()
                .map(ModifierDto::hydrate)
                .collect(),
        )
    }
}
