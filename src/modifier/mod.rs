mod modifiers_id;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GlobalModifier {
    advance: isize,
    march: isize,
    discipline: isize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OffensiveModifier {
    attack: isize,
    offensive: isize,
    strength: isize,
    armour_penetration: isize,
    agility: isize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DefensiveModifier {
    health_points: isize,
    defense: isize,
    resilience: isize,
    armour: isize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeleeWeaponModifier {
    strength: isize,
    armour_penetration: isize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RangedWeaponModifier {
    range: isize,
    shots: isize,
    strength: isize,
    armour_penetration: isize,
}

/// Struct containing all the information about a single Modifier for a Model
///
/// ## Attributes
/// stat (Stats): Stat modified by the Modifier
///
/// nb_dice (isize): The number of dice
///
/// requirements (Vec<String>): The requirements for the Modifier to apply (as an array of flags)
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Modifier {
    Global(GlobalModifier),
    Offensive(OffensiveModifier),
    Defensive(DefensiveModifier),
    MeleeWeapon(MeleeWeaponModifier),
    RangedWeapon(RangedWeaponModifier),
}

impl GlobalModifier {
    pub fn new(advance: isize, march: isize, discipline: isize) -> Self {
        GlobalModifier {
            advance,
            march,
            discipline,
        }
    }

    pub fn get_advance(&self) -> isize {
        self.advance
    }

    pub fn get_march(&self) -> isize {
        self.march
    }

    pub fn get_discipline(&self) -> isize {
        self.discipline
    }
}

impl OffensiveModifier {
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

    pub fn get_attack(&self) -> isize {
        self.attack
    }

    pub fn get_offensive(&self) -> isize {
        self.offensive
    }

    pub fn get_strength(&self) -> isize {
        self.strength
    }

    pub fn get_armour_penetration(&self) -> isize {
        self.armour_penetration
    }

    pub fn get_agility(&self) -> isize {
        self.agility
    }
}

impl DefensiveModifier {
    pub fn new(health_points: isize, defense: isize, resilience: isize, armour: isize) -> Self {
        DefensiveModifier {
            health_points,
            defense,
            resilience,
            armour,
        }
    }

    pub fn get_health_points(&self) -> isize {
        self.health_points
    }

    pub fn get_defense(&self) -> isize {
        self.defense
    }

    pub fn get_resilience(&self) -> isize {
        self.resilience
    }

    pub fn get_armour(&self) -> isize {
        self.armour
    }
}

impl MeleeWeaponModifier {
    pub fn new(strength: isize, armour_penetration: isize) -> MeleeWeaponModifier {
        MeleeWeaponModifier {
            strength,
            armour_penetration,
        }
    }

    pub fn get_strength(&self) -> isize {
        self.strength
    }

    pub fn get_armour_penetration(&self) -> isize {
        self.armour_penetration
    }
}

impl RangedWeaponModifier {
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

    pub fn get_range(&self) -> isize {
        self.range
    }

    pub fn get_shots(&self) -> isize {
        self.shots
    }

    pub fn get_strength(&self) -> isize {
        self.strength
    }

    pub fn get_armour_penetration(&self) -> isize {
        self.armour_penetration
    }
}

impl Modifier {
    pub fn new_melee_weapon(strength: isize, armour_penetration: isize) -> Self {
        Modifier::MeleeWeapon(MeleeWeaponModifier::new(strength, armour_penetration))
    }

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

    pub fn new_global(advance: isize, march: isize, discipline: isize) -> Self {
        Modifier::Global(GlobalModifier::new(advance, march, discipline))
    }

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
