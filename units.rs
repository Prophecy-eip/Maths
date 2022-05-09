struct Stats {
    adv: usize, // Advance (in turns)
    mar: usize, // March (in turns)
    dis: usize, // Discipline
    hp: isize, // Hit Points
    def: usize, // Defense
    res: usize: // Resilience
    arm: usize, // Armor
    aeg: usize, // Aegis
    att: usize, // Attack
    off: usize, // Offense
    str: usize, // Strength
    ap: usize, // Armure Penetration
    agi: usize, // Agility
}

struct Requirement {
    status: Status,
    Unit: Army
}

enum Modifier { // Modifier for a unit like weapon, armor, etc.
    stat: Stats, // Stats
    requirement: Requirement,
}

enum Status { // Units status like under charge or in fight
    FLEE(usize), // Flee (range)
    PANIC, // Panic
    CHARGE, // Charge
    FIGHT
}

struct Unit {
    stats: Stats, // Stats
    modifiers: Vec<Modifier>, // Modifiers
    position: (usize, usize), // Position
}
