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

/// Struct used to represent the modification granted to dice rolls out of rust code
///
/// # Attributes
///
/// dice_size (isize): The size of the dice
///
/// dice_number (isize): The number of dice
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize, Copy)]
pub struct DiceModifierDto {
    dice_size: isize,
    dice_number: isize,
}

impl DiceModifierDto {
    /// Create a new DiceModifierDto
    ///
    /// # Parameters
    ///
    /// dice_size (isize): The size of the dice
    ///
    /// dice_number (isize): The number of dice
    ///
    /// # Return
    ///
    /// DiceModifierDto: The newly created DiceModifierDto
    pub fn new(dice_size: isize, dice_number: isize) -> DiceModifierDto {
        DiceModifierDto {
            dice_size,
            dice_number,
        }
    }

    /// Hydrate a DiceModifierDto into a DiceModifier
    ///
    /// # Return
    ///
    /// DiceModifier: The hydrated DiceModifier
    pub fn hydrate(&self) -> crate::modifier::DiceModifier {
        crate::modifier::DiceModifier::new(self.dice_size, self.dice_number)
    }

    /// Dehydrate a DiceModifier into a DiceModifierDto
    ///
    /// # Parameters
    ///
    /// dice_modifier (&crate::modifier::DiceModifier): The DiceModifier to dehydrate
    ///
    /// # Return
    ///
    /// DiceModifierDto: The dehydrated DiceModifierDto
    pub fn dehydrate(dice_modifier: &crate::modifier::DiceModifier) -> DiceModifierDto {
        DiceModifierDto::new(
            dice_modifier.get_dice_size(),
            dice_modifier.get_dice_number(),
        )
    }
}

/// Enum used to represent a Statsmodifier outside of rust code
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
pub enum StatsModifierDto {
    Global(GlobalModifierDto),
    Defensive(DefensiveModifierDto),
    Offensive(OffensiveModifierDto),
    Weapon(WeaponModifierDto),
}

/// Struct used to represent a modifier outside of rust code
///
/// # Attributes
///
/// dice_modifier (Option<DiceModifierDto>): The dice modifier
///
/// stats_modifier (Option<StatsModifierDto>): The stats modifier
///
/// stats_replaced (bool): Whether the stats are replaced or not
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub struct ModifierDto {
    dice_modifier: Option<DiceModifierDto>,
    stats_modifier: StatsModifierDto,
    stats_replaced: bool,
}

impl ModifierDto {
    /// Convert the StatsModifierDto into a StatsModifier
    ///
    /// # Return
    ///
    /// Modifier: The newly created Modifier
    pub fn hydrate(&self) -> crate::modifier::Modifier {
        crate::modifier::Modifier::new(
            self.dice_modifier
                .as_ref()
                .map(|dice_modifier| dice_modifier.hydrate()),
            self.stats_modifier.hydrate(),
            self.stats_replaced,
        )
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
        ModifierDto {
            dice_modifier: modifier
                .get_dice_modifier()
                .map(|dice_modifier| DiceModifierDto::dehydrate(&dice_modifier)),
            stats_modifier: StatsModifierDto::dehydrate(&modifier.get_stats_modifier()),
            stats_replaced: modifier.get_stats_replaced(),
        }
    }
}

impl StatsModifierDto {
    /// Create a new StatsModifierDto
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
    /// StatsModifierDto: The newly created StatsModifierDto
    pub fn new_global_dto(advance: isize, march: isize, discipline: isize) -> StatsModifierDto {
        StatsModifierDto::Global(GlobalModifierDto::new(advance, march, discipline))
    }

    /// Create a new StatsModifierDto
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
    /// StatsModifierDto: The newly created StatsModifierDto
    pub fn new_defensive_dto(
        health_points: isize,
        defense: isize,
        resilience: isize,
        armour: isize,
        aegis: isize,
    ) -> StatsModifierDto {
        StatsModifierDto::Defensive(DefensiveModifierDto::new(
            health_points,
            defense,
            resilience,
            armour,
            aegis,
        ))
    }

    /// Create a new StatsModifierDto
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
    /// StatsModifierDto: The newly created StatsModifierDto
    pub fn new_offensive_dto(
        attack: isize,
        offensive: isize,
        strength: isize,
        armour_penetration: isize,
        agility: isize,
    ) -> StatsModifierDto {
        StatsModifierDto::Offensive(OffensiveModifierDto::new(
            attack,
            offensive,
            strength,
            armour_penetration,
            agility,
        ))
    }

    /// Create a new StatsModifierDto
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
    /// StatsModifierDto: The newly created StatsModifierDto
    pub fn new_weapon_dto(
        shots: Option<isize>,
        strength: isize,
        armour_penetration: isize,
    ) -> StatsModifierDto {
        StatsModifierDto::Weapon(WeaponModifierDto::new(shots, strength, armour_penetration))
    }

    /// Convert the StatsModifierDto into a StatsModifier
    ///
    /// # Return
    ///
    /// StatsModifier: The newly created StatsModifier
    pub fn hydrate(&self) -> crate::modifier::StatsModifier {
        match self {
            StatsModifierDto::Global(global) => crate::modifier::StatsModifier::new_global(
                global.advance,
                global.march,
                global.discipline,
            ),
            StatsModifierDto::Defensive(defensive) => {
                crate::modifier::StatsModifier::new_defensive(
                    defensive.health_points,
                    defensive.defense,
                    defensive.resilience,
                    defensive.armour,
                    defensive.aegis,
                )
            }
            StatsModifierDto::Offensive(offensive) => {
                crate::modifier::StatsModifier::new_offensive(
                    offensive.attack,
                    offensive.offensive,
                    offensive.strength,
                    offensive.armour_penetration,
                    offensive.agility,
                )
            }
            StatsModifierDto::Weapon(weapon) => crate::modifier::StatsModifier::new_weapon(
                Some(3),
                weapon.shots,
                weapon.strength,
                weapon.armour_penetration,
            ),
        }
    }

    /// Convert a statModifier into a StatsModifierDto
    ///
    /// # Parameters
    ///
    /// modifier (StatsModifier): The statsModifier to convert
    ///
    /// # Return
    ///
    /// StatsModifierDto: The newly created StatsModifierDto
    pub fn dehydrate(stat_modifier: &crate::modifier::StatsModifier) -> StatsModifierDto {
        match stat_modifier {
            crate::modifier::StatsModifier::Global(global) => StatsModifierDto::new_global_dto(
                global.get_advance(),
                global.get_march(),
                global.get_discipline(),
            ),
            crate::modifier::StatsModifier::Defensive(defensive) => {
                StatsModifierDto::new_defensive_dto(
                    defensive.get_health_points(),
                    defensive.get_defense(),
                    defensive.get_resilience(),
                    defensive.get_armour(),
                    defensive.get_aegis(),
                )
            }
            crate::modifier::StatsModifier::Offensive(offensive) => {
                StatsModifierDto::new_offensive_dto(
                    offensive.get_attack(),
                    offensive.get_offensive(),
                    offensive.get_strength(),
                    offensive.get_armour_penetration(),
                    offensive.get_agility(),
                )
            }
            crate::modifier::StatsModifier::Weapon(weapon) => StatsModifierDto::new_weapon_dto(
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
        let modifier = crate::modifier::Modifier::new_global(1, 2, 3, None, false);
        let modifier_dto = crate::dto::modifier::ModifierDto::dehydrate(&modifier);
        let modifier2 = modifier_dto.hydrate();
        assert_eq!(modifier, modifier2);
    }

    #[test]
    fn test_modifier_dto_defensive() {
        let modifier = crate::modifier::Modifier::new_defensive(1, 2, 3, 4, 5, None, false);
        let modifier_dto = crate::dto::modifier::ModifierDto::dehydrate(&modifier);
        let modifier2 = modifier_dto.hydrate();
        assert_eq!(modifier, modifier2);
    }

    #[test]
    fn test_modifier_dto_offensive() {
        let modifier = crate::modifier::Modifier::new_offensive(1, 2, 3, 4, 5, None, false);
        let modifier_dto = crate::dto::modifier::ModifierDto::dehydrate(&modifier);
        let modifier2 = modifier_dto.hydrate();
        assert_eq!(modifier, modifier2);
    }

    #[test]
    fn test_modifier_dto_shooting_weapon() {
        let modifier = crate::modifier::Modifier::new_weapon(Some(3), Some(1), 2, 3, None, false);
        let modifier_dto = crate::dto::modifier::ModifierDto::dehydrate(&modifier);
        let modifier2 = modifier_dto.hydrate();
        assert_eq!(modifier, modifier2);
    }

    #[test]
    fn test_modifier_dto_melee_weapon() {
        let modifier = crate::modifier::Modifier::new_weapon(Some(3), None, 2, 3, None, false);
        let modifier_dto = crate::dto::modifier::ModifierDto::dehydrate(&modifier);
        let modifier2 = modifier_dto.hydrate();
        assert_eq!(modifier, modifier2);
    }
}
