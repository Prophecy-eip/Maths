//! Modifier module
//!
//! This module contain all the structs and datas needed by a Modifier
//! A Modifier is a boost or a malus applied on a Model
//! It can be applied on the global stats or the offensive stats or the defensive stats
//! It can be applied on a specific stat or on all the stats of a category

mod modifiers_listing;

/// Struct containing the modification granted to the global stats of a Model
///
/// # Attributes
///
/// advance (isize): The advance stat boost
///
/// march (isize): The march stat boost
///
/// discipline (isize): The discipline stat boost
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize, Copy)]
pub struct GlobalModifier {
    advance: isize,
    march: isize,
    discipline: isize,
}

impl GlobalModifier {
    /// Constructor for the GlobalModifier struct
    ///
    /// # Parameters
    ///
    /// advance (isize): The advance stat boost
    ///
    /// march (isize): The march stat boost
    ///
    /// discipline (isize): The discipline stat boost
    ///
    /// # Return
    ///
    /// GlobalModifier: The GlobalModifier struct
    pub fn new(advance: isize, march: isize, discipline: isize) -> Self {
        GlobalModifier {
            advance,
            march,
            discipline,
        }
    }

    /// Getter for the advance stat boost
    ///
    /// # Return
    ///
    /// isize: The advance stat boost
    pub fn get_advance(&self) -> isize {
        self.advance
    }

    /// Getter for the march stat boost
    ///
    /// # Return
    ///
    /// isize: The march stat boost
    pub fn get_march(&self) -> isize {
        self.march
    }

    /// Getter for the discipline stat boost
    ///
    /// # Return
    ///
    /// isize: The discipline stat boost
    pub fn get_discipline(&self) -> isize {
        self.discipline
    }
}

/// Struct containing the modification granted to the offensive stats of a Model
///
/// # Attributes
///
/// attack (isize): The attack stat boost
///
/// offensive (isize): The offensive stat boost
///
/// strength (isize): The strength stat boost
///
/// armour_penetration (isize): The armour penetration stat boost
///
/// agility (isize): The agility stat boost
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize, Copy)]
pub struct OffensiveModifier {
    attack: isize,
    offensive: isize,
    strength: isize,
    armour_penetration: isize,
    agility: isize,
}

impl OffensiveModifier {
    /// Constructor for the OffensiveModifier struct
    ///
    /// # Parameters
    /// attack (isize): The attack stat boost
    ///
    /// offensive (isize): The offensive stat boost
    ///
    /// strength (isize): The strength stat boost
    ///
    /// armour_penetration (isize): The armour penetration stat boost
    ///
    /// agility (isize): The agility stat boost
    ///
    /// # Return
    ///
    /// OffensiveModifier: The OffensiveModifier struct
    pub fn new(
        attack: isize,
        offensive: isize,
        strength: isize,
        armour_penetration: isize,
        agility: isize,
    ) -> Self {
        OffensiveModifier {
            attack,
            offensive,
            strength,
            armour_penetration,
            agility,
        }
    }

    /// Getter for the attack stat boost
    ///
    /// # Return
    ///
    /// isize: The attack stat boost
    pub fn get_attack(&self) -> isize {
        self.attack
    }

    /// Getter for the offensive stat boost
    ///
    /// # Return
    ///
    /// isize: The offensive stat boost
    pub fn get_offensive(&self) -> isize {
        self.offensive
    }

    /// Getter for the strength stat boost
    ///
    /// # Return
    ///
    /// isize: The strength stat boost
    pub fn get_strength(&self) -> isize {
        self.strength
    }

    /// Getter for the armour penetration stat boost
    ///
    /// # Return
    ///
    /// isize: The armour penetration stat boost
    pub fn get_armour_penetration(&self) -> isize {
        self.armour_penetration
    }

    /// Getter for the agility stat boost
    ///
    /// # Return
    ///
    /// isize: The agility stat boost
    pub fn get_agility(&self) -> isize {
        self.agility
    }
}

/// Struct containing the modification granted to the defensive stats of a Model
///
/// # Attributes
///
/// health_points (isize): The health points stat boost
///
/// defense (isize): The defense stat boost
///
/// resilience (isize): The resilience stat boost
///
/// armour (isize): The armour stat boost
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize, Copy)]
pub struct DefensiveModifier {
    health_points: isize,
    defense: isize,
    resilience: isize,
    armour: isize,
    aegis: isize,
}

impl DefensiveModifier {
    /// Constructor for the DefensiveModifier struct
    ///
    /// # Parameters
    /// health_points (isize): The health points stat boost
    ///
    /// defense (isize): The defense stat boost
    ///
    /// resilience (isize): The resilience stat boost
    ///
    /// armour (isize): The armour stat boost
    ///
    /// # Return
    ///
    /// DefensiveModifier: The DefensiveModifier struct
    pub fn new(
        health_points: isize,
        defense: isize,
        resilience: isize,
        armour: isize,
        aegis: isize,
    ) -> Self {
        DefensiveModifier {
            health_points,
            defense,
            resilience,
            armour,
            aegis,
        }
    }

    /// Getter for the health points stat boost
    ///
    /// # Return
    ///
    /// isize: The health points stat boost
    pub fn get_health_points(&self) -> isize {
        self.health_points
    }

    /// Getter for the defense stat boost
    ///
    /// # Return
    ///
    /// isize: The defense stat boost
    pub fn get_defense(&self) -> isize {
        self.defense
    }

    /// Getter for the resilience stat boost
    ///
    /// # Return
    ///
    /// isize: The resilience stat boost
    pub fn get_resilience(&self) -> isize {
        self.resilience
    }

    /// Getter for the armour stat boost
    ///
    /// # Return
    ///
    /// isize: The armour stat boost
    pub fn get_armour(&self) -> isize {
        self.armour
    }

    /// Getter for the aegis stat boost
    ///
    /// # Return
    ///
    /// isize: The aegis stat boost
    pub fn get_aegis(&self) -> isize {
        self.aegis
    }
}

/// Struct containing the modification granted by a weapon
///
/// # Attributes
///
/// shots (Option<isize>): The number of shots if it is a ranged weapon and None if close range weapon
///
/// strength (isize): The strength stat boost
///
/// armour_penetration (isize): The armour penetration stat boost
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize, Copy)]
pub struct WeaponModifier {
    aim: Option<isize>,
    shots: Option<isize>,
    strength: isize,
    armour_penetration: isize,
}

impl WeaponModifier {
    /// Constructor for the WeaponModifier struct
    ///
    /// # Parameters
    /// shots (isize): The shots stat boost
    ///
    /// strength (isize): The strength stat boost
    ///
    /// armour_penetration (isize): The armour penetration stat boost
    ///
    /// # Return
    ///
    /// WeaponModifier: The WeaponModifier struct
    pub fn new(
        aim: Option<isize>,
        shots: Option<isize>,
        strength: isize,
        armour_penetration: isize,
    ) -> WeaponModifier {
        WeaponModifier {
            aim,
            shots,
            strength,
            armour_penetration,
        }
    }

    /// Getter for the shots stat boost
    ///
    /// # Return
    ///
    /// Option<isize>: The number of shots if it's a ranged weapon, None otherwise
    pub fn get_aim(&self) -> Option<isize> {
        self.aim
    }

    /// Getter for the shots stat boost
    ///
    /// # Return
    ///
    /// Option<isize>: The number of shots if it's a ranged weapon, None otherwise
    pub fn get_shots(&self) -> Option<isize> {
        self.shots
    }

    /// Getter for the strength stat boost
    ///
    /// # Return
    ///
    /// isize: The strength stat boost
    pub fn get_strength(&self) -> isize {
        self.strength
    }

    /// Getter for the armour penetration stat boost
    ///
    /// # Return
    ///
    /// isize: The armour penetration stat boost
    pub fn get_armour_penetration(&self) -> isize {
        self.armour_penetration
    }
}

/// Struct containing the modification granted to dice rolls
///
/// # Attributes
///
/// dice_size (isize): The size of the dice
///
/// dice_number (isize): The number of dice
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize, Copy)]
pub struct DiceModifier {
    dice_size: isize,
    dice_number: isize,
}

impl DiceModifier {
    /// Constructor for the DiceModifier struct
    ///
    /// # Parameters
    ///
    /// dice_size (isize): The size of the dice
    ///
    /// dice_number (isize): The number of dice
    ///
    /// # Return
    ///
    /// DiceModifier: The DiceModifier struct
    pub fn new(dice_size: isize, dice_number: isize) -> Self {
        DiceModifier {
            dice_size,
            dice_number,
        }
    }

    /// Getter for the dice size
    ///
    /// # Return
    ///
    /// isize: The size of the dice
    pub fn get_dice_size(&self) -> isize {
        self.dice_size
    }

    /// Getter for the dice number
    ///
    /// # Return
    ///
    /// isize: The number of dice
    pub fn get_dice_number(&self) -> isize {
        self.dice_number
    }
}

/// Struct representing the effect of a modifier
///
/// # Attributes
///
/// dice_modifier (Option<DiceModifier>): The dice modifier if any
///
/// stats_modifier (StatsModifier): The stats modifier
///
/// stats_replaced (bool): Whether the stats edited are replaced or added
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Modifier {
    dice_modifier: Option<DiceModifier>,
    stats_modifier: StatsModifier,
    stats_replaced: bool,
}

impl Modifier {
    /// Constructor for the Modifier struct
    ///
    /// # Parameters
    ///
    /// dice_modifier (Option<DiceModifier>): The dice modifier if any
    ///
    /// stats_modifier (StatsModifier): The stats modifier
    ///
    /// # Return
    ///
    /// Modifier: The Modifier struct
    pub fn new(
        dice_modifier: Option<DiceModifier>,
        stats_modifier: StatsModifier,
        stats_replaced: bool,
    ) -> Self {
        Modifier {
            dice_modifier,
            stats_modifier,
            stats_replaced,
        }
    }

    /// Getter for the dice modifier
    ///
    /// # Return
    ///
    /// Option<DiceModifier>: The dice modifier if any
    pub fn get_dice_modifier(&self) -> Option<DiceModifier> {
        self.dice_modifier
    }

    /// Getter for the stats modifier
    ///
    /// # Return
    ///
    /// StatsModifier: The stats modifier
    pub fn get_stats_modifier(&self) -> StatsModifier {
        self.stats_modifier
    }

    pub fn get_stats_replaced(&self) -> bool {
        self.stats_replaced
    }

    /// Constructor for a Modifier struct \[Variant: weapon\]
    ///
    /// # Parameters
    ///
    /// shots (isize): The shots stat boost
    ///
    /// strength (isize): The strength stat boost
    ///
    /// armour_penetration (isize): The armour penetration stat boost
    ///
    /// dice_modifier (Option<DiceModifier>): The dice modifier if any
    ///
    /// stats_replaced (bool): Whether the stats edited are replaced or added
    ///
    /// # Return
    ///
    /// Modifier: The Modifier struct
    pub fn new_weapon(
        aim: Option<isize>,
        shots: Option<isize>,
        strength: isize,
        armour_penetration: isize,
        dice_modifier: Option<DiceModifier>,
        stats_replaced: bool,
    ) -> Self {
        Modifier {
            dice_modifier,
            stats_modifier: StatsModifier::Weapon(WeaponModifier::new(
                aim,
                shots,
                strength,
                armour_penetration,
            )),
            stats_replaced,
        }
    }

    /// Constructor for a Modifier struct \[Variant: global\]
    ///
    /// # Parameters
    ///
    /// advance (isize): The advance stat boost
    ///
    /// march (isize): The march stat boost
    ///
    /// discipline (isize): The discipline stat boost
    ///
    /// dice_modifier (Option<DiceModifier>): The dice modifier if any
    ///
    /// stats_replaced (bool): Whether the stats edited are replaced or added
    ///
    /// # Return
    ///
    /// Modifier: The Modifier struct
    pub fn new_global(
        advance: isize,
        march: isize,
        discipline: isize,
        dice_modifier: Option<DiceModifier>,
        stats_replaced: bool,
    ) -> Self {
        Modifier {
            dice_modifier,
            stats_modifier: StatsModifier::Global(GlobalModifier::new(advance, march, discipline)),
            stats_replaced,
        }
    }

    /// Constructor for a Modifier struct \[Variant: offensive\]
    ///
    /// # Parameters
    ///
    /// attack (isize): The attack stat boost
    ///
    /// offensive (isize): The offensive stat boost
    ///
    /// strength (isize): The strength stat boost
    ///
    /// armour_penetration (isize): The armour penetration stat boost
    ///
    /// agility (isize): The agility stat boost
    ///
    /// dice_modifier (Option<DiceModifier>): The dice modifier if any
    ///
    /// stats_replaced (bool): Whether the stats edited are replaced or added
    ///
    /// # Return
    ///
    /// Modifier: The Modifier struct
    pub fn new_offensive(
        attack: isize,
        offensive: isize,
        strength: isize,
        armour_penetration: isize,
        agility: isize,
        dice_modifier: Option<DiceModifier>,
        stats_replaced: bool,
    ) -> Self {
        Modifier {
            dice_modifier,
            stats_modifier: StatsModifier::Offensive(OffensiveModifier::new(
                attack,
                offensive,
                strength,
                armour_penetration,
                agility,
            )),
            stats_replaced,
        }
    }

    /// Constructor for a Modifier struct \[Variant: defensive\]
    ///
    /// # Parameters
    ///
    /// health_points (isize): The health points stat boost
    ///
    /// defense (isize): The defense stat boost
    ///
    /// resilience (isize): The resilience stat boost
    ///
    /// armour (isize): The armour stat boost
    ///
    /// aegis (isize): The aegis stat boost
    ///
    /// dice_modifier (Option<DiceModifier>): The dice modifier if any
    ///
    /// stats_replaced (bool): Whether the stats edited are replaced or added
    ///
    /// # Return
    /// StatsModifier: The Modifier struct
    pub fn new_defensive(
        health_points: isize,
        defense: isize,
        resilience: isize,
        armour: isize,
        aegis: isize,
        dice_modifier: Option<DiceModifier>,
        stats_replaced: bool,
    ) -> Self {
        Modifier {
            dice_modifier,
            stats_modifier: StatsModifier::Defensive(DefensiveModifier::new(
                health_points,
                defense,
                resilience,
                armour,
                aegis,
            )),
            stats_replaced,
        }
    }
}

/// Enum containing all the possible modifiers
///
/// # Variants
///
/// GlobalModifier: The global modifier
///
/// OffensiveModifier: The offensive modifier
///
/// DefensiveModifier: The defensive modifier
///
/// MeleeWeaponModifier: The melee weapon modifier
///
/// WeaponModifier: The ranged weapon modifier
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize, Copy)]
pub enum StatsModifier {
    Global(GlobalModifier),
    Offensive(OffensiveModifier),
    Defensive(DefensiveModifier),
    Weapon(WeaponModifier),
}

impl StatsModifier {
    /// Constructor for the StatsModifier struct \[Variant: weapon\]
    ///
    /// # Parameters
    ///
    /// shots (isize): The shots stat boost
    ///
    /// strength (isize): The strength stat boost
    ///
    /// armour_penetration (isize): The armour penetration stat boost
    ///
    /// # Return
    ///
    /// StatsModifier: The Modifier struct
    pub fn new_weapon(
        aim: Option<isize>,
        shots: Option<isize>,
        strength: isize,
        armour_penetration: isize,
    ) -> Self {
        StatsModifier::Weapon(WeaponModifier::new(
            aim,
            shots,
            strength,
            armour_penetration,
        ))
    }

    /// Constructor for the StatsModifier struct \[Variant: global\]
    ///
    /// # Parameters
    ///
    /// advance (isize): The advance stat boost
    ///
    /// march (isize): The march stat boost
    ///
    /// discipline (isize): The discipline stat boost
    ///
    /// # Return
    ///
    /// StatsModifier: The Modifier struct
    pub fn new_global(advance: isize, march: isize, discipline: isize) -> Self {
        StatsModifier::Global(GlobalModifier::new(advance, march, discipline))
    }

    /// Constructor for the StatsModifier struct \[Variant: offensive\]
    ///
    /// # Parameters
    ///
    /// attack (isize): The attack stat boost
    ///
    /// offensive (isize): The offensive stat boost
    ///
    /// strength (isize): The strength stat boost
    ///
    /// armour_penetration (isize): The armour penetration stat boost
    ///
    /// agility (isize): The agility stat boost
    ///
    /// # Return
    ///
    /// StatsModifier: The Modifier struct
    pub fn new_offensive(
        attack: isize,
        offensive: isize,
        strength: isize,
        armour_penetration: isize,
        agility: isize,
    ) -> Self {
        StatsModifier::Offensive(OffensiveModifier::new(
            attack,
            offensive,
            strength,
            armour_penetration,
            agility,
        ))
    }

    /// Constructor for the StatsModifier struct \[Variant: defensive\]
    ///
    /// # Parameters
    ///
    /// health_points (isize): The health points stat boost
    ///
    /// defense (isize): The defense stat boost
    ///
    /// resilience (isize): The resilience stat boost
    ///
    /// armour (isize): The armour stat boost
    ///
    /// # Return
    /// StatsModifier: The Modifier struct
    pub fn new_defensive(
        health_points: isize,
        defense: isize,
        resilience: isize,
        armour: isize,
        aegis: isize,
    ) -> Self {
        StatsModifier::Defensive(DefensiveModifier::new(
            health_points,
            defense,
            resilience,
            armour,
            aegis,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ranged_weapon() {
        let modifier: StatsModifier = StatsModifier::new_weapon(Some(3), Some(2), 3, 4);
        match modifier {
            StatsModifier::Weapon(ranged_weapon_modifier) => {
                match ranged_weapon_modifier.get_shots() {
                    Some(shots) => assert_eq!(shots, 2),
                    None => assert!(false),
                }
                assert_eq!(ranged_weapon_modifier.get_strength(), 3);
                assert_eq!(ranged_weapon_modifier.get_armour_penetration(), 4);
            }
            _ => panic!("StatsModifier is not a ranged weapon modifier"),
        }
    }

    #[test]
    fn test_new_close_range_weapon() {
        let modifier: StatsModifier = StatsModifier::new_weapon(Some(3), None, 3, 4);
        match modifier {
            StatsModifier::Weapon(ranged_weapon_modifier) => {
                match ranged_weapon_modifier.get_shots() {
                    Some(_) => assert!(false),
                    None => assert!(true),
                }
                assert_eq!(ranged_weapon_modifier.get_strength(), 3);
                assert_eq!(ranged_weapon_modifier.get_armour_penetration(), 4);
            }
            _ => panic!("StatsModifier is not a ranged weapon modifier"),
        }
    }

    #[test]
    fn test_new_global() {
        let modifier: StatsModifier = StatsModifier::new_global(1, 2, 3);
        match modifier {
            StatsModifier::Global(global_modifier) => {
                assert_eq!(global_modifier.get_advance(), 1);
                assert_eq!(global_modifier.get_march(), 2);
                assert_eq!(global_modifier.get_discipline(), 3);
            }
            _ => panic!("StatsModifier is not a global modifier"),
        }
    }

    #[test]
    fn test_new_offensive() {
        let modifier: StatsModifier = StatsModifier::new_offensive(1, 2, 3, 4, 5);
        match modifier {
            StatsModifier::Offensive(offensive_modifier) => {
                assert_eq!(offensive_modifier.get_attack(), 1);
                assert_eq!(offensive_modifier.get_offensive(), 2);
                assert_eq!(offensive_modifier.get_strength(), 3);
                assert_eq!(offensive_modifier.get_armour_penetration(), 4);
                assert_eq!(offensive_modifier.get_agility(), 5);
            }
            _ => panic!("StatsModifier is not an offensive modifier"),
        }
    }

    #[test]
    fn test_new_defensive() {
        let modifier: StatsModifier = StatsModifier::new_defensive(1, 2, 3, 4, 0);
        match modifier {
            StatsModifier::Defensive(defensive_modifier) => {
                assert_eq!(defensive_modifier.get_health_points(), 1);
                assert_eq!(defensive_modifier.get_defense(), 2);
                assert_eq!(defensive_modifier.get_resilience(), 3);
                assert_eq!(defensive_modifier.get_armour(), 4);
            }
            _ => panic!("StatsModifier is not a defensive modifier"),
        }
    }
}
