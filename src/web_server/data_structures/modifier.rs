//! # Web_server Modifier module
//!
//! This module contains all the structs and datas needed by a modifier in the web server

/// Struct containing the modification granted to the global stats of a Model
///
/// ## Attributes
/// advance (isize): The advance stat boost
///
/// march (isize): The march stat boost
///
/// discipline (isize): The discipline stat boost
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub struct GlobalModifier {
    pub advance: isize,
    pub march: isize,
    pub discipline: isize,
}

/// Struct containing the modification granted to the offensive stats of a Model
///
/// ## Attributes
/// attack (isize): The attack stat boost
///
/// offensive (isize): The offensive stat boost
///
/// strength (isize): The strength stat boost
///
/// armour_penetration (isize): The armour penetration stat boost
///
/// agility (isize): The agility stat boost
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub struct OffensiveModifier {
    pub attack: isize,
    pub offensive: isize,
    pub strength: isize,
    pub armour_penetration: isize,
    pub agility: isize,
}

/// Struct containing the modification granted to the defensive stats of a Model
///
/// ## Attributes
/// health_points (isize): The health points stat boost
///
/// defense (isize): The defense stat boost
///
/// resilience (isize): The resilience stat boost
///
/// armour (isize): The armour stat boost
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub struct DefensiveModifier {
    pub health_points: isize,
    pub defense: isize,
    pub resilience: isize,
    pub armour: isize,
}

/// Struct containing the modification granted by a weapon
///
/// ## Attributes
///
/// shots (Option<isize>): The number of shots if it is a ranged weapon and None if close range weapon
///
/// strength (isize): The strength stat boost
///
/// armour_penetration (isize): The armour penetration stat boost
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub struct WeaponModifier {
    pub shots: Option<isize>,
    pub strength: isize,
    pub armour_penetration: isize,
}

/// Enum containing all the possible modifiers
///
/// ## Variants
/// GlobalModifier: The global modifier
///
/// OffensiveModifier: The offensive modifier
///
/// DefensiveModifier: The defensive modifier
///
/// MeleeWeaponModifier: The melee weapon modifier
///
/// WeaponModifier: The ranged weapon modifier
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Modifier {
    Global(GlobalModifier),
    Offensive(OffensiveModifier),
    Defensive(DefensiveModifier),
    Weapon(WeaponModifier),
}
