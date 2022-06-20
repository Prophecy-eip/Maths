use crate::charge_reaction::{Stats, Modifier, Unit, Status};

pub(crate) mod charge_reaction;

fn main() {
   
    // WEAK INITIALIZATION OF UNIT    
    let _stats_vectors_weak = Stats {
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
        force: 0,
    };

    let _modifiers_simple_weak = Modifier {
        requirements: Vec::new(),
        stat: _stats_vectors_weak,
        nb_dice: 0,
        nb_faces: 0,
    };
    
    let mut _modifiers_vectors_weak = Vec::new();
    _modifiers_vectors_weak.push(_modifiers_simple_weak);


    // STRONG INITIALIZATION OF UNIT
    let _stats_vectors_strong = Stats {
        advance_rate: 8,
        march_rate: 8,
        discipline: 8,
        health_point: 8,
        defense: 8,
        resilience: 8,
        armour: 8,
        aegis: 8,
        attack: 8,
        offensive: 8,
        strength: 8,
        armour_penetration: 8,
        agility: 8,
        force: 8,
    };

    let _modifiers_simple_strong = Modifier {
        requirements: Vec::new(),
        // uncomment the line beneath and commit the line above to get the first reacting, considering you are using this modifier in the primary Unit you'll call.
        // requirements: vec!["far".to_string()],
        stat: _stats_vectors_strong,
        nb_dice: 0,
        nb_faces: 0,
    };

    let mut _modifiers_vectors_strong = Vec::new();
    _modifiers_vectors_strong.push(_modifiers_simple_strong);

    //Basic position
    let _pos = (0, 0);

    let mut a = Unit {
        stats: _stats_vectors_strong,
        modifiers: _modifiers_vectors_strong,
        position: _pos,
        status: Status::IDLE,
    };

    let mut b = Unit {
        stats: _stats_vectors_weak,
        modifiers: _modifiers_vectors_weak,
        position: _pos,
        status: Status::IDLE,
    };

    // time to try the chargeReaction & print the result
    let charge_reaction = a.chargeReaction(&mut b);
    println!("{:?}", charge_reaction);

    return;
    
}