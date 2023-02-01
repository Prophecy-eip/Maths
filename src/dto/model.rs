#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModelDto {
    stats: super::stat::StatsDto,
    modifiers: Vec<super::modifier::ModifierDto>,
    banner_bearer: bool,
}

impl ModelDto {
    pub fn new(
        stats: super::stat::StatsDto,
        modifiers: Vec<super::modifier::ModifierDto>,
        banner_bearer: bool,
    ) -> ModelDto {
        ModelDto {
            stats,
            modifiers,
            banner_bearer,
        }
    }

    pub fn hydrate(&self) -> crate::model::Model {
        crate::model::Model::new(
            self.stats.hydrate(),
            self.modifiers
                .iter()
                .map(|modifier| modifier.hydrate())
                .collect(),
            self.banner_bearer,
        )
    }

    pub fn dehydrate(model: &crate::model::Model) -> ModelDto {
        ModelDto {
            stats: super::stat::StatsDto::dehydrate(model.get_stats()),
            modifiers: model
                .get_modifiers()
                .iter()
                .map(super::modifier::ModifierDto::dehydrate)
                .collect(),
            banner_bearer: model.is_banner_bearer(),
        }
    }
}
