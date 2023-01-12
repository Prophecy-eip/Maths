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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web_server::data_structures::model;
    use crate::web_server::data_structures::regiment;

    fn create_dummy_regiment(value: usize) -> regiment::Regiment {
        regiment::Regiment {
            nb_rows: 1,
            nb_cols: 1,
            nb_models: 1,
            model: model::Model {
                modifiers: vec![],
                stats: model::Stats {
                    advance: value,
                    march: value,
                    discipline: value,
                    health_points: value,
                    defense: value,
                    resilience: value,
                    armour: value,
                    aegis: value,
                    attack: value,
                    offensive: value,
                    strength: value,
                    armour_penetration: value,
                    agility: value,
                },
            },
        }
    }

    fn create_prophecy_request() -> MakeProphecyRequest {
        let key: String = std::env::var("PRIVATE_KEY").unwrap_or_else(|_| "".to_string());
        let attacking_regiment: regiment::Regiment = create_dummy_regiment(1);
        let defending_regiment: regiment::Regiment = create_dummy_regiment(2);
        MakeProphecyRequest::new(key, attacking_regiment, defending_regiment)
    }

    #[test]
    fn test_make_prophecy_request_new() {
        let make_prophecy_request: MakeProphecyRequest = create_prophecy_request();

        assert_eq!(
            make_prophecy_request.get_key(),
            &std::env::var("PRIVATE_KEY").unwrap_or_else(|_| "".to_string())
        );
        assert_eq!(
            make_prophecy_request.get_attacking_regiment(),
            &create_dummy_regiment(1)
        );
        assert_eq!(
            make_prophecy_request.get_defending_regiment(),
            &create_dummy_regiment(2)
        );
    }

    #[test]
    fn test_make_prophecy_request_get_key() {
        let make_prophecy_request: MakeProphecyRequest = create_prophecy_request();

        assert_eq!(
            make_prophecy_request.get_key(),
            &std::env::var("PRIVATE_KEY").unwrap_or_else(|_| "".to_string())
        )
    }

    #[test]
    fn test_make_prophecy_request_get_attacking_regiment() {
        let make_prophecy_request: MakeProphecyRequest = create_prophecy_request();

        assert_eq!(
            make_prophecy_request.get_attacking_regiment(),
            &create_dummy_regiment(1)
        )
    }

    #[test]
    fn test_make_prophecy_request_get_defending_regiment() {
        let make_prophecy_request: MakeProphecyRequest = create_prophecy_request();

        assert_eq!(
            make_prophecy_request.get_defending_regiment(),
            &create_dummy_regiment(2)
        )
    }
}
