/// This structure represent all the stats that a unit can have.
struct Stats {
    /// The distance the model can Advance Move in inches per turn.
    advance_rate: usize,
    /// The distance the model can March Move in inches per turn.
    march_rate: usize,
    /// Represents the model’s overall morale, bravery and a unit’s ability to quickly reorganise during battle.
    discipline: usize,
    /// When the model loses this many Health Points, it is removed as a casualty
    health_point: usize,
    /// How well the model avoids being hit in melee.
    defense: usize,
    /// How easily the model withstands blows.
    resilience: usize,
    /// The Armour of the model.
    armour: usize,
    /// A special protection for the model, the final defense after the model’s Armour.
    aegis: usize,
    /// The number of Melee Attacks the model has.
    attack: usize,
    /// How good the model is at scoring hits in melee.
    offensive: usize,
    /// How easily the model can wound enemy models in melee.
    strength: usize,
    /// How well the model can penetrate the Armour of enemy models in melee.
    armour_penetration: usize,
    /// Models with higher Agility strike first in melee.
    agility: usize,
}

/// Modifier for a unit like weapon, armor, etc.
struct Modifier {
    /// The modifier effect
    stat: Stats,
    /// Number of faces added to dice rolling
    nb_faces: isize,
    /// Number of dice added to dice rolling
    nb_dice: isize,
    /// This represents the requirements for the unit to be able to use the modifier as an array of flags
    requirements: Vec<String>,
}

/// Units status
enum Status {
    FLEE,
    CHARGE,
    FIGHT,
}

/// This structure represent a unit reaction when beeing charged.
enum ChargeReaction {
    SHOOT,
    RUN,
    HOLD,
}

/// This represent a unit in the game.
struct Unit {
    /// The unit's stats
    stats: Stats,
    /// The currently affecting modofiers for the unit like weapon oe spell
    modifiers: Vec<Modifier>,
    /// The current unit position (may be helpfull if we consider close range unit and far range unit)
    position: (usize, usize),
    /// The current status of the unit
    status: Status,
}
