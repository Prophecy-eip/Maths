impl math::Unit {
    fn charge(&self, target: &math::Unit) {
        self.status = math::Status::CHARGE;
    }

    fn estimateMeleeDamage(&self, target: &math::Unit) {
        let hitNumber = self.stats.attack;
        let minimumToTouch = 6;
        let minimumToWound = 6;
        let nbTouch = 0;
        let nbWound = 0;
        let foeSave = target.estimateArmourSave(self);

        for n in self.modifier {
            hitNumber += n.stat.attack;
        }
        for n in charge::THROWSTOHIT {
            if n.0.contains(self.stat.offensive - target.stat.defense) {
                minimumToTouch = n.1;
                break;
            }
        }
        for n in charge::THROWSTOWOUND {
            if n.0.contains(self.stat.force - target.stat.resilience) {
                minimumToWound = n.1;
                break;
            }
        }
        nbTouch = (hitNumber * ((SIZE_DICE - minimumToTouch + 1) / SIZE_DICE)).ceil();
        nbWound = (nbTouch * ((SIZE_DICE - minimumToWound + 1) / SIZE_DICE)).ceil();
        return nbWound - (nbWound * foeSave).ceil();
    }

    fn estimateArmourSave(&self, target: &math::Unit) -> f64 {
        let armourSave =
            ARMOUR_SAVE_THRESHOLD - (self.stat.armour - target.stat.armour_penetration);
        if (self.stat.aegis == 0) {
            return (SIZE_DICE - armourSave + 1) / SIZE_DICE;
        }
        return ((SIZE_DICE - armourSave + 1) / SIZE_DICE)
            * ((SIZE_DICE - self.stat.aegis + 1) / SIZE_DICE);
    }

    fn chargeReaction(&self, foe: &math::Unit) {
        if (self
            .modifiers
            .iter()
            .find(|x| x.requirements.contains("far"))
            != None)
        {
            println!("Shoot them as if they were lil rabbits!");
        } else if self.estimateMeleeDamage(foe) > foe.estimateMeleeDamage(self) {
            println!("HOLD THE POSITION!");
        } else {
            println!("RUN FOR YOUR LIFE!");
        }
    }
}

fn main() {
    let a: math::Unit;
    let b: math::Uint;

    a.chargeReaction(&b);
}
