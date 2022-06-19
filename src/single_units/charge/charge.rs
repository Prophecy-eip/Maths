impl math::Unit {
    /// Compute the minimum value on a dice roll to hit a target.
    fn computeRollToHit(offensive: usize, defense: usize) -> u8 {
        let difference: i32 = offensive - defense;

        match difference {
            i32::MIN..-7 => 6,
            -7..-3 => 5,
            -3..1 => 4,
            1..4 => 3,
            _ => 2,
        }
    }

    /// Compute the minimum value on a dice roll to wound a target.
    fn computeRollToWound(force: usize, resilience: usize) -> u8 {
        let difference: i32 = force - resilience;

        match difference {
            i32::MIN..-2 => 6,
            -1..0 => 5,
            0..1 => 4,
            1..2 => 3,
            _ => 2,
        }
    }

    /// Estimate the damage that the unit will do to the target unit.
    fn estimateMeleeDamage(&self, target: &math::Unit) -> usize {
        /// The minimum value on a dice roll to touch the enemy
        let mut minimumToHit: usize = computeRollToHit(self.stats.offensive, target.stats.defense);
        /// The minimum value on a dice roll to wound the enemy
        let mut minimumToWound: usize = computeRollToWound(self.stats.force, target.stats.resilience);
        /// The number of attack that touched the enemy
        let mut nbHit: usize = 0;
        /// The number of attack that wounded the enemy
        let mut nbWound: usize = 0;
        /// The number of attack that the enemy can save
        let mut foeSave: usize = target.estimateArmourSave(self);
        /// The number of face on the dices for hit rolling
        let mut nb_hit_face: usize = DEFAULT_DICE;
        /// The number of dices to roll while hitting
        let mut nb_hit_roll: usize = self.stats.attack;
        /// The number of face on the dices for wound rolling
        let mut nb_wound_face: usize = DEFAULT_DICE;

        /// We will here determines the number of dices to roll and there number of faces. Both to hit and wound
        for n in self.modifier {
            if (n.requirements.contains("hit")) {
                nb_hit_face += n.faces;
                nb_hit_roll += n.nb_dice;
            }
            if (n.requirements.contains("wound")) {
                nb_wound_face += n.faces;
            }
        }

        /// With (nb_hit_face - minimumToHit + 1) / nb_hit_face we compute the probability to hit
        /// we then time this probability with the number of attack to obtains, the mean number of hit done.
        nbHit = (nb_hit_roll * ((nb_hit_face - minimumToHit + 1) / nb_hit_face)).ceil();
        /// With (nb_wound_face - minimumToWound + 1) / nb_wound_face we compute the probability to wound
        /// we then time this probability with the number of attack that hit to obtains, the mean number of wound done.
        nbWound = (nbHit * ((nb_wound_face - minimumToWound + 1) / nb_wound_face)).ceil();
        return nbWound - (nbWound * foeSave).ceil();
    }

    /// We estimate here the damage that the unit will block form an attack
    fn estimateArmourSave(&self, target: &math::Unit) -> f64 {
        /// The number of faces on the dices
        let success_faces: usize = DEFAULT_DICE;
        /// The minimum to save an attack
        let mut armourSave: usize = 0;
        /// The probability to save an attack
        let mut armour_stat: f64 = 0;
        /// The probabibility to save an attack with aegis
        let mut aegis_stat: f64 = 0;
            
        
        for n in self.modifier {
            if (n.requirements.contains("save")) {
                success_faces += n.faces;
                self.stat.armour += n.armour;
            }
        }

        armourSave = ARMOUR_SAVE_THRESHOLD - (self.stat.armour - target.stat.armour_penetration);
        /// size of dice - number of winning faces + 1 / size of dice = probability to win
        armour_stat = (success_faces - armourSave + 1) / success_faces;
        aegis_stat = (success_faces - self.stat.aegis + 1) / success_faces; 

        if (self.stat.aegis == 0) {
            /// The probability to save an attack with no aegis
            return armour_stat
        } else {
            /// Here we compute the probability to save an attack with aegis OR without aegis
            return armour_stat + aegis_stat - (aegis_stat * armour_stat);
        }
        return 0.0;
    }

    /// Charge the target unit.
    pub fn charge(&self, target: &math::Unit) {
        self.status = math::Status::CHARGE;
    }

    /// This function is used to determined the appropriate reaction in case of charge
    pub fn chargeReaction(&self, target: &math::Unit) -> math::ChargeReaction {
        /// If we arez already in a battle, we don't have other choices than hold the positions
        if (self.status == math::Status::HOLD) {
            return math::ChargeReaction::HOLD;
        }
        /// If we have range weapon, we can attack the target
        if (self
            .modifiers
            .iter()
            .find(|x| x.requirements.contains("far"))
            != None)
        {
            return math::ChargeReaction::SHOOT;
            /// If we can injure the opponent more than him, we can attack him
        } else if self.estimateMeleeDamage(foe) > foe.estimateMeleeDamage(self) {
            return math::ChargeReaction::HOLD;
            /// Or else we run away
        } else {
            return math::ChargeReaction::RUN;
        }
    }
}
