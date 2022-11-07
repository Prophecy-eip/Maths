use crate::api::dto::raw_array::RawArray;
use crate::api::dto::requirements_dto::{RequirementsDto, RequirementsDtoMut};
use crate::api::dto::stats_dto::StatsDto;
use crate::model;

#[repr(C)]
#[derive(Clone)]
pub struct ModifierDto {
    stat: StatsDto,
    bonus: bool,
    nb_dice: usize,
    requirements: RawArray<RequirementsDto>,
}

impl ModifierDto {
    pub fn dehydrate(modifier: model::Modifier) -> ModifierDto {
        ModifierDto {
            stat: StatsDto::dehydrate(modifier.get_stat()),
            bonus: modifier.is_bonus(),
            nb_dice: modifier.get_nb_dice(),
            requirements: RawArray::dehydrate(
                modifier
                    .get_requirements()
                    .iter()
                    .map(|r| RequirementsDto::dehydrate(r.to_owned()))
                    .collect(),
            ),
        }
    }

    pub fn hydrate(modifier_dto: &ModifierDto) -> model::Modifier {
        model::Modifier::new(
            StatsDto::hydrate(&modifier_dto.stat),
            modifier_dto.bonus,
            modifier_dto.nb_dice,
            RawArray::hydrate(&modifier_dto.requirements)
                .iter()
                .map(RequirementsDto::hydrate)
                .collect(),
        )
    }
}
