#[path = "./single_units/mods.rs"]
mod single_units;

pub use single_units::unit::Status;
pub use single_units::unit::Unit;
pub use single_units::unit::ChargeReaction;
pub use single_units::unit::Stats;
pub use single_units::unit::Modifier;
pub use single_units::charge::global_values::charge::*;


impl Unit {

     // Compute the minimum value on a dice roll to hit a target.
    fn computeRollToHit(offensive: i32, defense: i32) -> u8 {
        let difference: i32 = offensive - defense;

        match difference {
            i32::MIN..=-7 => 6,
            -7..=-3 => 5,
            -3..=1 => 4,
            1..=4 => 3,
            _ => 2,
        }
    }

     // Compute the minimum value on a dice roll to wound a target.
    fn computeRollToWound(force: i32, resilience: i32) -> u8 {
        let difference: i32 = force - resilience;

        match difference {
            i32::MIN..=-2 => 6,
            -1..=0 => 5,
            0..=1 => 4,
            1..=2 => 3,
            _ => 2,
        }
    }

     // Estimate the damage that the unit will do to the target unit.
    fn estimateMeleeDamage(&mut self, target: &mut Unit) -> f64 {
         // The minimum value on a dice roll to touch the enemy
        let mut minimumToHit: u8 = Unit::computeRollToHit(self.stats.offensive.try_into().unwrap(), target.stats.defense.try_into().unwrap());
         // The minimum value on a dice roll to wound the enemy
        let mut minimumToWound: u8 = Unit::computeRollToWound(self.stats.force.try_into().unwrap(), target.stats.resilience.try_into().unwrap());
         // The number of attack that touched the enemy
        let mut nbHit: f64 = 0.0;
         // The number of attack that wounded the enemy
        let mut nbWound: f64 = 0.0;
         // The number of attack that the enemy can save
        let mut foeSave: f64 = target.estimateArmourSave(self);
         // The number of face on the dices for hit rolling
        let mut nb_hit_face: isize = DEFAULT_DICE;
         // The number of dices to roll while hitting
        let mut nb_hit_roll: isize = self.stats.attack;
         // The number of face on the dices for wound rolling
        let mut nb_wound_face: isize = DEFAULT_DICE;

        let wound:String = "wound".to_owned();
        let hit:String = "hit".to_owned();

         // We will here determines the number of dices to roll and there number of faces. Both to hit and wound
        for n in &self.modifiers {
            if (n.requirements.contains(&hit)) {
                nb_hit_face += n.nb_faces;
                nb_hit_roll += n.nb_dice;
            }
            if (n.requirements.contains(&wound)) {
                nb_wound_face += n.nb_faces;
            }
        }

        // With (nb_hit_face - minimumToHit + 1) / nb_hit_face we compute the probability to hit
        // we then time this probability with the number of attack to obtains, the mean number of hit done.
        nbHit = ((nb_hit_roll as f64) * ( ((nb_hit_face as f64 - (minimumToHit as f64) + 1 as f64) / nb_hit_face as f64)) as f64 ).abs().ceil();

        // With (nb_wound_face - minimumToWound + 1) / nb_wound_face we compute the probability to wound
        // we then time this probability with the number of attack that hit to obtains, the mean number of wound done.
        nbWound = (nbHit * ( ((nb_wound_face as f64- (minimumToWound as f64) + 1.0) / nb_wound_face as f64)) as f64 ).abs().ceil();
        
        return (nbWound - (nbWound * foeSave).ceil()).abs();
    }

    // We estimate here the damage that the unit will block form an attack
    fn estimateArmourSave(&mut self, target: & Unit) -> f64 {
         // The number of faces on the dices
        let mut success_faces  = DEFAULT_DICE;
         // The minimum to save an attack
        let mut armourSave = 0;
         // The probability to save an attack
        let mut armour_stat: f64 = 0.0;
         // The probabibility to save an attack with aegis
        let mut aegis_stat: f64 = 0.0;

        let save:String = "save".to_owned();
        
        for n in &self.modifiers {
            if (n.requirements.contains(&save)) {
                success_faces += n.nb_faces;
                self.stats.armour += n.stat.armour;
            }
        }

        armourSave = ARMOUR_SAVE_THRESHOLD - (self.stats.armour - target.stats.armour_penetration);
        
        // size of dice - number of winning faces + 1 / size of dice = probability to win
        //DEBUG
        let mut temp:f64 = (((success_faces as f64 - armourSave as f64 + 1 as f64) / success_faces as f64) as f64);
        armour_stat = temp.abs().ceil(); //as f64;
        aegis_stat = (((success_faces as f64 - self.stats.aegis as f64 + 1 as f64) / success_faces as f64) as f64).abs().ceil();

        if (self.stats.aegis == 0) {
             // The probability to save an attack with no aegis
            return armour_stat
        } else {
             // Here we compute the probability to save an attack with aegis OR without aegis
            // return armour_stat + aegis_stat - (aegis_stat * armour_stat);
            return armour_stat + aegis_stat + (aegis_stat * armour_stat);
        }
        // return 0.0;
    }

     // Charge the target unit.
    pub fn charge(&mut self, target: & Unit) {
        self.status = Status::CHARGE;
    }

     // This function is used to determined the appropriate reaction in case of charge
    pub fn chargeReaction(&mut self, target: &mut Unit) ->  ChargeReaction {
        let far = "far".to_owned();
         // If we arez already in a battle, we don't have other choices than hold the positions
        if self.status as i32 ==  Status::FLEE as i32 {
            return  ChargeReaction::HOLD;
        }
         // If we have range weapon, we can attack the target
        if self
            .modifiers
            .iter()
            .find(|x| x.requirements.contains(&far))
            != None 
        {
            println!("enemy is in range");
            return  ChargeReaction::SHOOT;
            // If we can injure the opponent more than him, we can attack him
        } else if self.estimateMeleeDamage(target) > target.estimateMeleeDamage(self) {
            println!("{} dmg, We hold", self.estimateMeleeDamage(target));
            return  ChargeReaction::HOLD;
            // Or else we run away
        } else {
            println!("{} dmg we do", self.estimateMeleeDamage(target));
            println!("{} dmg enemy do", target.estimateMeleeDamage(self));
            println!("{}", "Run away");
            return  ChargeReaction::RUN;
        }
    }
}
