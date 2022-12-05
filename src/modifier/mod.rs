// use std::fs;

mod modifiers_id;

#[derive(Clone, PartialEq, Eq, Debug)]

pub struct MeleeWeaponStats {
    strength: usize,
    armour_penetration: usize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RangedWeaponStats {
    range: usize,
    shots: usize,
    strength: usize,
    armour_penetration: usize,
}

/// Struct containing all the information about a single Modifier for a Model
///
/// ## Attributes
/// stat (Stats): Stat modified by the Modifier
///
/// nb_dice (usize): The number of dice
///
/// requirements (Vec<String>): The requirements for the Modifier to apply (as an array of flags)
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Modifier {
    MeleeWeapon(MeleeWeaponStats),
    RangedWeapon(RangedWeaponStats),
}

impl MeleeWeaponStats {
    pub fn new(strength: usize, armour_penetration: usize) -> MeleeWeaponStats {
        MeleeWeaponStats {
            strength,
            armour_penetration,
        }
    }

    pub fn get_strength(&self) -> usize {
        self.strength
    }

    pub fn get_armour_penetration(&self) -> usize {
        self.armour_penetration
    }
}

impl RangedWeaponStats {
    pub fn new(
        range: usize,
        shots: usize,
        strength: usize,
        armour_penetration: usize,
    ) -> RangedWeaponStats {
        RangedWeaponStats {
            range,
            shots,
            strength,
            armour_penetration,
        }
    }

    pub fn get_range(&self) -> usize {
        self.range
    }

    pub fn get_shots(&self) -> usize {
        self.shots
    }

    pub fn get_strength(&self) -> usize {
        self.strength
    }

    pub fn get_armour_penetration(&self) -> usize {
        self.armour_penetration
    }
}

impl Modifier {
    pub fn new_melee_weapon(strength: usize, armour_penetration: usize) -> Self {
        Modifier::MeleeWeapon(MeleeWeaponStats {
            strength,
            armour_penetration,
        })
    }

    pub fn new_ranged_weapon(
        range: usize,
        shots: usize,
        strength: usize,
        armour_penetration: usize,
    ) -> Self {
        Modifier::RangedWeapon(RangedWeaponStats {
            range,
            shots,
            strength,
            armour_penetration,
        })
    }
}

/*fn get_file_contents(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents
}
*/
