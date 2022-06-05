impl math::Unit {
    fn charge(&self, target: &math::Unit) {
        let hitNumber = self.stats.attack;
        let minimumToTouch = 6;

        for n in self.modifier {
            hitNumber += n.stat.attack;
        }
        for n in charge::THROWSTOHIT {
            if n.0.contains(self.stat.offensive - target.stat.defense) {
                minimumToTouch = n.1;
                break;
            }
        }
    }

    fn estimateMeleeDamage(&self, target: &math::Unit) {
        let hitNumber = self.stats.attack;
        let minimumToTouch = 6;
        let minimumToWound = 6;
        let nbTouch = 0;
        let nbWound = 0;

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
        nbTouch = (hitNumber * ((SIZE_DICE - minimumToTouch) / SIZE_DICE)).ceil();
        nbWound = (nbTouch * ((SIZE_DICE - minimumToWound) / SIZE_DICE)).ceil();
    }
}
