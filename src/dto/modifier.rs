//! ModifierDto module
//!
//! This module contains the ModifierDto enum and its implementation.
//! The goal is to be able to communicate easily with outside of the library.

/// Struct used to represent a Global modifier outside of rust code
///
/// # Attributes
///
/// advance (isize): The advance modifier
///
/// march (isize): The march modifier
///
/// discipline (isize): The discipline modifier
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct GlobalModifierDto {
    pub advance: isize,
    pub march: isize,
    pub discipline: isize,
}

impl GlobalModifierDto {
    /// Create a new GlobalModifierDto
    ///
    /// # Parameters
    ///
    /// advance (isize): The advance modifier
    ///
    /// march (isize): The march modifier
    ///
    /// discipline (isize): The discipline modifier
    ///
    /// # Return
    ///
    /// GlobalModifierDto: The newly created GlobalModifierDto
    pub fn new(advance: isize, march: isize, discipline: isize) -> GlobalModifierDto {
        GlobalModifierDto {
            advance,
            march,
            discipline,
        }
    }
}

/// Struct used to represent a Defensive modifier outside of rust code
///
/// # Attributes
///
/// health_points (isize): The health points modifier
///
/// defense (isize): The defense modifier
///
/// resilience (isize): The resilience modifier
///
/// armour (isize): The armour modifier
///
/// aegis (isize): The aegis modifier
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct DefensiveModifierDto {
    pub health_points: isize,
    pub defense: isize,
    pub resilience: isize,
    pub armour: isize,
    pub aegis: isize,
}

impl DefensiveModifierDto {
    /// Create a new DefensiveModifierDto
    ///
    /// # Parameters
    ///
    /// health_points (isize): The health points modifier
    ///
    /// defense (isize): The defense modifier
    ///
    /// resilience (isize): The resilience modifier
    ///
    /// armour (isize): The armour modifier
    ///
    /// aegis (isize): The aegis modifier
    ///
    /// # Return
    ///
    /// DefensiveModifierDto: The newly created DefensiveModifierDto
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

/// Struct used to represent an Offensive modifier outside of rust code
///
/// # Attributes
///
/// attack (isize): The attack modifier
///
/// offensive (isize): The offensive modifier
///
/// strength (isize): The strength modifier
///
/// armour_penetration (isize): The armour penetration modifier
///
/// agility (isize): The agility modifier
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct OffensiveModifierDto {
    pub attack: isize,
    pub offensive: isize,
    pub strength: isize,
    pub armour_penetration: isize,
    pub agility: isize,
}

impl OffensiveModifierDto {
    /// Create a new OffensiveModifierDto
    ///
    /// # Parameters
    ///
    /// attack (isize): The attack modifier
    ///
    /// offensive (isize): The offensive modifier
    ///
    /// strength (isize): The strength modifier
    ///
    /// armour_penetration (isize): The armour penetration modifier
    ///
    /// agility (isize): The agility modifier
    ///
    /// # Return
    ///
    /// OffensiveModifierDto: The newly created OffensiveModifierDto
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

/// Struct used to represent a Weapon modifier outside of rust code
///
/// # Attributes
///
/// shots (Option<isize>): The shots modifier
///
/// strength (isize): The strength modifier
///
/// armour_penetration (isize): The armour penetration modifier
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct WeaponModifierDto {
    shots: Option<isize>,
    strength: isize,
    armour_penetration: isize,
}

impl WeaponModifierDto {
    /// Create a new WeaponModifierDto
    ///
    /// # Parameters
    ///
    /// shots (Option<isize>): The shots modifier
    ///
    /// strength (isize): The strength modifier
    ///
    /// armour_penetration (isize): The armour penetration modifier
    ///
    /// # Return
    ///
    /// WeaponModifierDto: The newly created WeaponModifierDto
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

/// Enum used to represent a modifier outside of rust code
///
/// # Attributes
///
/// Global (GlobalModifierDto): The global modifier
///
/// Defensive (DefensiveModifierDto): The defensive modifier
///
/// Offensive (OffensiveModifierDto): The offensive modifier
///
/// Weapon (WeaponModifierDto): The weapon modifier
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ModifierDto {
    Global(GlobalModifierDto),
    Defensive(DefensiveModifierDto),
    Offensive(OffensiveModifierDto),
    Weapon(WeaponModifierDto),
}

impl ModifierDto {
    /// Create a new ModifierDto
    ///
    /// # Parameters
    ///
    /// advance (isize): The advance modifier
    ///
    /// march (isize): The march modifier
    ///
    /// discipline (isize): The discipline modifier
    ///
    /// # Return
    ///
    /// ModifierDto: The newly created ModifierDto
    pub fn new_global_dto(advance: isize, march: isize, discipline: isize) -> ModifierDto {
        ModifierDto::Global(GlobalModifierDto::new(advance, march, discipline))
    }

    /// Create a new ModifierDto
    ///
    /// # Parameters
    ///
    /// health_points (isize): The health points modifier
    ///
    /// defense (isize): The defense modifier
    ///
    /// resilience (isize): The resilience modifier
    ///
    /// armour (isize): The armour modifier
    ///
    /// aegis (isize): The aegis modifier
    ///
    /// # Return
    ///
    /// ModifierDto: The newly created ModifierDto
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

    /// Create a new ModifierDto
    ///
    /// # Parameters
    ///
    /// attack (isize): The attack modifier
    ///
    /// offensive (isize): The offensive modifier
    ///
    /// strength (isize): The strength modifier
    ///
    /// armour_penetration (isize): The armour penetration modifier
    ///
    /// agility (isize): The agility modifier
    ///
    /// # Return
    ///
    /// ModifierDto: The newly created ModifierDto
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

    /// Create a new ModifierDto
    ///
    /// # Parameters
    ///
    /// shots (Option<isize>): The shots modifier
    ///
    /// strength (isize): The strength modifier
    ///
    /// armour_penetration (isize): The armour penetration modifier
    ///
    /// # Return
    ///
    /// ModifierDto: The newly created ModifierDto
    pub fn new_weapon_dto(
        shots: Option<isize>,
        strength: isize,
        armour_penetration: isize,
    ) -> ModifierDto {
        ModifierDto::Weapon(WeaponModifierDto::new(shots, strength, armour_penetration))
    }

    /// Convert the ModifierDto into a Modifier
    ///
    /// # Return
    ///
    /// Modifier: The newly created Modifier
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

    /// Convert a Modifier into a ModifierDto
    ///
    /// # Parameters
    ///
    /// modifier (Modifier): The modifier to convert
    ///
    /// # Return
    ///
    /// ModifierDto: The newly created ModifierDto
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_modifier_dto_global() {
        let modifier = crate::modifier::Modifier::new_global(1, 2, 3);
        let modifier_dto = crate::dto::modifier::ModifierDto::dehydrate(&modifier);
        let modifier2 = modifier_dto.hydrate();
        assert_eq!(modifier, modifier2);
    }

    #[test]
    fn test_modifier_dto_defensive() {
        let modifier = crate::modifier::Modifier::new_defensive(1, 2, 3, 4, 5);
        let modifier_dto = crate::dto::modifier::ModifierDto::dehydrate(&modifier);
        let modifier2 = modifier_dto.hydrate();
        assert_eq!(modifier, modifier2);
    }

    #[test]
    fn test_modifier_dto_offensive() {
        let modifier = crate::modifier::Modifier::new_offensive(1, 2, 3, 4, 5);
        let modifier_dto = crate::dto::modifier::ModifierDto::dehydrate(&modifier);
        let modifier2 = modifier_dto.hydrate();
        assert_eq!(modifier, modifier2);
    }

    #[test]
    fn test_modifier_dto_shooting_weapon() {
        let modifier = crate::modifier::Modifier::new_weapon(Some(1), 2, 3);
        let modifier_dto = crate::dto::modifier::ModifierDto::dehydrate(&modifier);
        let modifier2 = modifier_dto.hydrate();
        assert_eq!(modifier, modifier2);
    }

    #[test]
    fn test_modifier_dto_melee_weapon() {
        let modifier = crate::modifier::Modifier::new_weapon(None, 2, 3);
        let modifier_dto = crate::dto::modifier::ModifierDto::dehydrate(&modifier);
        let modifier2 = modifier_dto.hydrate();
        assert_eq!(modifier, modifier2);
    }
}
