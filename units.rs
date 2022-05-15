struct Stats {
    // he distance the model can Advance Move in inches per turn.
    advance_rate: usize,
    // The distance the model can March Move in inches per turn.
    march_rate: usize,
    // Represents the model’s overall morale, bravery and a unit’s ability to quickly reorganise during battle.
    discipline: usize,
    // When the model loses this many Health Points, it is removed as a casualty
    health_point: isize,
    // How well the model avoids being hit in melee. A high Defensive Skill may also represent models with equipment useful for parrying melee attacks, such as shields. 
    defense: usize,
    // How easily the model withstands blows.
    resilience: usize,
    // The Armour of the model. This may represent particularly tough scaly hides as well as worn pieces of Armour.
    armour: usize,
    // A special protection for the model, the final defense after the model’s Armour. This may represent a model with magical defenses or regenerative abilities.
    aegis: usize,
    // The number of Melee Attacks the model has. This maybe higher due to sheer attacking speed and skill, equipment such as dual wielded blades, or the size to potentially hit many opponents with a single blow.
    attack: usize,
    // How good the model is at scoring hits in melee.
    offensive: usize,
    // How easily the model can wound enemy models in melee. This may represent not just sheer physical strength, but also well-equipped models with weapons such as halberds or heavy two handed weaponry.
    strength: usize,
    // How well the model can penetrate the Armour of enemy models in melee. This may be due to the physical attributes of the model, or also due to having weaponry designed to pierce Armour such as lances or spears
    armour_penetration: usize,
    // Models with higher Agility strike first in melee. This may be influenced by the model’s equipment. Models with an Aglity of zero for example represent models equipped with weapons that are particularly slow to strike with.
    agility: usize,
}

enum Modifier { // Modifier for a unit like weapon, armor, etc.
    // The modifier effect
    stat: Stats,
    // This represents the requirements for the unit to be able to use the modifier as an array of flags
    requirements: Vec<String>,
}

enum Status { // Units status like under charge or hand to hand
    FLEE,
    PANIC,
    CHARGE,
    FIGHT
}

struct Unit {
    stats: Stats, // Stats
    modifiers: Vec<Modifier>, // Modifiers
    position: (usize, usize), // Position
}
