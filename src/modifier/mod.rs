/// Struct containing the modification granted to the global stats of a Model
///
/// ## Attributes
/// advance (isize): The advance stat boost
///
/// march (isize): The march stat boost
///
/// discipline (isize): The discipline stat boost
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GlobalModifier {
    advance: isize,
    march: isize,
    discipline: isize,
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
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OffensiveModifier {
    attack: isize,
    offensive: isize,
    strength: isize,
    armour_penetration: isize,
    agility: isize,
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
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DefensiveModifier {
    health_points: isize,
    defense: isize,
    resilience: isize,
    armour: isize,
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
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WeaponModifier {
    shots: Option<isize>,
    strength: isize,
    armour_penetration: isize,
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
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Modifier {
    Global(GlobalModifier),
    Offensive(OffensiveModifier),
    Defensive(DefensiveModifier),
    Weapon(WeaponModifier),
}

impl GlobalModifier {
    /// Constructor for the GlobalModifier struct
    ///
    /// ## Parameters
    /// (isize) advance : The advance stat boost
    ///
    /// (isize) march : The march stat boost
    ///
    /// (isize) discipline : The discipline stat boost
    ///
    /// ## Return
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
    /// ## Return
    /// isize: The advance stat boost
    pub fn get_advance(&self) -> isize {
        self.advance
    }

    /// Getter for the march stat boost
    ///
    /// ## Return
    /// isize: The march stat boost
    pub fn get_march(&self) -> isize {
        self.march
    }

    /// Getter for the discipline stat boost
    ///
    /// ## Return
    /// isize: The discipline stat boost
    pub fn get_discipline(&self) -> isize {
        self.discipline
    }
}

impl OffensiveModifier {
    /// Constructor for the OffensiveModifier struct
    ///
    /// ## Parameters
    /// (isize) attack : The attack stat boost
    ///
    /// (isize) offensive : The offensive stat boost
    ///
    /// (isize) strength : The strength stat boost
    ///
    /// (isize) armour_penetration : The armour penetration stat boost
    ///
    /// (isize) agility : The agility stat boost
    ///
    /// ## Return
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
    /// ## Return
    /// isize: The attack stat boost
    pub fn get_attack(&self) -> isize {
        self.attack
    }

    /// Getter for the offensive stat boost
    ///
    /// ## Return
    /// isize: The offensive stat boost
    pub fn get_offensive(&self) -> isize {
        self.offensive
    }

    /// Getter for the strength stat boost
    ///
    /// ## Return
    /// isize: The strength stat boost
    pub fn get_strength(&self) -> isize {
        self.strength
    }

    /// Getter for the armour penetration stat boost
    ///
    /// ## Return
    /// isize: The armour penetration stat boost
    pub fn get_armour_penetration(&self) -> isize {
        self.armour_penetration
    }

    /// Getter for the agility stat boost
    ///
    /// ## Return
    /// isize: The agility stat boost
    pub fn get_agility(&self) -> isize {
        self.agility
    }
}

impl DefensiveModifier {
    /// Constructor for the DefensiveModifier struct
    ///
    /// ## Parameters
    /// (isize) health_points : The health points stat boost
    ///
    /// (isize) defense : The defense stat boost
    ///
    /// (isize) resilience : The resilience stat boost
    ///
    /// (isize) armour : The armour stat boost
    ///
    /// ## Return
    /// DefensiveModifier: The DefensiveModifier struct
    pub fn new(health_points: isize, defense: isize, resilience: isize, armour: isize) -> Self {
        DefensiveModifier {
            health_points,
            defense,
            resilience,
            armour,
        }
    }

    /// Getter for the health points stat boost
    ///
    /// ## Return
    /// isize: The health points stat boost
    pub fn get_health_points(&self) -> isize {
        self.health_points
    }

    /// Getter for the defense stat boost
    ///
    /// ## Return
    /// isize: The defense stat boost
    pub fn get_defense(&self) -> isize {
        self.defense
    }

    /// Getter for the resilience stat boost
    ///
    /// ## Return
    /// isize: The resilience stat boost
    pub fn get_resilience(&self) -> isize {
        self.resilience
    }

    /// Getter for the armour stat boost
    ///
    /// ## Return
    /// isize: The armour stat boost
    pub fn get_armour(&self) -> isize {
        self.armour
    }
}

impl WeaponModifier {
    /// Constructor for the WeaponModifier struct
    ///
    /// ## Parameters
    /// (isize) shots : The shots stat boost
    ///
    /// (isize) strength : The strength stat boost
    ///
    /// (isize) armour_penetration : The armour penetration stat boost
    ///
    /// ## Return
    /// WeaponModifier: The WeaponModifier struct
    pub fn new(shots: Option<isize>, strength: isize, armour_penetration: isize) -> WeaponModifier {
        WeaponModifier {
            shots,
            strength,
            armour_penetration,
        }
    }

    /// Getter for the shots stat boost
    ///
    /// ## Return
    /// Option<isize>: The nulber of shots if it's a ranged weapon, None otherwise
    pub fn get_shots(&self) -> Option<isize> {
        self.shots
    }

    /// Getter for the strength stat boost
    ///
    /// ## Return
    /// isize: The strength stat boost
    pub fn get_strength(&self) -> isize {
        self.strength
    }

    /// Getter for the armour penetration stat boost
    ///
    /// ## Return
    /// isize: The armour penetration stat boost
    pub fn get_armour_penetration(&self) -> isize {
        self.armour_penetration
    }
}

impl Modifier {
    /// Constructor for the Modifier struct \[Variant: weapon\]
    ///
    /// ## Parameters
    /// (isize) shots : The shots stat boost
    ///
    /// (isize) strength : The strength stat boost
    ///
    /// (isize) armour_penetration : The armour penetration stat boost
    ///
    /// ## Return
    /// Modifier: The Modifier struct
    pub fn new_weapon(shots: Option<isize>, strength: isize, armour_penetration: isize) -> Self {
        Modifier::Weapon(WeaponModifier::new(shots, strength, armour_penetration))
    }

    /// Constructor for the Modifier struct \[Variant: global\]
    ///
    /// ## Parameters
    /// (isize) advance : The advance stat boost
    ///
    /// (isize) march : The march stat boost
    ///
    /// (isize) discipline : The discipline stat boost
    ///
    /// ## Return
    /// Modifier: The Modifier struct
    pub fn new_global(advance: isize, march: isize, discipline: isize) -> Self {
        Modifier::Global(GlobalModifier::new(advance, march, discipline))
    }

    /// Constructor for the Modifier struct \[Variant: offensive\]
    ///
    /// ## Parameters
    /// (isize) attack : The attack stat boost
    ///
    /// (isize) offensive : The offensive stat boost
    ///
    /// (isize) strength : The strength stat boost
    ///
    /// (isize) armour_penetration : The armour penetration stat boost
    ///
    /// (isize) agility : The agility stat boost
    ///
    /// ## Return
    /// Modifier: The Modifier struct
    pub fn new_offensive(
        attack: isize,
        offensive: isize,
        strength: isize,
        armour_penetration: isize,
        agility: isize,
    ) -> Self {
        Modifier::Offensive(OffensiveModifier::new(
            attack,
            offensive,
            strength,
            armour_penetration,
            agility,
        ))
    }

    /// Constructor for the Modifier struct \[Variant: defensive\]
    ///
    /// ## Parameters
    /// (isize) health_points : The health points stat boost
    ///
    /// (isize) defense : The defense stat boost
    ///
    /// (isize) resilience : The resilience stat boost
    ///
    /// (isize) armour : The armour stat boost
    ///
    /// ## Return
    /// Modifier: The Modifier struct
    pub fn new_defensive(
        health_points: isize,
        defense: isize,
        resilience: isize,
        armour: isize,
    ) -> Self {
        Modifier::Defensive(DefensiveModifier::new(
            health_points,
            defense,
            resilience,
            armour,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ranged_weapon() {
        let modifier: Modifier = Modifier::new_weapon(Some(2), 3, 4);
        match modifier {
            Modifier::Weapon(ranged_weapon_modifier) => {
                match ranged_weapon_modifier.get_shots() {
                    Some(shots) => assert_eq!(shots, 2),
                    None => assert!(false),
                }
                assert_eq!(ranged_weapon_modifier.get_strength(), 3);
                assert_eq!(ranged_weapon_modifier.get_armour_penetration(), 4);
            }
            _ => panic!("Modifier is not a ranged weapon modifier"),
        }
    }

    #[test]
    fn test_new_close_range_weapon() {
        let modifier: Modifier = Modifier::new_weapon(None, 3, 4);
        match modifier {
            Modifier::Weapon(ranged_weapon_modifier) => {
                match ranged_weapon_modifier.get_shots() {
                    Some(_) => assert!(false),
                    None => assert!(true),
                }
                assert_eq!(ranged_weapon_modifier.get_strength(), 3);
                assert_eq!(ranged_weapon_modifier.get_armour_penetration(), 4);
            }
            _ => panic!("Modifier is not a ranged weapon modifier"),
        }
    }

    #[test]
    fn test_new_global() {
        let modifier: Modifier = Modifier::new_global(1, 2, 3);
        match modifier {
            Modifier::Global(global_modifier) => {
                assert_eq!(global_modifier.get_advance(), 1);
                assert_eq!(global_modifier.get_march(), 2);
                assert_eq!(global_modifier.get_discipline(), 3);
            }
            _ => panic!("Modifier is not a global modifier"),
        }
    }

    #[test]
    fn test_new_offensive() {
        let modifier: Modifier = Modifier::new_offensive(1, 2, 3, 4, 5);
        match modifier {
            Modifier::Offensive(offensive_modifier) => {
                assert_eq!(offensive_modifier.get_attack(), 1);
                assert_eq!(offensive_modifier.get_offensive(), 2);
                assert_eq!(offensive_modifier.get_strength(), 3);
                assert_eq!(offensive_modifier.get_armour_penetration(), 4);
                assert_eq!(offensive_modifier.get_agility(), 5);
            }
            _ => panic!("Modifier is not an offensive modifier"),
        }
    }

    #[test]
    fn test_new_defensive() {
        let modifier: Modifier = Modifier::new_defensive(1, 2, 3, 4);
        match modifier {
            Modifier::Defensive(defensive_modifier) => {
                assert_eq!(defensive_modifier.get_health_points(), 1);
                assert_eq!(defensive_modifier.get_defense(), 2);
                assert_eq!(defensive_modifier.get_resilience(), 3);
                assert_eq!(defensive_modifier.get_armour(), 4);
            }
            _ => panic!("Modifier is not a defensive modifier"),
        }
    }
}
