//! Web server module
//!
//! This module contains the web server implementation and the response structures.

pub mod response;

///ProphecyRequest
///
/// # Attributes
///
/// key (String): The key to access the web server
///
/// attacking_position (String): The attacking position
///
/// attacking_regiment (RegimentDto): The attacking regiment
///
/// defending_regiment (RegimentDto): The defending regiment
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProphecyRequest {
    key: String,
    attacking_position: String,
    attacking_regiment: maths::dto::regiment::RegimentDto,
    defending_regiment: maths::dto::regiment::RegimentDto,
}

impl ProphecyRequest {
    /// Creates a new ProphecyRequest
    ///
    /// # Arguments
    ///
    /// key (String): The key to access the web server
    ///
    /// attacking_position (String): The attacking position
    ///
    /// attacking_regiment (RegimentDto): The attacking regiment
    ///
    /// defending_regiment (RegimentDto): The defending regiment
    ///
    /// # Return
    ///
    /// ProphecyRequest: The new ProphecyRequest
    pub fn new(
        key: String,
        attacking_position: String,
        attacking_regiment: maths::dto::regiment::RegimentDto,
        defending_regiment: maths::dto::regiment::RegimentDto,
    ) -> Self {
        Self {
            key,
            attacking_position,
            attacking_regiment,
            defending_regiment,
        }
    }

    /// Get the key
    ///
    /// # Return
    ///
    /// String: The key
    pub fn get_key(&self) -> &String {
        &self.key
    }

    /// Get the attacking position
    ///
    /// # Return
    ///
    /// String: The attacking position
    pub fn get_attacking_position(&self) -> &String {
        &self.attacking_position
    }

    /// Get the attacking regiment
    ///
    /// # Return
    ///
    /// RegimentDto: The attacking regiment
    pub fn get_attacking_regiment(&self) -> &maths::dto::regiment::RegimentDto {
        &self.attacking_regiment
    }

    /// Get the defending regiment
    ///
    /// # Return
    ///
    /// RegimentDto: The defending regiment
    pub fn get_defending_regiment(&self) -> &maths::dto::regiment::RegimentDto {
        &self.defending_regiment
    }

    /// Convert the attacking position received to an AttackPosition usable in the library
    ///
    /// # Return
    ///
    /// AttackPosition: The converted attacking position
    pub fn convert_attacking_position(&self) -> maths::fight::AttackPosition {
        match self.attacking_position.as_str() {
            "front" => maths::fight::AttackPosition::FRONT,
            "flank" => maths::fight::AttackPosition::FLANK,
            "back" => maths::fight::AttackPosition::BACK,
            _ => maths::fight::AttackPosition::FRONT,
        }
    }

    /// Convert the attacking or defending regiment received to a Regiment usable in the library
    ///
    /// # Arguments
    ///
    /// attacking (bool): If the attacking regiment should be converted or the defending regiment
    ///
    /// # Return
    ///
    /// regiment::Regiment: The converted attacking or defending regiment
    pub fn convert_regiment(&self, attacking: bool) -> maths::regiment::Regiment {
        match attacking {
            true => self.attacking_regiment.hydrate(),
            false => self.defending_regiment.hydrate(),
        }
    }
}
