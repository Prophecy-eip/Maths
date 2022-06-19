//pub use unit...
/// This structure represent all the stats that a unit can have.
#[derive(PartialEq, Copy, Clone)]
pub struct Stats {
    /// The distance the model can Advance Move in inches per turn.
    pub advance_rate: isize,
    /// The distance the model can March Move in inches per turn.
    pub march_rate: isize,
    /// Represents the model’s overall morale, bravery and a unit’s ability to quickly reorganise during battle.
    pub discipline: isize,
    /// When the model loses this many Health Points, it is removed as a casualty
    pub health_point: isize,
    /// How well the model avoids being hit in melee.
    pub defense: isize,
    /// How easily the model withstands blows.
    pub resilience: isize,
    /// The Armour of the model.
    pub armour: isize,
    /// A special protection for the model, the final defense after the model’s Armour.
    pub aegis: isize,
    /// The number of Melee Attacks the model has.
    pub attack: isize,
    /// How good the model is at scoring hits in melee.
    pub offensive: isize,
    /// How easily the model can wound enemy models in melee.
    pub strength: isize,
    /// How well the model can penetrate the Armour of enemy models in melee.
    pub armour_penetration: isize,
    /// Models with higher Agility strike first in melee.
    pub agility: isize,
    // Force ?
    pub force: isize,
}

/// Modifier for a unit like weapon, armor, etc.
#[derive(PartialEq, Clone)]
pub struct Modifier {
    /// The modifier effect
    pub stat: Stats,
    /// Number of faces added to dice rolling
    pub nb_faces: isize,
    /// Number of dice added to dice rolling
    pub nb_dice: isize,
    /// This represents the requirements for the unit to be able to use the modifier as an array of flags
    pub requirements: Vec<String>,
}

/// Units status
#[derive(Copy, Clone)]
pub enum Status {
    FLEE,
    CHARGE,
    FIGHT,
    IDLE,
}

/// This structure represent a unit reaction when beeing charged.
#[derive(Debug)]
pub enum ChargeReaction {
    SHOOT,
    RUN,
    HOLD,
}

/// This represent a unit in the game.
<<<<<<< HEAD
pub struct Unit {
    /// The unit's stats
    pub stats: Stats,
    /// The currently affecting modofiers for the unit like weapon oe spell
    pub modifiers: Vec<Modifier>,
    /// The current unit position (may be helpfull if we consider close range unit and far range unit)
    pub position: (isize, isize),
    /// The current status of the unit
    pub status: Status,
=======
struct Unit {
    /// The unit's stats
    stats: Stats,
    /// The currently affecting modofiers for the unit like weapon oe spell
    modifiers: Vec<Modifier>,
    /// The current unit position (may be helpfull if we consider close range unit and far range unit)
    position: (usize, usize),
    /// The current status of the unit
    status: Status,
>>>>>>> d1a7a77 (add a more clean version of the charge module)
}
