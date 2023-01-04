//! # Web_server make_prophecy_request module
//!
//! This module contains the data structure needed by the make_prophecy request

use crate::web_server::data_structures::regiment;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MakeProphecyRequest {
    key: String,
    attacking_regiment: regiment::Regiment,
    defending_regiment: regiment::Regiment,
}

impl MakeProphecyRequest {
    pub fn new(
        key: String,
        attacking_regiment: regiment::Regiment,
        defending_regiment: regiment::Regiment,
    ) -> Self {
        Self {
            key,
            attacking_regiment,
            defending_regiment,
        }
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_attacking_regiment(&self) -> &regiment::Regiment {
        &self.attacking_regiment
    }

    pub fn get_defending_regiment(&self) -> &regiment::Regiment {
        &self.defending_regiment
    }
}
