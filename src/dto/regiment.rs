#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegimentDto {
    model: super::model::ModelDto,
    nb_rows: usize,
    nb_cols: usize,
    nb_models: usize,
}

impl RegimentDto {
    pub fn new(
        model: super::model::ModelDto,
        nb_rows: usize,
        nb_cols: usize,
        nb_models: usize,
    ) -> RegimentDto {
        RegimentDto {
            model,
            nb_rows,
            nb_cols,
            nb_models,
        }
    }

    pub fn hydrate(&self) -> crate::regiment::Regiment {
        crate::regiment::Regiment::new(
            self.model.hydrate(),
            self.nb_rows,
            self.nb_cols,
            self.nb_models,
            None,
        )
    }

    pub fn dehydrate(regiment: &crate::regiment::Regiment) -> RegimentDto {
        RegimentDto {
            model: super::model::ModelDto::dehydrate(regiment.get_model()),
            nb_rows: regiment.get_rows(),
            nb_cols: regiment.get_cols(),
            nb_models: regiment.get_nb_models(),
        }
    }
}
