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

///ProphecyUnit
///
/// # Attributes
///
/// name (String): The unit name
///
/// modifiers (Vec<String>): The units modifiers
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ProphecyUnit {
    name: String,
    modifiers: Vec<String>,
}

impl ProphecyUnit {
    /// Create a new prophecy unit object
    ///
    /// # Arguments
    ///
    /// name (String): String of the name of the unit
    ///
    /// Modifiers (Vec<String>) : list of String for the modifiers linked to the unit
    ///
    /// # Return
    ///
    /// ProphecyUnit: a new prophecy unit object
    pub fn new(name: String, modifiers: Vec<String>) -> Self {
        Self { name, modifiers }
    }

    /// Get the name of the unit
    ///
    /// # Return
    ///
    /// String: The unit name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Get a vector of modifiers linked to the unit
    ///
    /// # Return
    ///
    /// Vec<String>: The list of modifiers
    pub fn get_modifiers(&self) -> &Vec<String> {
        &self.modifiers
    }
}

///ProphecyRequestArmies
///
/// # Attributes
///
/// key (String): The key to access the web server
///
/// first_player (Vec<ProphecyUnit>): The attacking player units
///
/// second_player (Vec<ProphecyUnit>): The attacking player units
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProphecyRequestArmies {
    key: String,
    first_player_army: Vec<ProphecyUnit>,
    second_player_army: Vec<ProphecyUnit>,
}

impl ProphecyRequestArmies {
    /// Creates a new ProphecyRequest
    ///
    /// # Arguments
    ///
    /// key (String): The key to access the web server
    ///
    /// first player (Vec<ProphecyUnit>): a list of attacking prophecy units
    ///
    /// second player (Vec<ProphecyUnit>): a list of defending prophecy units
    ///
    /// # Return
    ///
    /// ProphecyRequestArmies : The new ProphecyRequestArmies
    pub fn new(
        key: String,
        first_player_army: Vec<ProphecyUnit>,
        second_player_army: Vec<ProphecyUnit>,
    ) -> Self {
        Self {
            key,
            first_player_army,
            second_player_army,
        }
    }

    /// Get the key for the request
    ///
    /// # Return
    ///
    /// &String: The key name
    pub fn get_key(&self) -> &String {
        &self.key
    }

    /// Get the key for the request
    ///
    /// # Return
    ///
    /// &Vec<ProphecyUnit>: A reference the army list of the first player
    pub fn get_first_player_army(&self) -> &Vec<ProphecyUnit> {
        &self.first_player_army
    }

    /// Get the key for the request
    ///
    /// # Return
    ///
    /// &Vec<ProphecyUnit>: A reference the army list of the second player
    pub fn get_second_player_army(&self) -> &Vec<ProphecyUnit> {
        &self.second_player_army
    }

    /// Add a unit to the first player army.
    ///
    /// # Parameters
    ///
    /// `unit` (ProphecyUnit): The unit to be added to the first player army.
    pub fn add_unit_to_first_player_army(&mut self, unit: ProphecyUnit) {
        self.first_player_army.push(unit);
    }

    /// Add a unit to the second player army.
    ///
    /// # Parameters
    ///
    /// `unit` (ProphecyUnit): The unit to be added to the second player army.
    pub fn add_unit_to_second_player_army(&mut self, unit: ProphecyUnit) {
        self.second_player_army.push(unit);
    }
}
