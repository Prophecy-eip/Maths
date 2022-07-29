/// # This structure represent all the stats that a Model have.
///
/// ## Members
/// advance_rate (isize): The distance the model can Advance Move in inches per turn.
///
/// march_rate (isize): The distance the model can March Move in inches per turn.
///
/// discipline (isize): Represents the model’s overall morale, bravery and a unit’s ability to quickly reorganise during battle.
///
/// health_point (isize): When the model loses this many Health Points, it is removed as a casualty
///
/// defense (isize): How well the model avoids being hit in melee.
///
/// resilience (isize): How easily the model withstands blows.
///
/// armour (isize): The Armour of the model.
///
/// aegis (isize): A special protection for the model, the final defense after the model’s Armour.
///
/// attack (isize): The number of Melee Attacks the model has.
///
/// offensive (isize): How good the model is at scoring hits in melee.
///
/// strength (isize): How easily the model can wound enemy models in melee.
///
/// armour_penetration (isize): How well the model can penetrate the Armour of enemy models in melee.
///
/// agility (isize): Models with higher Agility strike first in melee.
#[derive(PartialEq, Copy, Clone)]
pub struct Stats {
    pub advance_rate: isize,
    pub march_rate: isize,
    pub discipline: isize,
    pub health_point: isize,
    pub defense: isize,
    pub resilience: isize,
    pub armour: isize,
    pub aegis: isize,
    pub attack: isize,
    pub offensive: isize,
    pub strength: isize,
    pub armour_penetration: isize,
    pub agility: isize,
}

/// # Modifier for a unit like weapon, armor, etc.
///
/// ## Members
/// stat (Stats): The stat modified
///
/// nb_faces (isize): Number of faces added to dice rolling
///
/// nb_dice (isize): Number of dice added to dice rolling
///
/// requirements (Vec<String>): This represents the requirements for the unit to be able to use the modifier as an array of flags
#[derive(PartialEq, Clone)]
pub struct Modifier {
    pub stat: Stats,
    pub nb_faces: isize,
    pub nb_dice: isize,
    pub requirements: Vec<String>,
}

/// # Units status
///
/// ## Values
/// FLEE
///
/// CHARGE
///
/// FIGHT
///
/// IDLE
#[derive(Copy, Clone)]
pub enum Status {
    FLEE,
    CHARGE,
    FIGHT,
    IDLE,
}

/// This structure represent a unit reaction when beeing charged.
enum ChargeReaction {
    SHOOT,
    RUN,
    HOLD,
}

/// # This represent a unit in the game.
///
/// ## Members
/// stat (Stat): The unit's stats
///
/// modifiers (Vec<Modifiers>): The currently affecting modifiers for the unit like weapon or spell
///
/// position ((isize, isize)): The current unit position (may be helpfull if we consider close range unit and far range unit)
///
/// status (Status): The current status of the unit
pub struct Unit {
    pub stats: Stats,
    pub modifiers: Vec<Modifier>,
    pub position: (isize, isize),
    pub status: Status,
}
