#[derive(serde::Serialize, serde::Deserialize)]
pub struct GlobalModifierDto {
    pub advance: isize,
    pub march: isize,
    pub discipline: isize,
}

impl GlobalModifierDto {
    pub fn new(advance: isize, march: isize, discipline: isize) -> GlobalModifierDto {
        GlobalModifierDto {
            advance,
            march,
            discipline,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DefensiveModifierDto {
    pub health_points: isize,
    pub defense: isize,
    pub resilience: isize,
    pub armour: isize,
    pub aegis: isize,
}

impl DefensiveModifierDto {
    pub fn new(
        health_points: isize,
        defense: isize,
        resilience: isize,
        armour: isize,
        aegis: isize,
    ) -> DefensiveModifierDto {
        DefensiveModifierDto {
            health_points,
            defense,
            resilience,
            armour,
            aegis,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OffensiveModifierDto {
    pub attack: isize,
    pub offensive: isize,
    pub strength: isize,
    pub armour_penetration: isize,
    pub agility: isize,
}

impl OffensiveModifierDto {
    pub fn new(
        attack: isize,
        offensive: isize,
        strength: isize,
        armour_penetration: isize,
        agility: isize,
    ) -> OffensiveModifierDto {
        OffensiveModifierDto {
            attack,
            offensive,
            strength,
            armour_penetration,
            agility,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WeaponModifierDto {
    shots: Option<isize>,
    strength: isize,
    armour_penetration: isize,
}

impl WeaponModifierDto {
    pub fn new(
        shots: Option<isize>,
        strength: isize,
        armour_penetration: isize,
    ) -> WeaponModifierDto {
        WeaponModifierDto {
            shots,
            strength,
            armour_penetration,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ModifierDto {
    Global(GlobalModifierDto),
    Defensive(DefensiveModifierDto),
    Offensive(OffensiveModifierDto),
    Weapon(WeaponModifierDto),
}

impl ModifierDto {
    pub fn new_global_dto(advance: isize, march: isize, discipline: isize) -> ModifierDto {
        ModifierDto::Global(GlobalModifierDto::new(advance, march, discipline))
    }

    pub fn new_defensive_dto(
        health_points: isize,
        defense: isize,
        resilience: isize,
        armour: isize,
        aegis: isize,
    ) -> ModifierDto {
        ModifierDto::Defensive(DefensiveModifierDto::new(
            health_points,
            defense,
            resilience,
            armour,
            aegis,
        ))
    }

    pub fn new_offensive_dto(
        attack: isize,
        offensive: isize,
        strength: isize,
        armour_penetration: isize,
        agility: isize,
    ) -> ModifierDto {
        ModifierDto::Offensive(OffensiveModifierDto::new(
            attack,
            offensive,
            strength,
            armour_penetration,
            agility,
        ))
    }

    pub fn new_weapon_dto(
        shots: Option<isize>,
        strength: isize,
        armour_penetration: isize,
    ) -> ModifierDto {
        ModifierDto::Weapon(WeaponModifierDto::new(shots, strength, armour_penetration))
    }

    pub fn hydrate(&self) -> crate::modifier::Modifier {
        match self {
            ModifierDto::Global(global) => crate::modifier::Modifier::new_global(
                global.advance,
                global.march,
                global.discipline,
            ),
            ModifierDto::Defensive(defensive) => crate::modifier::Modifier::new_defensive(
                defensive.health_points,
                defensive.defense,
                defensive.resilience,
                defensive.armour,
                defensive.aegis,
            ),
            ModifierDto::Offensive(offensive) => crate::modifier::Modifier::new_offensive(
                offensive.attack,
                offensive.offensive,
                offensive.strength,
                offensive.armour_penetration,
                offensive.agility,
            ),
            ModifierDto::Weapon(weapon) => crate::modifier::Modifier::new_weapon(
                weapon.shots,
                weapon.strength,
                weapon.armour_penetration,
            ),
        }
    }

    pub fn dehydrate(modifier: &crate::modifier::Modifier) -> ModifierDto {
        match modifier {
            crate::modifier::Modifier::Global(global) => ModifierDto::new_global_dto(
                global.get_advance(),
                global.get_march(),
                global.get_discipline(),
            ),
            crate::modifier::Modifier::Defensive(defensive) => ModifierDto::new_defensive_dto(
                defensive.get_health_points(),
                defensive.get_defense(),
                defensive.get_resilience(),
                defensive.get_armour(),
                defensive.get_aegis(),
            ),
            crate::modifier::Modifier::Offensive(offensive) => ModifierDto::new_offensive_dto(
                offensive.get_attack(),
                offensive.get_offensive(),
                offensive.get_strength(),
                offensive.get_armour_penetration(),
                offensive.get_agility(),
            ),
            crate::modifier::Modifier::Weapon(weapon) => ModifierDto::new_weapon_dto(
                weapon.get_shots(),
                weapon.get_strength(),
                weapon.get_armour_penetration(),
            ),
        }
    }
}
