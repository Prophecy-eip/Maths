mod modifiers_id;

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

/// Struct containing the modification granted by a close range weapon
///
/// ## Attributes
/// strength (isize): The strength stat boost
///
/// armour_penetration (isize): The armour penetration stat boost
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeleeWeaponModifier {
    strength: isize,
    armour_penetration: isize,
}

/// Struct containing the modification granted by a ranged weapon
///
/// ## Attributes
/// range (isize): The range of the weapon
///
/// shots (isize): The number of shots
///
/// strength (isize): The strength stat boost
///
/// armour_penetration (isize): The armour penetration stat boost
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RangedWeaponModifier {
    range: isize,
    shots: isize,
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
/// RangedWeaponModifier: The ranged weapon modifier
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Modifier {
    Global(GlobalModifier),
    Offensive(OffensiveModifier),
    Defensive(DefensiveModifier),
    MeleeWeapon(MeleeWeaponModifier),
    RangedWeapon(RangedWeaponModifier),
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
    /// ## Returns
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
    /// ## Returns
    /// isize: The advance stat boost
    pub fn get_advance(&self) -> isize {
        self.advance
    }

    /// Getter for the march stat boost
    ///
    /// ## Returns
    /// isize: The march stat boost
    pub fn get_march(&self) -> isize {
        self.march
    }

    /// Getter for the discipline stat boost
    ///
    /// ## Returns
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
    /// ## Returns
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
    /// ## Returns
    /// isize: The attack stat boost
    pub fn get_attack(&self) -> isize {
        self.attack
    }

    /// Getter for the offensive stat boost
    ///
    /// ## Returns
    /// isize: The offensive stat boost
    pub fn get_offensive(&self) -> isize {
        self.offensive
    }

    /// Getter for the strength stat boost
    ///
    /// ## Returns
    /// isize: The strength stat boost
    pub fn get_strength(&self) -> isize {
        self.strength
    }

    /// Getter for the armour penetration stat boost
    ///
    /// ## Returns
    /// isize: The armour penetration stat boost
    pub fn get_armour_penetration(&self) -> isize {
        self.armour_penetration
    }

    /// Getter for the agility stat boost
    ///
    /// ## Returns
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
    /// ## Returns
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
    /// ## Returns
    /// isize: The health points stat boost
    pub fn get_health_points(&self) -> isize {
        self.health_points
    }

    /// Getter for the defense stat boost
    ///
    /// ## Returns
    /// isize: The defense stat boost
    pub fn get_defense(&self) -> isize {
        self.defense
    }

    /// Getter for the resilience stat boost
    ///
    /// ## Returns
    /// isize: The resilience stat boost
    pub fn get_resilience(&self) -> isize {
        self.resilience
    }

    /// Getter for the armour stat boost
    ///
    /// ## Returns
    /// isize: The armour stat boost
    pub fn get_armour(&self) -> isize {
        self.armour
    }
}

impl MeleeWeaponModifier {
    /// Constructor for the MeleeWeaponModifier struct
    ///
    /// ## Parameters
    /// (isize) strength : The strength stat boost
    ///
    /// (isize) armour_penetration : The armour penetration stat boost
    ///
    /// ## Returns
    /// MeleeWeaponModifier: The MeleeWeaponModifier struct
    pub fn new(strength: isize, armour_penetration: isize) -> MeleeWeaponModifier {
        MeleeWeaponModifier {
            strength,
            armour_penetration,
        }
    }

    /// Getter for the strength stat boost
    ///
    /// ## Returns
    /// isize: The strength stat boost
    pub fn get_strength(&self) -> isize {
        self.strength
    }

    /// Getter for the armour penetration stat boost
    ///
    /// ## Returns
    /// isize: The armour penetration stat boost
    pub fn get_armour_penetration(&self) -> isize {
        self.armour_penetration
    }
}

impl RangedWeaponModifier {
    /// Constructor for the RangedWeaponModifier struct
    ///
    /// ## Parameters
    /// (isize) range : The range stat boost
    ///
    /// (isize) shots : The shots stat boost
    ///
    /// (isize) strength : The strength stat boost
    ///
    /// (isize) armour_penetration : The armour penetration stat boost
    ///
    /// ## Returns
    /// RangedWeaponModifier: The RangedWeaponModifier struct
    pub fn new(
        range: isize,
        shots: isize,
        strength: isize,
        armour_penetration: isize,
    ) -> RangedWeaponModifier {
        RangedWeaponModifier {
            range,
            shots,
            strength,
            armour_penetration,
        }
    }

    /// Getter for the range stat boost
    ///
    /// ## Returns
    /// isize: The range stat boost
    pub fn get_range(&self) -> isize {
        self.range
    }

    /// Getter for the shots stat boost
    ///
    /// ## Returns
    /// isize: The shots stat boost
    pub fn get_shots(&self) -> isize {
        self.shots
    }

    /// Getter for the strength stat boost
    ///
    /// ## Returns
    /// isize: The strength stat boost
    pub fn get_strength(&self) -> isize {
        self.strength
    }

    /// Getter for the armour penetration stat boost
    ///
    /// ## Returns
    /// isize: The armour penetration stat boost
    pub fn get_armour_penetration(&self) -> isize {
        self.armour_penetration
    }
}

impl Modifier {
    /// Constructor for the Modifier struct \[Variant: melee weapon\]
    ///
    /// ## Parameters
    /// (isize) strength : The strength stat boost
    ///
    /// (isize) armour_penetration : The armour penetration stat boost
    ///
    /// ## Returns
    /// Modifier: The Modifier struct
    pub fn new_melee_weapon(strength: isize, armour_penetration: isize) -> Self {
        Modifier::MeleeWeapon(MeleeWeaponModifier::new(strength, armour_penetration))
    }

    /// Constructor for the Modifier struct \[Variant: ranged weapon\]
    ///
    /// ## Parameters
    /// (isize) range : The range stat boost
    ///
    /// (isize) shots : The shots stat boost
    ///
    /// (isize) strength : The strength stat boost
    ///
    /// (isize) armour_penetration : The armour penetration stat boost
    ///
    /// ## Returns
    /// Modifier: The Modifier struct
    pub fn new_ranged_weapon(
        range: isize,
        shots: isize,
        strength: isize,
        armour_penetration: isize,
    ) -> Self {
        Modifier::RangedWeapon(RangedWeaponModifier::new(
            range,
            shots,
            strength,
            armour_penetration,
        ))
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
    /// ## Returns
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
    /// ## Returns
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
    /// ## Returns
    /// Modifier: The Modifier struct
    pub fn new_defense(
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
