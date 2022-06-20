use crate::charge_reaction::{Stats, Modifier, Unit, Status};

pub(crate) mod charge_reaction;

fn main() {
    //make two variables of type Unit a and b and use the Impl Unit for them
    
    let _stats_vectors_0 = Stats {
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

    let _stats_vectors_8 = Stats {
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

    let _modifiers_simple_0 = Modifier {
        requirements: Vec::new(),
        stat: _stats_vectors_0,
        nb_dice: 0,
        nb_faces: 0,
    };

    let _modifiers_simple_8 = Modifier {
        requirements: Vec::new(),
        stat: _stats_vectors_8,
        nb_dice: 0,
        nb_faces: 0,
    };

    // make a vectors of modifiers and initialize it with the simple modifier
    let mut _modifiers_vectors_8 = Vec::new();
    _modifiers_vectors_8.push(_modifiers_simple_8);

    //make a vector of modifiers for B
    let mut _modifiers_vectors_0 = Vec::new();
    _modifiers_vectors_0.push(_modifiers_simple_0);

    let _pos = (0, 0);

    let mut a = Unit {
        stats: _stats_vectors_0,
        modifiers: _modifiers_vectors_0,
        position: _pos,
        status: Status::IDLE,
    };

    let mut b = Unit {
        stats: _stats_vectors_8,
        modifiers: _modifiers_vectors_8,
        position: _pos,
        status: Status::IDLE,
    };

    // time to try the chargeReaction
    let charge_reaction = a.chargeReaction(&mut b);
    println!("{:?}", charge_reaction);

    return;
    
}