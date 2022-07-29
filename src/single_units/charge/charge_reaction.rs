pub use crate::single_units::charge::global_values::*;

pub use crate::single_units::unit::{ChargeReaction, Modifier, Stats, Status, Unit};

impl Unit {
    /// # Compute the minimum value on a dice roll to hit a target.
    ///
    /// ## Parameters
    /// offensive (i32): The offensive value of the attacker
    ///
    /// defense (i32): The defense value of the defender
    ///
    /// ## Return
    /// i8: Value to even up or overtake to hit the defender
    fn compute_roll_to_hit(offensive: i32, defense: i32) -> u8 {
        let difference: i32 = offensive - defense;

        match difference {
            i32::MIN..=-8 => 6,
            -7..=-4 => 5,
            -3..=0 => 4,
            1..=3 => 3,
            _ => 2,
        }
    }

    /// # Compute the minimum value on a dice roll to wound a target.
    ///
    /// ## Parameters
    /// strength (i32): The strength value of the attacker
    ///
    /// resilience (i32): The resilience value of the defenser
    ///
    /// ## Return
    /// u8: Value to even up or overtake to wound the defender
    fn compute_roll_to_wound(strength: i32, resilience: i32) -> u8 {
        let difference: i32 = strength - resilience;

        match difference {
            i32::MIN..=-2 => 6,
            -1 => 5,
            0 => 4,
            1 => 3,
            _ => 2,
        }
    }

    /// # Estimate the damage that the unit will do to the target unit.
    ///
    /// ## Parameters
    /// target (&mut Unit): The other unit to fight with
    ///
    /// ## Return
    /// f64: Something with a comma
    fn estimate_melee_damage(&mut self, target: &mut Unit) -> f64 {
        let minimum_to_hit: u8 = Unit::compute_roll_to_hit(
            self.stats.offensive.try_into().unwrap(),
            target.stats.defense.try_into().unwrap(),
        );
        let minimum_to_wound: u8 = Unit::compute_roll_to_wound(
            self.stats.strength.try_into().unwrap(),
            target.stats.resilience.try_into().unwrap(),
        );
        let nb_saves: f64 = target.estimate_armour_save(self);
        // The number of face on the dices for hit rolling
        let mut nb_hit_face: isize = DEFAULT_DICE;
        // The number of dices to roll while hitting
        let mut nb_hit_roll: isize = self.stats.attack;
        // The number of face on the dices for wound rolling
        let mut nb_wound_face: isize = DEFAULT_DICE;

        let wound: String = "wound".to_owned();
        let hit: String = "hit".to_owned();

        // We will here determines the number of dices to roll and there number of faces. Both to hit and wound
        for n in &self.modifiers {
            if n.requirements.contains(&hit) {
                nb_hit_face += n.nb_faces;
                nb_hit_roll += n.nb_dice;
            }
            if n.requirements.contains(&wound) {
                nb_wound_face += n.nb_faces;
            }
        }

        // With (nb_hit_face - minimumToHit + 1) / nb_hit_face we compute the probability to hit
        // we then time this probability with the number of attack to obtains, the mean number of hit done.
        let nb_hit: f64 = ((nb_hit_roll as f64)
            * ((nb_hit_face as f64 - (minimum_to_hit as f64) + 1.0) / nb_hit_face as f64))
            .abs()
            .ceil();

        // With (nb_wound_face - minimumToWound + 1) / nb_wound_face we compute the probability to wound
        // we then time this probability with the number of attack that hit to obtains, the mean number of wound done.
        let nb_wound: f64 = (nb_hit
            * ((nb_wound_face as f64 - (minimum_to_wound as f64) + 1.0) / nb_wound_face as f64)
                as f64)
            .abs()
            .ceil();

        let final_result: f64 = nb_wound - (nb_wound * nb_saves).ceil();
        if final_result > 0.0 {
            final_result
        } else {
            0.0
        }
    }

    /// # We estimate here the damage that the unit will block form an attack
    ///
    /// ## Parameters
    /// target (Unit): The other unit
    ///
    /// ## Return
    /// f64: Something with a comma in it
    fn estimate_armour_save(&mut self, target: &Unit) -> f64 {
        // The number of faces on the dices
        let mut success_faces = DEFAULT_DICE;

        for n in &self.modifiers {
            if n.requirements.contains(&("save".to_owned())) {
                success_faces += n.nb_faces;
                self.stats.armour += n.stat.armour;
            }
        }

        let mut armour_save: isize =
            ARMOUR_SAVE_THRESHOLD - (self.stats.armour - target.stats.armour_penetration);
        armour_save = if armour_save < 0 { 0 } else { armour_save };

        // size of dice - number of winning faces + 1 / size of dice = probability to win
        //DEBUG
        let mut temp: f64 =
            (success_faces as f64 - armour_save as f64 + 1.0) / success_faces as f64;
        temp = if temp < 0.0 { 0.0 } else { temp };
        let armour_stat = temp.ceil(); //as f64;
        let aegis_stat = ((success_faces as f64 - self.stats.aegis as f64 + 1.0)
            / success_faces as f64)
            .abs()
            .ceil();

        if self.stats.aegis == 0 {
            // The probability to save an attack with no aegis
            armour_stat
        } else {
            // Here we compute the probability to save an attack with aegis OR without aegis
            // return armour_stat + aegis_stat - (aegis_stat * armour_stat);
            armour_stat + aegis_stat + (aegis_stat * armour_stat)
        }
        // return 0.0;
    }

    /// # Charge the target unit.
    ///
    /// ## Parameters
    /// **UNUSED** _target (Unit): The other unit
    pub fn charge(&mut self, _target: &Unit) {
        self.status = Status::CHARGE;
    }

    /// # This function is used to determined the appropriate reaction in case of charge
    ///
    /// ## Parameters
    /// target (&mut Unit): The other unit
    ///
    /// ## Return
    /// ChargeReaction: The charge reaction of the other unit
    pub fn charge_reaction(&mut self, target: &mut Unit) -> ChargeReaction {
        let far = "far".to_owned();
        // If we are already in a battle, we don't have other choices than hold the positions
        if self.status as i32 == Status::FLEE as i32 {
            return ChargeReaction::HOLD;
        }
        // If we have range weapon, we can attack the target
        if self
            .modifiers
            .iter()
            .find(|x| x.requirements.contains(&far))
            != None
        {
            ChargeReaction::SHOOT
        } else if self.estimate_melee_damage(target) >= target.estimate_melee_damage(self) {
            println!(
                "self: {}, other: {}",
                self.estimate_melee_damage(target),
                target.estimate_melee_damage(self)
            );
            ChargeReaction::HOLD
        } else {
            ChargeReaction::RUN
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Unit;

    fn generate_unit(strength: String) -> Unit {
        let stat = if strength == "weak" { 0 } else { 9 };
        let _stats = crate::single_units::unit::Stats {
            advance_rate: stat,
            march_rate: stat,
            discipline: stat,
            health_point: stat,
            defense: stat,
            resilience: stat,
            armour: stat,
            aegis: 0,
            attack: stat,
            offensive: stat,
            strength: stat,
            armour_penetration: stat,
            agility: stat,
        };
        let _modifier_stat = crate::single_units::unit::Stats {
            advance_rate: 0,
            march_rate: 0,
            discipline: 0,
            health_point: 0,
            defense: 0,
            resilience: 0,
            armour: 0,
            aegis: 0,
            attack: 0,
            offensive: 0,
            strength: 0,
            armour_penetration: 0,
            agility: 0,
        };

        let _modifier = crate::single_units::unit::Modifier {
            requirements: Vec::new(),
            stat: _modifier_stat,
            nb_dice: 0,
            nb_faces: 0,
        };
        let mut _modifiers = Vec::new();
        _modifiers.push(_modifier);

        let unit = crate::single_units::unit::Unit {
            stats: _stats,
            modifiers: _modifiers,
            position: (0, 0),
            status: crate::single_units::unit::Status::IDLE,
        };
        return unit;
    }

    #[test]
    fn compute_roll_to_hit() {
        let sup = crate::single_units::unit::Unit::compute_roll_to_hit(5, 3);
        let inf = crate::single_units::unit::Unit::compute_roll_to_hit(7, 10);
        assert_eq!(sup, 3);
        assert_eq!(inf, 4);
    }

    #[test]
    fn compute_roll_to_wound() {
        let sup = crate::single_units::unit::Unit::compute_roll_to_wound(5, 3);
        let inf = crate::single_units::unit::Unit::compute_roll_to_wound(7, 10);
        assert_eq!(sup, 2);
        assert_eq!(inf, 6);
    }

    #[test]
    fn estimate_melee_damage() {
        let mut unit = generate_unit("weak".to_string());
        let mut target = generate_unit("strong".to_string());

        let damage_taken = unit.estimate_melee_damage(&mut target);
        assert_eq!(damage_taken, 0.0f64);
    }

    #[test]
    fn estimate_armour_save() {
        let mut unit = generate_unit("weak".to_string());
        let mut target = generate_unit("strong".to_string());

        let damage_saved = unit.estimate_armour_save(&mut target);
        assert_eq!(damage_saved, 0.0f64);
    }

    #[test]
    fn charge() {
        let mut unit = generate_unit("weak".to_string());
        let mut target = generate_unit("strong".to_string());

        unit.charge(&mut target);
        match unit.status {
            crate::single_units::unit::Status::CHARGE => assert_eq!(true, true),
            _ => panic!("Unit should be charging"),
        }
    }

    #[test]
    fn charge_reaction() {
        let mut unit = generate_unit("weak".to_string());
        let mut target = generate_unit("strong".to_string());
        unit.stats.advance_rate = 8;
        unit.stats.march_rate = 8;
        unit.stats.discipline = 8;
        unit.stats.health_point = 8;
        unit.stats.defense = 8;
        unit.stats.resilience = 8;
        unit.stats.armour = 8;
        unit.stats.aegis = 0;
        unit.stats.attack = 8;
        unit.stats.offensive = 8;
        unit.stats.strength = 8;
        unit.stats.armour_penetration = 8;
        unit.stats.agility = 8;

        let mut result = target.charge_reaction(&mut unit);
        match result {
            crate::single_units::unit::ChargeReaction::HOLD => assert_eq!(true, true),
            _ => panic!("Unit should be holding"),
        }

        result = unit.charge_reaction(&mut target);
        match result {
            crate::single_units::unit::ChargeReaction::RUN => assert_eq!(true, true),
            val => {
                println!("val: {:?}", val);
                panic!("Unit should be fleeing");
            }
        }
    }
}
