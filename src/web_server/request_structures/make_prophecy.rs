//! # Web_server make_prophecy_request module
//!
//! This module contains the data structure needed by the make_prophecy request

use crate::web_server::data_structures::regiment;

/// The request sent to the server on the make_prophecy endpoint
///
/// ## Attributes
/// key (String): The private key used to authenticate the request
///
/// attacking_regiment (Regiment): The attacking regiment
///
/// defending_regiment (Regiment): The defending regiment
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MakeProphecyRequest {
    key: String,
    attacking_position: String,
    attacking_regiment: regiment::Regiment,
    defending_regiment: regiment::Regiment,
}

impl MakeProphecyRequest {
    /// Create a new MakeProphecyRequest
    ///
    /// ## Parameters
    /// (String) key: The private key used to authenticate the request
    ///
    /// (Regiment) attacking_regiment: The attacking regiment
    ///
    /// (Regiment) defending_regiment: The defending regiment
    ///
    /// ## Return
    /// MakeProphecyRequest: The newly created MakeProphecyRequest
    pub fn new(
        key: String,
        attacking_position: String,
        attacking_regiment: regiment::Regiment,
        defending_regiment: regiment::Regiment,
    ) -> Self {
        Self {
            key,
            attacking_position,
            attacking_regiment,
            defending_regiment,
        }
    }

    /// Get the private key used to authenticate the request
    ///
    /// ## Return
    /// String: The private key used to authenticate the request
    pub fn get_key(&self) -> &String {
        &self.key
    }

    /// Get the attacking position
    ///
    /// ## Return
    /// &String: The attacking position
    pub fn get_attacking_position(&self) -> String {
        self.attacking_position.clone()
    }

    /// Get the attacking regiment
    ///
    /// ## Return
    /// Regiment: The attacking regiment
    pub fn get_attacking_regiment(&self) -> &regiment::Regiment {
        &self.attacking_regiment
    }

    /// Get the defending regiment
    ///
    /// ## Return
    /// Regiment: The defending regiment
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
                banner_bearer: false,
            },
        }
    }

    fn create_prophecy_request() -> MakeProphecyRequest {
        let key: String = std::env::var("PRIVATE_KEY").unwrap_or_else(|_| "".to_string());
        let attacking_position: String = String::from("front");
        let attacking_regiment: regiment::Regiment = create_dummy_regiment(1);
        let defending_regiment: regiment::Regiment = create_dummy_regiment(2);
        MakeProphecyRequest::new(
            key,
            attacking_position,
            attacking_regiment,
            defending_regiment,
        )
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
