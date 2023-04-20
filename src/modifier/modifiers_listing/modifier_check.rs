/// Enum containing the two ways to apply a Modifier
///
/// # Attributes
///
/// AlwaysTrue (bool): When the Modifier always apply
///
/// Function (fn(crate::regiment::Regiment, crate::regiment::Regiment) -> bool): When the Modifier need a function to specify if it can be applied in this specific case
pub enum ModifierCheck {
    AlwaysTrue(bool),
    #[allow(dead_code)]
    Function(fn(crate::regiment::Regiment, crate::regiment::Regiment) -> bool),
}

#[allow(dead_code)]
pub fn flame_of_the_east(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wielder gains Volcanic Embrace (D3) in the Melee Phase while using this weapon.
    true
}

#[allow(dead_code)]
pub fn eye_of_the_bull(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Close Combat Attacks and Shooting Attacks made with this weapon hit automatically. The Strength of these hits is always set to 5 and their Armour Penetration is always set to 10. . In addition, while using this weapon, the wielder's Attack Value is set to 1 and Close Combat Attacks made with this weapon gain Multiple Wounds (2).
    true
}

#[allow(dead_code)]
pub fn onyx_core(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon become Flaming Attacks and gain Multiple Wounds (D3, against Flammable). In addition, their Strength is always set to 6.
    true
}

#[allow(dead_code)]
pub fn banner_of_the_twice_branded(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated at the end of the Charge Phase, immediately after all Charge Moves have been resolved. If the bearer’s unit was successfully Charged during this phase, it may perform a Combat Reform (following the normal rules for Combat Reforms).
    true
}

#[allow(dead_code)]
pub fn their_masters_banner(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Apply the following effects while the bearer's unit is within 6in of one or more models with Infernal Brand: Models in the bearer's unit without Infernal Brand gain Battle Focus. If the bearer's unit is composed entirely of models without Infernal Brand, it may reroll failed Charge Range rolls in the Charge Phase.
    true
}

#[allow(dead_code)]
pub fn icon_of_ashuruk(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated at the start of any Melee Phase. The bearer gains Volcanic Embrace~(X), where X is the number of friendly units within 6in of the bearer's unit that contain at least one model with Magical Attacks. In addition, attacks made by friendly units within 6in of the bearer's unit, except attacks made by the bearer, lose Flaming Attacks and Magical Attacks (if applicable). The effects last until the end of the Player Turn.
    true
}

#[allow(dead_code)]
pub fn blaze_of_protection(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +3 Armour. Every enemy model in base contact with the wearer's model that could allocate one or more Close Combat Attacks towards it but doesn't, after resolving its Close Combat Attacks, suffers 1 hit with Strength 4, Armour Penetration 0, and Flaming Attacks, distributed onto the model's Health Pool. This is considered a Special Attack.
    true
}

#[allow(dead_code)]
pub fn kadim_binding(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's weapons lose Two-Handed if they had it. While using this Shield, the bearer gains Aegis (+1, against Flaming Attacks, max. 3+) and Parry.
    true
}

#[allow(dead_code)]
pub fn breath_of_the_brass_bull(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's model gains +1 Health Point and the bearer gains Breath Attack (Toxic Attacks).
    true
}

#[allow(dead_code)]
pub fn golden_idol_of_shamut(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // If the bearer's model is Infantry, its Advance Rate is set to 4in and its March Rate is set to 12in. In addition, the bearer can cast Glory of Gold (Alchemy) as a Bound Spell with Power Level (4/8).
    true
}

#[allow(dead_code)]
pub fn mask_of_ages(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Aegis (5+, against Special Attacks), Aegis (5+, against Magical Attacks), and Fear. In addition, the bearer must reroll failed to-hit rolls with its Close Combat Attacks.
    true
}

#[allow(dead_code)]
pub fn lugars_dice(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // A single model part of the bearer's model can reroll a single failed to-hit, to-wound, or Armour Save roll per Player Turn. Crush Attacks are not affected.
    true
}

#[allow(dead_code)]
pub fn ring_of_desiccation(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // At the start of each Round of Combat that the bearer's unit is fighting, every enemy unit in base contact with the bearer's model gains one Incendiary marker.
    true
}

#[allow(dead_code)]
pub fn tablet_of_vezodinezh(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // When the bearer attempts to cast a non-Bound Spell using three or more Magic Dice, treat a single rolled ‘1’ or ‘2’ as a natural ‘3’. If the bearer would suffer a Witchfire Miscast effect, treat it as Magical Inferno instead.
    true
}

#[allow(dead_code)]
pub fn legend_of_the_black_king(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +1 Armour and Aegis (4+).
    true
}

#[allow(dead_code)]
pub fn black_standard_of_zagvozd(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Close Combat Attacks made by R&F model parts without Harnessed in the bearer's unit gain +1 to hit.
    true
}

#[allow(dead_code)]
pub fn reapers_harvest(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon always have Strength 10 and Armour Penetration 10 and become Divine Attacks. When rolling to wound with attacks made with this weapon, use the enemy's Discipline instead of its Resilience.
    true
}

#[allow(dead_code)]
pub fn true_thirst(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain +1 Strength, +1 Armour Penetration, and Vampiric (3+). If the wielder's model has Towering Presence, the attacks gain Vampiric (5+) instead of Vampiric (3+). For each unsaved wound caused by this weapon during a Melee Phase, Raise 1 Health Point of R&F models in the wielder's unit at the end of the Melee Phase. The number of Raised Health Points in each phase cannot exceed the fixed component of the Reanimated value of the R&F models in the unit, disregarding any D3 or D6 parts (e.g. you can Raise 4 Zombies in a single phase).
    true
}

#[allow(dead_code)]
pub fn hypnotic_pendant(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Distracting. All Standard Height R&F models in the bearer's unit gain Parry.
    true
}

#[allow(dead_code)]
pub fn eternity_gem(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's model gains Immune (Lethal Strike, Multiple Wounds (X)). One use only: Must be activated when the bearer's model suffers its first wound in the game after Armour Saves. The bearer's model gains Aegis (2+) against this wound.
    true
}

#[allow(dead_code)]
pub fn necromantic_staff(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Channel (1) and can cast the first Boosted version (6in Aura) of Arise! (Hereditary Spell) as a Bound Spell with Power Level (4/8).
    true
}

#[allow(dead_code)]
pub fn nights_crown(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Close Combat Attacks allocated towards the bearer's model do not gain Strength modifiers of the +X type conferred by Close Combat Weapons. Natural to-wound rolls of ‘1’, ‘2’, and ‘3’ with Close Combat Attacks against the bearer's model are always considered failed.
    true
}

#[allow(dead_code)]
pub fn unholy_tome(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer can cast Danse Macabre (Evocation) as a Bound Spell with Power Level (4/8).
    true
}

#[allow(dead_code)]
pub fn cursed_medallion(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Right before the battle (during step 7 of the Deployment Phase Sequence), choose a Character, Champion, or a single model unit on the opponent's Army List. The bearer must reroll failed to-hit and to-wound rolls against the chosen model.
    true
}

#[allow(dead_code)]
pub fn lahmia(_attacker: crate::regiment::Regiment, _defender: crate::regiment::Regiment) -> bool {
    // The model part gains Distracting and Lightning Reflexes.
    true
}

#[allow(dead_code)]
pub fn strigoi(_attacker: crate::regiment::Regiment, _defender: crate::regiment::Regiment) -> bool {
    // The model part's Health Points are set to 4, and it gains Fortitude (4+) and Hatred.
    true
}

#[allow(dead_code)]
pub fn brotherhood_of_the_dragon(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model part gains Plate Armour, Weapon Master, and can take any number of Close Combat Weapons.
    true
}

#[allow(dead_code)]
pub fn nosferatu(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Gates of the Netherworld and Awaken (Skeletons, Zombies), and it knows the Hereditary Spell in addition to its other spells.
    true
}

#[allow(dead_code)]
pub fn von_karnstein(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Awaken (Bat Swarms, Dire Wolves, Great Bats, Zombies), Commanding Presence (+6in), and Rally Around the Flag (+6in). In addition, the model's unit gains Autonomous.
    true
}

#[allow(dead_code)]
pub fn ghoul_lord(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model and all R&F models in its unit gain Poison Attacks. If the R&F models already had Poison Attacks, they wound automatically on a successful natural to-hit roll of ‘5’ or ‘6’, unless the target has Immune (Poison Attacks).
    true
}

#[allow(dead_code)]
pub fn commandment(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The Defensive Skill and Offensive Skill of R&F models in the model's unit are set to 6.
    true
}

#[allow(dead_code)]
pub fn crimson_rage(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model part gains Battle Focus and must reroll failed to-hit rolls.
    true
}

#[allow(dead_code)]
pub fn storm_caller(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // All units within 12in of the model gain Hard Target (1).
    true
}

#[allow(dead_code)]
pub fn arcane_knowledge(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Spells cast by the model gain +6in range. Aura spells gain +3in range instead. Bound Spells and spells without range are not affected.
    true
}

#[allow(dead_code)]
pub fn supreme_lich(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model becomes a Wizard Master.
    true
}

#[allow(dead_code)]
pub fn eternal_duellist(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model part must reroll natural to-hit and to-wound rolls of ‘1’ with its Close Combat Attacks.
    true
}

#[allow(dead_code)]
pub fn monster_hunter(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model part gains Multiple Wounds (2, against Towering Presence).
    true
}

#[allow(dead_code)]
pub fn flying_horror(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Fly (7in, 14in), Light Troops, and Swiftstride.
    true
}

#[allow(dead_code)]
pub fn unbreakable_will(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // At the start of each Round of Combat, select a single friendly unit Engaged in the same Combat as the model (this can be the models's own unit). This unit gains Stubborn until the end of the Melee Phase.
    true
}

#[allow(dead_code)]
pub fn bestial_bulk(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model's Height is changed to Large and its base size to 40×40mm. While joined to a unit of Ghasts, the model gains Scoring. If playing Capture the Flags, the model gains Scoring (no matter if joined to a unit of Ghasts or not).
    true
}

#[allow(dead_code)]
pub fn mesmerising_gaze(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Whenever possible, unless another model does so first, the model must accept or issue a Duel. Duels issued by the model must (if possible) be accepted by a Character unless a Champion accepts first. Enemy model parts without Harnessed fighting a Duel with the model suffer −1 Attack Value.
    true
}

#[allow(dead_code)]
pub fn mysteries_of_the_night(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model's Path Access is replaced with Cosmology, Shamanism, and Witchcraft.
    true
}

#[allow(dead_code)]
pub fn oaken_might(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain +3 Strength.
    true
}

#[allow(dead_code)]
pub fn banner_of_deception(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // At the end of step 4 of the Deployment Phase Sequence (before deploying Scouts), the owner may remove the bearer's unit from the Battlefield and deploy it again elsewhere (any Characters joined to the unit must remain in the unit; this does not affect the number of Undeployed Units for calculating the starting roll-off bonus).
    true
}

#[allow(dead_code)]
pub fn predator_pennant(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's unit gains Devastating Charge~(Distracting).
    true
}

#[allow(dead_code)]
pub fn banner_of_silent_mist(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's unit gains Soft Cover. Enemy units within 3in of the bearer's unit may not gain any benefit from a Musician.
    true
}

#[allow(dead_code)]
pub fn hunters_honour(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain +1 Strength and +1 Armour Penetration. If the wielder causes at least one unsaved wound with this weapon, the wielder and all R&F models in the wielder's unit gain Distracting until the end of the Melee Phase.
    true
}

#[allow(dead_code)]
pub fn watchers_woe(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Shots always set to 4. Shooting Attacks made with this weapon become Poison Attacks.
    true
}

#[allow(dead_code)]
pub fn bough_of_wyscan(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Shots always set to 1, Str 4 [6], AP 2 [10], Area Attack (1×5), [Multiple Wounds (2)], Reload!.
    true
}

#[allow(dead_code)]
pub fn spirit_of_the_whirlwind(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wielder gains +1 Attack Value, and attacks made with this weapon gain +1 Strength and Lethal Strike.
    true
}

#[allow(dead_code)]
pub fn shielding_bark(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +1 Armour, Aegis (5+), Fearless, Flammable, and Magical Attacks.
    true
}

#[allow(dead_code)]
pub fn mist_walkers_mirror(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated at the end of any friendly Movement Phase if the bearer’s unit consists entirely of Standard Height Infantry models, is unengaged, and is fully inside a Forest Terrain Feature that doesn’t contain any enemy models. Apply the following rules:  The bearer's unit loses Scoring until the start of the next phase. Remove the bearer's unit from the Battlefield. Immediately place it back on the Battlefield with Special Ambush (centre of a Forest Terrain Feature). If the unit cannot be placed, it is considered destroyed where it was removed.
    true
}

#[allow(dead_code)]
pub fn hail_shot(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. When this Artefact is used, it is a Shooting Weapon with the following profile: Range 30in, Shots 3D6, Str 4, AP 1, Magical Attacks. Aim is set to 2+. When fired from Short Range, it gains +1 Armour Penetration. Master Archer cannot be used in conjunction with Hail Shot.
    true
}

#[allow(dead_code)]
pub fn sacred_seeds(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. The player may activate this Artefact at the end of any friendly Movement Phase and place a Forest Terrain Feature in contact with the bearer and at least 1in away from any enemy units and other Terrain Features. The Forest must fit within a circle with a diameter of 6in.
    true
}

#[allow(dead_code)]
pub fn horn_of_the_wild_hunt(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated when a friendly unit within 8in fails a roll for Charge Range. The roll may be rerolled.
    true
}

#[allow(dead_code)]
pub fn drums_of_cenyrn(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated when the bearer's unit declares a Charge. The target of the Charge may only declare Hold as its Charge Reaction unless it is already Fleeing. The enemy unit may still declare Charge Reactions as normal if it is subsequently Charged by other units.
    true
}

#[allow(dead_code)]
pub fn glyph_of_amryl(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Cannot be Stomped. When fighting a Duel, the bearer gains +3 Defensive Skill.
    true
}

#[allow(dead_code)]
pub fn shapeshifter(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Exclusive, Fortitude (4+), and Vanguard, and its March Rate is set to 20in.
    true
}

#[allow(dead_code)]
pub fn wild_hunter(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The Forest Prince or Chieftain model part gains Battle Focus, Devastating Charge (+2 Att), Fearless, Frenzy, and Light Troops.
    true
}

#[allow(dead_code)]
pub fn pathfinder(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Master Archer and Scout. The Shots of a Sylvan Longbow wielded by the model are set to 3. If wielded by a Forest Prince, its Shots are set to 4 instead.
    true
}

#[allow(dead_code)]
pub fn forest_guardian(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Hatred and +1 Armour.
    true
}

#[allow(dead_code)]
pub fn blade_dancer(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Aegis (6+), Dances of Cenyrn (see Blade Dancer unit), Exclusive, and Fearless. The model's unit gains Swiftstride. The model cannot use any Shooting Weapons nor benefit from Armour (neither mundane nor enchanted).
    true
}

#[allow(dead_code)]
pub fn scarred_bark(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // All Dryads in the model's unit gain Hatred.
    true
}

#[allow(dead_code)]
pub fn toxic_spores(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model's unit gains Lethal Strike.
    true
}

#[allow(dead_code)]
pub fn entangling_vines(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // In a Duel, opponents must reroll successful to-hit rolls against the model.
    true
}

#[allow(dead_code)]
pub fn oaken_crown(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Swift Reform.
    true
}

#[allow(dead_code)]
pub fn executioners_icon(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer and R&F model parts with Ruthless Efficiency in the bearer's unit gain Artistry of Death.
    true
}

#[allow(dead_code)]
pub fn eye_of_the_gorgon(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Petrifying Stare (see Gorgons unit). Enemy units in base contact with the bearer must reroll successful Discipline Tests.
    true
}

#[allow(dead_code)]
pub fn caedhrens_pennon(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Infantry models in the bearer's unit gain Scent of Blood and Swiftstride.
    true
}

#[allow(dead_code)]
pub fn crippling_frost(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Enemy units in base contact with the bearer's model suffer -2 Defensive Skill.
    true
}

#[allow(dead_code)]
pub fn mastery_of_slaughter(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain Artistry of Death, Battle Focus, Lethal Strike, and Multiple Wounds (2).
    true
}

#[allow(dead_code)]
pub fn pride_of_gar_daecos(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // While using this weapon, the wielder gains +1 Attack Value, and attacks made with this weapon become Divine Attacks.
    true
}

#[allow(dead_code)]
pub fn transcendence(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // For each unsaved wound inflicted on enemy models with this weapon, the wielder gains +1 Strength and +1 Armour Penetration for the rest of the game, up to a maximum of +2 each.
    true
}

#[allow(dead_code)]
pub fn lacerating_touch(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain +2 Armour Penetration. While using this weapon, the wielder gains +2 Attack Value and Fear.
    true
}

#[allow(dead_code)]
pub fn seal_of_the_republic(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // For each unsaved wound inflicted on enemy models with the wearer's Close Combat Attacks, the wearer gains +1 Armour for the rest of the game.
    true
}

#[allow(dead_code)]
pub fn beastmasters_whistle(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated at the start of any friendly Charge Phase. Friendly Manticores and friendly models with Type Beast within 12in of the bearer gain Maximised (Charge Range) until the end of the Charge Phase.
    true
}

#[allow(dead_code)]
pub fn ceinrans_scales(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer can cast Spectral Blades (Evocation) as a Bound Spell with Power Level (4/8).
    true
}

#[allow(dead_code)]
pub fn ring_of_the_obsidian_thrones(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // While within 6in of the bearer, friendly units gain Minimised (Break Tests, Panic Tests).
    true
}

#[allow(dead_code)]
pub fn seal_of_the_9th_fleet(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // If the bearer's unit consists entirely of Infantry models, it gains Academy Trained.
    true
}

#[allow(dead_code)]
pub fn mask_of_the_war_crow(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's unit gains Fear. In addition, the bearer and any friendly unit that contains one or more models with Fear within 6in of the bearer's unit must reroll natural to-wound rolls of ‘1’ with its Close Combat Attacks.
    true
}

#[allow(dead_code)]
pub fn moithirs_mirror(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Aegis (5,+ against Melee Attacks). In addition, at Initiative Step 0 of a Round of Combat in which one or more Melee Attacks are allocated towards or distributed onto the bearer's model (including Initiative Step 0), the bearer inflicts 3 hits with Strength 4, Armour Penetration 2, and Magical Attacks on each of the attacking models' units. This is considered a Special Attack.
    true
}

#[allow(dead_code)]
pub fn banner_of_the_entombed(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // If taken by a Character, the bearer gains Special Ambush (Open Terrain). If taken by a R&F model, the bearer’s unit gains Special Ambush (Open Terrain) and additional models cannot be added to the unit during Army List creation. Standard Height models using this banner to Ambush must arrive in a formation containing exactly 5 models per rank (except for the last) and cannot make a Reform (or a Swift Reform) during this Player Turn. Units with Special Ambush (Open Terrain) also count towards Entombed.
    true
}

#[allow(dead_code)]
pub fn godslayer(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wielder gains +1 Attack Value while using this weapon. Attacks made with this weapon become Divine Attacks and gain Multiple Wounds (2, against Aegis (X+)) (note that the latter also applies against models with Aegis Saves with Conditional Application).
    true
}

#[allow(dead_code)]
pub fn scourge_of_kings(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // While using this weapon, the wielder's Attack Value is set to 6. When fighting a Duel, attacks made with this weapon must reroll failed to-wound rolls.
    true
}

#[allow(dead_code)]
pub fn jackals_blessing(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +2 Health Points and Fortitude (5+).
    true
}

#[allow(dead_code)]
pub fn suns_embrace(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Distracting while using this Shield.
    true
}

#[allow(dead_code)]
pub fn crown_of_the_pharaohs(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Commanding Presence~(+6in). At the start of each of your Player Turns, the bearer may lose Undying Will until the start of your next Player Turn and choose a friendly unit within 12in. This unit gains Undying Will until the start of your next Player Turn.
    true
}

#[allow(dead_code)]
pub fn sacred_hourglass(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer may reroll the first failed Casting Attempt of a spell of type Augment in each Magic Phase that was rolled using 2 Magic Dice (by rerolling both Magic Dice).
    true
}

#[allow(dead_code)]
pub fn book_of_the_dead(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer can cast Death is Only the Beginning as a Bound Spell with Power Level (4/8) and the following modification: The spell's range is changed to 12in Aura. The spell may target units containing one or more models with Ensouled Statue.
    true
}

#[allow(dead_code)]
pub fn steeds_of_nephet_ra(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Models with at least one Skeletal Horse model part in the bearer's unit gain Ghost Step and +4in March Rate.
    true
}

#[allow(dead_code)]
pub fn blessed_wrappings(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains +1 Health Point and loses Flammable if it had it (note that this does not prevent the model from gaining Flammable from other sources).
    true
}

#[allow(dead_code)]
pub fn sekhem_sceptre(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Autonomous and Stubborn.
    true
}

#[allow(dead_code)]
pub fn ankh_of_naptesh(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Hierophant and can never lose it. R&F models in the bearer's unit gain Fortitude (6+).
    true
}

#[allow(dead_code)]
pub fn sandstorm_cloak(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Fly~(5in, 15in), Light Troops, and Swiftstride, and can perform a Sweeping Attack that causes 2D6 hits with Strength 2 and Armour Penetration 1.
    true
}

#[allow(dead_code)]
pub fn death_mask_of_teput(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Enemy units in base contact with the bearer suffer -2 Offensive Skill.
    true
}

#[allow(dead_code)]
pub fn scroll_of_desiccation(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // After determining Deployment Zones (at the end of step 6 of the Pre-Game Sequence), choose a Field, Forest, or Water Terrain Feature. This Terrain Feature ceases to be the Terrain Feature it used to be and loses all its rules. It is treated as Dangerous Terrain (1) for all enemy units.
    true
}

#[allow(dead_code)]
pub fn oriflamme(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Fear while Engaged in Combat. Enemy units in base contact with the bearer’s unit cannot benefit from Rally Around the Flag.
    true
}

#[allow(dead_code)]
pub fn banner_of_roland(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's unit gains ( Devastating Charge (Aegis (4+)). In addition, enemy units cannot choose Stand and Shoot as a Charge Reaction against Charges declared by the bearer's unit.
    true
}

#[allow(dead_code)]
pub fn relic_shroud(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer can cast Breath of the Lady (Hereditary Spell) as a Bound Spell with Power Level (4/8).
    true
}

#[allow(dead_code)]
pub fn banner_of_elan(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Close Combat Attacks made with Lances by the bearer’s unit gain +2 Strength and +2 Armour Penetration in the First Round of Combat if the following conditions are met: The bearer’s unit is only Engaged in its Front Facing. The bearer’s unit is not Charging. The bearer’s unit failed a Charge in its previous Charge Phase.
    true
}

#[allow(dead_code)]
pub fn castellans_crest(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated immediately before declaring a Charge with the bearer or the bearer’s unit in the Charge Phase. Failed Charge Range rolls of the bearer or the bearer’s unit must be rerolled until the end of the phase. Other Characters Charging out of the bearer’s unit are not affected.
    true
}

#[allow(dead_code)]
pub fn tristans_resolve(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // While using this weapon, the wielder gains +1 Attack Value, and attacks made with this weapon gain +1 Armour Penetration. After a successful to-hit roll, the _attacker may discard one of the hits with this weapon and choose an enchanted weapon carried by the model the attack was allocated towards. Any Weapon Enchantment of the chosen weapon is ignored for the rest of the game.
    true
}

#[allow(dead_code)]
pub fn mortal_reminder(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wielder gains Fear and Terror. Attacks made with this weapon gain +1 Strength and +1 Armour Penetration.
    true
}

#[allow(dead_code)]
pub fn divine_judgement(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // After the wielder completes a Charge, attacks made with this weapon gain +2 Strength and +2 Armour Penetration until the wielder is no longer Engaged in Combat.
    true
}

#[allow(dead_code)]
pub fn uthers_mettle(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Once per Round of Combat, unless fighting a Duel, after one or more successful to-hit rolls made with this weapon against an enemy model, the target's unit suffers 1 hit with Area Attack (1×5) in the same Initiative Step as the initial Close Combat Attack. The hits from the Area Attack have the same Strength, Armour Penetration, and Attack Attributes as the initial Close Combat Attack. This is considered a Special Attack.
    true
}

#[allow(dead_code)]
pub fn prayer_etched(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +1 Armour and Aegis (+1, max. 4+).
    true
}

#[allow(dead_code)]
pub fn percivals_panoply(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +2 Armour.
    true
}

#[allow(dead_code)]
pub fn fortress_of_faith(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // While using this Shield, the bearer must reroll Armour Save rolls of ‘1’.
    true
}

#[allow(dead_code)]
pub fn black_knights_tabard(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. The first time the bearer’s model suffers an unsaved wound from an attack with Multiple Wounds (X), the model gains Immune (Multiple Wounds (X)) until the end of the phase.
    true
}

#[allow(dead_code)]
pub fn sacred_chalice(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Magic Resistance (1). When the bearer’s unit is the target of an enemy Casting Attempt, including Attribute Spells, the bearer’s owner gains 1 Veil Token.
    true
}

#[allow(dead_code)]
pub fn quin(_attacker: crate::regiment::Regiment, _defender: crate::regiment::Regiment) -> bool {
    // The model is a Wizard Adept that chooses Witchcraft as its Path of Magic. If on foot, the model gains Scout.
    true
}

#[allow(dead_code)]
pub fn cleric(_attacker: crate::regiment::Regiment, _defender: crate::regiment::Regiment) -> bool {
    // The model gains Honesty, Ordo Minister, Ordained, and loses Ordeal.
    true
}

#[allow(dead_code)]
pub fn bannerman(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model becomes the Battle Standard Bearer.
    true
}

#[allow(dead_code)]
pub fn minstrel(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Musician and is a Wizard Apprentice that chooses Divination as its Path of Magic.
    true
}

#[allow(dead_code)]
pub fn castellan(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model part gains +2 Attack Value.
    true
}

#[allow(dead_code)]
pub fn honour(_attacker: crate::regiment::Regiment, _defender: crate::regiment::Regiment) -> bool {
    // The model gains Maximised (Charge Range). In addition, while Engaged with an enemy unit's Front Facing, the model part gains Devastating Charge (+2 Att).
    true
}

#[allow(dead_code)]
pub fn justice(_attacker: crate::regiment::Regiment, _defender: crate::regiment::Regiment) -> bool {
    // Whenever another model in the model's unit suffers an unsaved wound due to an enemy Melee Attack (including Initiative Step 0), the model part must perform a single Close Combat Attack at Initiative Step 0 against an enemy model in base contact. If this is not possible, the effect is ignored. The number of these bonus attacks that the model part performs can never be higher than 3 per Round of Combat.
    true
}

#[allow(dead_code)]
pub fn valour(_attacker: crate::regiment::Regiment, _defender: crate::regiment::Regiment) -> bool {
    // The model gains Fearless and the model part's Close Combat Attacks gain Multiple Wounds (2, against Fear).
    true
}

#[allow(dead_code)]
pub fn excellence(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Duels issued by the model must (if possible) be accepted by an enemy Character, unless a Champion accepts first. In addition, while fighting a Duel, the model part's Close Combat Attacks gain +2 Armour Penetration.
    true
}

#[allow(dead_code)]
pub fn faith(_attacker: crate::regiment::Regiment, _defender: crate::regiment::Regiment) -> bool {
    // The model gains Ordained . In addition, if the model's unit is the target of an Orison while Engaged in Combat, add one Blessing Token to the owner's Blessing Token pool.
    true
}

#[allow(dead_code)]
pub fn forbearance(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // While using a Shield, the model gains Distracting.
    true
}

#[allow(dead_code)]
pub fn generosity(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Attached and Exclusive (R&F model). R&F models with Courage in the unit that the model is deployed in gain Bastard Sword and Weapon Master until the end of the game.
    true
}

#[allow(dead_code)]
pub fn thrice_forged(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +3 Armour. If the wearer's model has Towering Presence, its Armour can never be improved beyond 5.
    true
}

#[allow(dead_code)]
pub fn gladiators_spirit(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +1 Armour and Parry.
    true
}

#[allow(dead_code)]
pub fn icon_of_the_infinite(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer can cast Hellfire (Hereditary Spell) as a Bound Spell with Power Level (4/8).
    true
}

#[allow(dead_code)]
pub fn wasteland_torch(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's unit gains Strider (Ruins). After determining Deployment Zones (at the end of step 6 of the Pre-Game Sequence), you may choose a single Field or Forest Terrain Feature that becomes Ruins. The bearer's unit gains Flaming Attacks in the First Round of Combat.
    true
}

#[allow(dead_code)]
pub fn zealots_banner(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Models in the second rank of the bearer's unit gain Extra Support (2).
    true
}

#[allow(dead_code)]
pub fn burning_portent(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon become Flaming Attacks, gain Multiple Wounds (D3), and their Armour Penetration is set to 10.
    true
}

#[allow(dead_code)]
pub fn symbol_of_slaughter(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // While using this weapon, the wielder gains +2 Attack Value and +2 Agility. Close Combat Attacks made against the wielder's model gain +1 to hit.
    true
}

#[allow(dead_code)]
pub fn ledger_of_souls(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Immediately before removing a friendly model without Insignificant in a unit within 9in of the bearer's model due to enemy attacks, you gain one Veil Token for each Health Point that model had the first time it entered the Battlefield.
    true
}

#[allow(dead_code)]
pub fn immortal_gauntlets(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // At the start of any Round of Combat that the bearer is fighting, you may discard a Veil Token from your Veil Token pool. If so, choose either Divine Attacks, Flaming Attacks, or Magical Attacks. The bearer's Close Combat Attacks gain the chosen Attack Attribute. The chosen effect lasts until the end of the phase.
    true
}

#[allow(dead_code)]
pub fn veilgate_orb(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // At the end of Siphon the Veil, the army may keep up to 6 Veil Tokens instead of the normal 3.
    true
}

#[allow(dead_code)]
pub fn lord_of_the_damned(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Any unit with Irredeemable within 18in of the bearer's model may reroll the distance it moves in the Movement Phase with Random Movement.
    true
}

#[allow(dead_code)]
pub fn wyrd_stone(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. Must be activated when the bearer's model suffers the first hit in the game. This hit is ignored. If the bearer is hit by several simultaneous attacks, the bearer chooses which attack to ignore.
    true
}

#[allow(dead_code)]
pub fn daemonic_wings(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Fly~(8in, 16in), Light Troops, and Swiftstride.
    true
}

#[allow(dead_code)]
pub fn dark_prelate(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer can cast Spectral Blades (Evocation) and Hand of Glory (Occultism) as Bound Spells with Power Level (4/8) and type Caster's Unit. Hand of Glory is cast as the amplified version without performing The Sacrifice.
    true
}

#[allow(dead_code)]
pub fn entropic_aura(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Weapon Enchantments and Armour Enchantments carried by the bearer, models in the bearer's unit, and models in units that are in base contact with the bearer are ignored.
    true
}

#[allow(dead_code)]
pub fn luck_of_the_dark_gods(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's model gains Aegis (+1, max. 4+).
    true
}

#[allow(dead_code)]
pub fn idol_of_spite(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated at the start of a Round of Combat. For the duration of that Round of Combat, the bearer gains +1 Attack Value, +1 Strength, and +1 Armour Penetration.
    true
}

#[allow(dead_code)]
pub fn banner_of_the_wild_herd(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. A single Banner of the Wild Herd per unit may be activated at the start of each Round of Combat. For the duration of this Round of Combat, all Mongrel Herd and Wildhorn Herd models in the bearer's unit gain +1 Strength and +1 Armour Penetration.
    true
}

#[allow(dead_code)]
pub fn hawthorne_curse(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain Devastating Charge (+2 Str, +2 AP). The weapon can be used as a Shooting Weapon (3+) with the following profile: Range 18in, Shots 1, Str 3 [6], AP 10, Area Attack (1×5), Reload!, [Multiple Wounds (D3)]. This Shooting Attack never suffers negative to-hit modifiers.
    true
}

#[allow(dead_code)]
pub fn ancestral_carvings(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain +2 Strength and +2 Armour Penetration. The wielder gains +2 Attack Value and Distracting while using this weapon.
    true
}

#[allow(dead_code)]
pub fn twin_hungers(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain Lethal Strike. Whenever the wielder rolls a natural ‘6’ to wound with a Close Combat Attack, and this attack causes an unsaved wound, the bearer Recovers 1 Health Point at the end of the Initiative Step (unless the wielder's model was removed as a casualty in this Initiative Step). No more than 1 Health Point may be Recovered per phase in this manner.
    true
}

#[allow(dead_code)]
pub fn fatal_folly(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // For each Close Combat Attack against the wielder's model that rolls a natural to-hit roll of ‘1’, the wielder must perform a Close Combat Attack at the same Initiative Step (this overrides the normal restriction that Beast Axe attacks always strike at Initiative Step 0). This must be allocated towards the model (or Health Pool) that rolled the ‘1’ to hit.
    true
}

#[allow(dead_code)]
pub fn aaghors_affliction(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +1 Resilience and Fortitude (4+), but automatically fails all of its Armour Saves.
    true
}

#[allow(dead_code)]
pub fn tricksters_cunning(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Successful to-wound rolls against the wearer's model must be rerolled.
    true
}

#[allow(dead_code)]
pub fn wild_form(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // At the start of the Melee Phase, the bearer may choose to gain either of the following:+1 Strength, +1 Armour Penetration, and -1 Resilience-1 Strength, -1 Armour Penetration, and +1 ResilienceThe effects last until the end of the Melee Phase.
    true
}

#[allow(dead_code)]
pub fn obscuring_fog(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // While using this Shield, enemy units in base contact with the bearer suffer -1 Agility. The bearer's unit does not benefit from +1 Agility from Charging Momentum.
    true
}

#[allow(dead_code)]
pub fn dark_rain(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. This Artefact is automatically activated at the start of the first Game Turn (if the bearer is not on the Battlefield at this time, the item cannot be used). Its effects last until the end of the Game Turn. If the owner has the second Player Turn, all Shooting Attacks suffer -2 to hit during the opponent's Shooting Phase. If the owner has the first Player Turn, instead all Shooting Attacks suffer -1 to hit during the opponent's Shooting Phase.
    true
}

#[allow(dead_code)]
pub fn seed_of_the_dark_forest(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. Right before the battle (during step 7 of the Deployment Phase Sequence), the bearer must place a single Forest Terrain Feature that must be no larger than 10in in length and 6in in width on the Battlefield, not in contact with any other Terrain Feature except Open Terrain, more than 1in away from all enemy units, and with its centre within 12in of the bearer. While inside this Forest Terrain Feature, friendly models gain a +1 Casting Modifier for Augment, Hex, and Universal spells, and add (+1/+1) to the Power Level of Totem Bound Spells they cast.
    true
}

#[allow(dead_code)]
pub fn pillager_icon(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // All friendly units within 12in of the bearer comprised entirely of Razortusks or single model Chariots, excluding Characters, gain Vanguard.
    true
}

#[allow(dead_code)]
pub fn inscribing_burin(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // While the bearer has the centre of its base inside a Forest Terrain Feature, all friendly units with more than half of their models with the centre of their bases inside any Forest Terrain Feature on the Battlefield gain Magic Resistance~(2).
    true
}

#[allow(dead_code)]
pub fn eye_of_dominance(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Cannot be Stomped and Fear.
    true
}

#[allow(dead_code)]
pub fn crown_of_horns(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's unit and all units within range of its Commanding Presence (if applicable) automatically pass Discipline Tests taken due to Primal Instinct.
    true
}

#[allow(dead_code)]
pub fn daemons_bane(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +2 Armour against Magical Attacks.
    true
}

#[allow(dead_code)]
pub fn star_metal_alloy(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The first time the bearer's model suffers an unsaved wound from an attack with Multiple Wounds (X) while using this Shield, the number of wounds suffered is halved, rounding fractions up.
    true
}

#[allow(dead_code)]
pub fn protection_of_dorac(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +2 Armour and +2 Defensive Skill.
    true
}

#[allow(dead_code)]
pub fn gleaming_robe(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains Aegis (3+). In addition, its Armour is set to 1 and can never be improved beyond this. If the wearer Miscasts and rolls Magical Inferno or Witchfire, the number of hits is halved, rounding fractions up.
    true
}

#[allow(dead_code)]
pub fn navigators_banner(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // R&F models in the bearer's unit gain Distracting in the First Round of Combat against attacks from enemies Engaged in the bearer's unit's Front Facing.
    true
}

#[allow(dead_code)]
pub fn banner_of_becalming(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // In the opponent's Magic Phase, during Siphon the Veil before converting Veil Tokens into Magic Dice, remove one Veil Token from the opponent's Veil Token pool and add one Veil Token to your Veil Token pool.
    true
}

#[allow(dead_code)]
pub fn war_banner_of_ryma(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // R&F model parts without Harnessed in a unit with one or more War Banners of Ryma gain Devastating Charge (+1 Str). In addition, all Infantry models in the unit gain Devastating Charge (+1in Adv).
    true
}

#[allow(dead_code)]
pub fn nova_flare(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain Devastating Charge (+1 Att), Lethal Strike, and become Divine Attacks. One use only. May be activated at the start of any Round of Combat. The wielder counts as Charging for the purpose of Devastating Charge.
    true
}

#[allow(dead_code)]
pub fn sliver_of_the_blazing_dawn(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain +1 Strength and +2 Armour Penetration. Each successful to-hit roll with this weapon causes two hits instead of one.
    true
}

#[allow(dead_code)]
pub fn elus_heartwood(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // This weapon gains Shots 3, Str as user +1, and AP as user +1.
    true
}

#[allow(dead_code)]
pub fn book_of_meladys(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Once per Magic Phase, the bearer may reroll a single Magic Dice when making a casting roll, provided the spell was not Miscast. When rerolling a natural ‘1’, the rerolled Magic Dice benefits from Fizzle (if the Casting Attempt fails) regardless of the value rolled from the reroll.
    true
}

#[allow(dead_code)]
pub fn ring_of_the_pearl_throne(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Weapon Enchantments in the bearer’s unit and in units that are in base contact with the bearer are ignored.
    true
}

#[allow(dead_code)]
pub fn diadem_of_protection(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Aegis (+2, max 4+).
    true
}

#[allow(dead_code)]
pub fn amethyst_crystal(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Dispelling rolls made by the bearer's army gain a +1 modifier.
    true
}

#[allow(dead_code)]
pub fn glittering_lacquer(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Hard Target (1).
    true
}

#[allow(dead_code)]
pub fn asfad_scholar(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The range of the Wizard's non-Bound Spells is increased by 6in. Aura spells gain +3in range instead. Spells with type Caster are unaffected. The Wizard can cast Drain Magic as a Bound Spell with Power Level (4/8).Drain Magic: Range 18in, Type Universal, Duration Instant. All spells with duration One Turn for which the target of Drain Magic or a model or model part inside that unit was the spells' target immediately come to an end (note that if any of these spells had more than one target, their effects also end for these targets).
    true
}

#[allow(dead_code)]
pub fn order_of_the_fiery_heart(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Flaming Attacks. The model replaces its available Paths with Alchemy and Pyromancy, and ignores the Missile and Damage spell types for Silver Spike (Alchemy) and all Pyromancy spells, but only when targeting units that are Engaged in Combat with the model.The first time in each Magic Phase that the model successfully casts a Learned Spell, its mount (if there is any) gains +1in Advance Rate, +2in March Rate, and +2 Attack Value. The effects last until the start of the next friendly Magic Phase. In addition, the model gains access to the following options: •Shield: 10 •Heavy Armour: 15 •Dragonforged Armour: 25 •Paired Weapons: 5
    true
}

#[allow(dead_code)]
pub fn master_of_canreig_tower(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Master of Spellcrafting, Protean Magic, and Wizard Adept. The model has access to Alchemy, Cosmology, Druidism, Shamanism, and Witchcraft. •High Prince must gain 3 additional Learned Spells: 30
    true
}

#[allow(dead_code)]
pub fn high_warden_of_the_flame(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Aegis (4+), Fearless, Flaming Attacks, Magic Resistance (1), and cannot be equipped with a Shield.
    true
}

#[allow(dead_code)]
pub fn fleet_officer(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model part gains Steady Aim and, when using a Sky Reaper, +2 to-hit. An army with one or more Fleet Officers may add +1 to the roll for choosing Deployment Zones. While within the model's range of Commanding Presence or Rally Around the Flag (if applicable), models with Martial Discipline gain Minimised (Panic Tests).
    true
}

#[allow(dead_code)]
pub fn royal_huntsman(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Lion's Fur and the model's unit gains Valiant. When using a Great Weapon, the model part gains Multiple Wounds (2, against Large and Beast, Large and Cavalry, Gigantic).
    true
}

#[allow(dead_code)]
pub fn queens_cavalier(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model part gains Devastating Charge (+1 Att, Fear). If the army contains one or more Large or Gigantic models with Queen's Cavalier: The maximum sum of Ancient Allies values in the army is increased by 2. Characters is increased to Max. 50%. All Characters in the army must be Large Cavalry or Gigantic Beasts. Dragons and Ancient Dragons become 0--2 Mounts/Army. Sea Guard Reapers and Sky Sloops may not be taken in the army.
    true
}

#[allow(dead_code)]
pub fn queens_companion(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // When shooting with a Longbow without Weapon Enchantment, the model's weapon gains Shots 3. In addition, the model's unit gains Quick to Fire. One choice only: •Fae Miasma, Scout, Exclusive (Grey Watchers): 25 •Moonlight Arrows: 30
    true
}

#[allow(dead_code)]
pub fn household_standard(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // If the General is part of the bearer's unit, it gains Commanding Presence(+6in) .
    true
}

#[allow(dead_code)]
pub fn banner_of_unity(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Whenever the bearer's unit is targeted by an Order, it may immediately give an Order to a single Support Unit within 8in of the bearer's unit.
    true
}

#[allow(dead_code)]
pub fn marksmans_pennant(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's unit gains Steady Aim .
    true
}

#[allow(dead_code)]
pub fn the_light_of_sonnstahl(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon wound automatically and always have Armour Penetration 10 .
    true
}

#[allow(dead_code)]
pub fn death_warrant(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain Battle Focus . If a hit is scored with it against an enemy unit, friendly models with Parent Unit or Support Unit gain Battle Focus with attacks allocated towards the same enemy unit in the same phase in subsequent Initiative Steps.
    true
}

#[allow(dead_code)]
pub fn hammer_of_witches(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's Attack Value is set to 5 while using this weapon, and attacks made with it gain Battle Focus (against Channel (X)) .
    true
}

#[allow(dead_code)]
pub fn imperial_seal(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +3 Armour and +1 Discipline. The wearer's unit cannot voluntarily declare Flee as a Charge Reaction.
    true
}

#[allow(dead_code)]
pub fn blacksteel(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +1 Armour and Fear. If taken by a model on foot, the wearer gains an additional +1 Armour.
    true
}

#[allow(dead_code)]
pub fn witchfire_guard(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Aegis (4+, against Magical Attacks) while using this Shield.
    true
}

#[allow(dead_code)]
pub fn shield_of_volund(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // While using this Shield, the bearer's model gains Immune (Battle Focus, Lethal Strike) .
    true
}

#[allow(dead_code)]
pub fn winter_cloak(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Aegis (3+, against Flaming Attacks), Aegis (5+), and Distracting. The bearer automatically fails all Fortitude Saves.
    true
}

#[allow(dead_code)]
pub fn locket_of_sunna(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // When fighting a Duel, choose a single model part with neither Harnessed nor Inanimate that the bearer is fighting with. The bearer and the chosen model part must swap their corresponding unmodified Characteristic values of Attack Value, Strength, Armour Penetration, Agility, and Resilience. This is done before applying other modifiers. Note that if the bearer and/or the chosen model part are part of a Multipart Model, the Multipart Model's Resilience value is used.
    true
}

#[allow(dead_code)]
pub fn exemplars_flame(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer can cast Glory of Gold (Alchemy) as a Bound Spell with Power Level (4/8).
    true
}

#[allow(dead_code)]
pub fn karadons_courser(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated at the start of any friendly Player Turn. For the duration of this Player Turn, friendly units within 6in of the bearer must reroll failed Charge Range rolls.
    true
}

#[allow(dead_code)]
pub fn mantle_of_ullor(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Enemy units within 6in of the bearer do not gain +1 Agility for Charging Momentum.
    true
}

#[allow(dead_code)]
pub fn trolleater(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Fortitude (4+) and Multiple Wounds (2, against Large and Infantry).
    true
}

#[allow(dead_code)]
pub fn hoardmaster(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Weapon Master, Plate Armour, Great Weapon, Halberd, Iron Fist, and Paired Weapons. The bearer cannot take Weapon Enchantments.
    true
}

#[allow(dead_code)]
pub fn headhunter(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // At the end of any Melee Phase in which attacks made by the bearer have caused one or more enemy models to lose their last Health Point, roll a D6, unless the bearer is Fleeing. On a roll of 3+, the bearer's model Recovers a single Health Point.
    true
}

#[allow(dead_code)]
pub fn cult_leader(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Commanding Presence(+6in, max. 18in) and Rally Around the Flag(+6in, max. 18in).
    true
}

#[allow(dead_code)]
pub fn firebrand(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // If the bearer selects one or more spells from Pyromancy, it gains Aegis (3+, against Flaming Attacks), Breath Attack (Str 4, AP 0, Flaming Attacks), Flaming Attacks, and can cast Fireball (Pyromancy) as a Bound Spell with Power Level (4/8). The bearer automatically fails all Fortitude Saves.
    true
}

#[allow(dead_code)]
pub fn gut_roarer(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains +1 Resilience, Channel (1), and Fear.
    true
}

#[allow(dead_code)]
pub fn wildheart(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer loses Not a Leader and must be the General. Its Special Item allowance is increased to 150 pts. Another Mammoth Hunter in the army may be the Battle Standard Bearer for 50 pts; this Battle Standard Bearer gains Scrapling Lookout while joined to Yeti units. The Core limit is reduced to Min. 20%. The army may not include any Great Khans, Khans, Bruisers, Mercenary Veterans, Bombardiers, or Thunder Cannons. Mammoth Hunters become 0--4 Units/Army.
    true
}

#[allow(dead_code)]
pub fn spinesplitter(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Devastating Charge (+1 Att, +1 Str, +1 AP). The Strength and Armour Penetration bonuses from this instance of Devastating Charge also affect Impact Hits and Stomp Attacks.
    true
}

#[allow(dead_code)]
pub fn rottenjaw(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Poison Attacks, and all friendly Kin-Eater units may reroll Ambush rolls of 1 and 2 while the bearer is on the Battlefield. Unless the bearer is Gigantic, it gains Immune (Poison Attacks).
    true
}

#[allow(dead_code)]
pub fn banner_of_the_gyengget(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // In the First Round of Combat, natural to-hit, to-wound, and Armour Save rolls of ‘1’ from the bearer's unit must be rerolled, including Special Attacks.
    true
}

#[allow(dead_code)]
pub fn pennant_of_the_great_grass_sky(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's unit gains Swiftstride.
    true
}

#[allow(dead_code)]
pub fn skull_of_qenghet(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's unit gains Fear and automatically passes Panic Tests caused by Terror.
    true
}

#[allow(dead_code)]
pub fn khagadais_legacy(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain Multiple Wounds (D3).
    true
}

#[allow(dead_code)]
pub fn vipers_curse(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Shots 4, Str 4, AP 2 (Range is dependent on which weapon is enchanted). This weapon also gains Poison Attacks (in case of Brace of Ogre Pistols, the Attack Attribute also applies to Close Combat Attacks made with it). An enchanted Ogre Crossbow loses Area Attack (1×5). Shooting Attacks made with this weapon always hit on 4+.
    true
}

#[allow(dead_code)]
pub fn heart_ripper(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain Lethal Strike, +1 Armour Penetration, and can never hit on worse than 3+.
    true
}

#[allow(dead_code)]
pub fn ritual_bloodletter(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wielder gains +1 Attack Value while using this weapon. While the wielder's unit is Engaged in Combat, the wielder gains Channel (1).
    true
}

#[allow(dead_code)]
pub fn wrestlers_belt(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +2 Armour and +1 Strength.
    true
}

#[allow(dead_code)]
pub fn mammoth_hide_cloak(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +1 Armour. Attacks against the wearer can never have a Strength above 5.
    true
}

#[allow(dead_code)]
pub fn karkadans_resilience(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +1 Resilience but automatically fails all Special Saves.
    true
}

#[allow(dead_code)]
pub fn yeti_furs(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +1 Armour. Enemy units in base contact with the wearer suffer -1 Agility.
    true
}

#[allow(dead_code)]
pub fn lygurs_tongue(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Enemy units in base contact with the bearer suffer -2 Offensive Skill. .
    true
}

#[allow(dead_code)]
pub fn aurochs_charm(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Mountain Hide.
    true
}

#[allow(dead_code)]
pub fn rampagers_chain(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Stomp Attacks (D3+1), and all models in the bearer's unit must reroll failed to-wound rolls with Stomp Attacks.
    true
}

#[allow(dead_code)]
pub fn accurate(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    //
    true
}

#[allow(dead_code)]
pub fn devastating_charge(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // ( +1 Str, +1 AP)
    true
}

#[allow(dead_code)]
pub fn lethal_strike(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    //
    true
}

#[allow(dead_code)]
pub fn magic_resistance(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // ( 2)
    true
}

#[allow(dead_code)]
pub fn plate_armour(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    //
    true
}

#[allow(dead_code)]
pub fn poison_attacks(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    //
    true
}

#[allow(dead_code)]
pub fn swiftstride(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    //
    true
}

#[allow(dead_code)]
pub fn vanguard(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    //
    true
}

#[allow(dead_code)]
pub fn mikinoks_totem(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // At the start of each Round of Combat that the model's unit is fighting, choose one of the following: A Special Item* carried by a Character or Champion in a single enemy unit in base contact with the bearer's unit. A Special Item* carried by a single model unit in base contact with the bearer's unit. A Banner Enchantment carried by a Standard Bearer in a single enemy unit in base contact with the bearer's unit.This Special Item* is ignored during this Round of Combat. *Or, when fighting a Dwarven Holds army, a combination of Runic Weapon Enchantments, Runic Armour Enchantments, or Runic Artefacts.
    true
}

#[allow(dead_code)]
pub fn green_tide(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's unit gains Fight in Extra Rank.
    true
}

#[allow(dead_code)]
pub fn omen_of_the_apocalypse(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Roll a single D3 at the Initiative Step in which the wielder is attacking. During this Initiative Step, the wielder gains a modifier equal to the result of the D3 roll to its Attack Value, Strength, and Armour Penetration while using this weapon. .
    true
}

#[allow(dead_code)]
pub fn shady_shanking(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain Lethal Strike and Lightning Reflexes. When fighting a Duel, failed to-wound rolls with attacks made with this weapon must be rerolled.
    true
}

#[allow(dead_code)]
pub fn mazas_zappin(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wielder's unit gains Quick to Fire. This Bow gains Aim (2+), and its profile is changed to: Range 24in, Shots 3, Str as user, AP as user.
    true
}

#[allow(dead_code)]
pub fn tukteks_guard(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer's model gains +1 Resilience and Immune (Lethal Strike).
    true
}

#[allow(dead_code)]
pub fn crown_of_the_cavern_king(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // All models in the bearer's unit that have at least one model part of the Greenhide Races Common Goblin, Cave Goblin, or Forest Goblin gain Feigned Flight and Vanguard. If the bearer is a Common Goblin, Cave Goblin, or Forest Goblin, it gains Commanding Presence and Rally Around the Flag (+6in).
    true
}

#[allow(dead_code)]
pub fn skull_fetish(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // At the start of any friendly Magic Phase, add X Veil Tokens to your pool, where X is the number of friendly units Engaged in Combat minus the number of friendly Fleeing units. You cannot gain more than 3 Veil Tokens this way. These tokens are in addition to Veil Tokens gained from other sources. The Skull Fetish can never cause a loss of Veil Tokens.
    true
}

#[allow(dead_code)]
pub fn troll_ale_flask(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer may perform a Special Attack that is made at the bearer's Agility. In the corresponding Initiative Step, choose an enemy unit that the bearer is able to attack with Close Combat Attacks. This unit suffers a hit with Strength 5 and Armour Penetration 10.
    true
}

#[allow(dead_code)]
pub fn pan_of_protection_pinchin(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // When successfully wounded, the bearer may choose to use the _attacker's Armour value and/or its Special Save: Use the Armour value that the attacking model would have against the attack that inflicted the wound (including Conditional Application, modifiers, etc.). If so, the bearer cannot use its own Armour (including any modifiers), if available. Use the Special Save that the attacking model would have against the attack that inflicted the wound (including Conditional Application, modifiers, etc.). If so, the bearer cannot use its own Special Save (including any modifiers), if available.In addition, when the bearer's unit is the target of a spell, the bearer gains the same Magic Resistance as the Caster of the spell.
    true
}

#[allow(dead_code)]
pub fn plague_hermits_blessing(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer gains +1 Health Point and Fortitude (5+). Successful to-hit rolls with Close Combat Attacks against the wearer must be rerolled.
    true
}

#[allow(dead_code)]
pub fn sacred_aquila(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer and R&F models in the bearer's unit gain Battle Focus. Additionally, for each natural to-hit roll of ‘1’ with a Close Combat Attack made by the bearer's unit, the bearer's unit suffers 1 hit with Strength 3 and Armour Penetration 0 in the same Initiative Step. This is considered a Special Attack.
    true
}

#[allow(dead_code)]
pub fn bell_of_the_deep_roads(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's unit may start the game in Tunnel Reserve.
    true
}

#[allow(dead_code)]
pub fn rodentium_bullets(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The weapon's range is set to 18in. The weapon gains +2 Shots, +1 Armour Penetration, and Accurate.
    true
}

#[allow(dead_code)]
pub fn storm_rocket(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Aim 2+. Range is set to 24in. The weapon gains Flaming Attacks, Volley Fire, and Mishap (To-hit roll). If the weapon hits, it causes D6 hits whose Strength is set to 5 and whose Armour Penetration is set to 2.
    true
}

#[allow(dead_code)]
pub fn secrets_of_the_doom_blade(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // While using this weapon, the wielder gains +1 Attack Value. Attacks made with this weapon gain Multiple Wounds (D6) and become Divine Attacks and their Strength is always set to 10 and their Armour Penetration is always set to 3. At the end of each friendly Movement Phase, if the wielder is not Engaged in Combat, it suffers 1 hit with Toxic Attacks. The wielder may not take any other Special Items.
    true
}

#[allow(dead_code)]
pub fn swarm_master(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // While using this weapon, the wielder gains Grind Attacks (X), where X is the number of Full Ranks in the wielder's unit, up to a maximum of 10. These Grind Attacks are resolved with Strength 3 and Armour Penetration 1.
    true
}

#[allow(dead_code)]
pub fn tome_of_the_ratking(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer can cast The Awakened Swarm (Hereditary Spell) as a Bound Spell with Power Level (4/8).
    true
}

#[allow(dead_code)]
pub fn tarinas_lyre(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Stomp Attacks made by enemy units within 8in of the bearer suffer a -2 to-wound modifier.
    true
}

#[allow(dead_code)]
pub fn orators_toga(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Stand Behind and cannot issue Duels.
    true
}

#[allow(dead_code)]
pub fn crown_of_hubris(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // At the start of any friendly Magic Phase, the bearer may choose to inflict 3 hits on its unit that wound automatically with no saves of any kind allowed. If so, the owner gains 3 additional Veil Tokens.
    true
}

#[allow(dead_code)]
pub fn darkstone_detonator(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // At the end of any friendly Movement Phase after the first, the bearer may detonate a single friendly Tunnel Marker within 24in of it. If so, apply the following effects before removing the Tunnel Marker: Each unengaged unit within 4in of the Tunnel Marker suffers 2D6 hits with Strength 4 and Armour Penetration 1. If one or more units that are Engaged in the same Combat are within 4in of the Tunnel Marker, a total of 2D6 hits with Strength 4 and Armour Penetration 1 is inflicted. Roll a D6 for each hit: on a roll of 4+, the hit is distributed onto a randomly chosen friendly unit; otherwise, the hit is distributed onto a randomly chosen enemy unit.
    true
}

#[allow(dead_code)]
pub fn orb_of_ateus(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Once per Shooting Phase, the bearer may discard 2 Veil Tokens from its owner's Veil Token pool and nominate a friendly unit within 6in. Weapons with Trial and Terror in this unit gain +6in range and Magical Attacks, and the number of hits from each Mishap is increased by 2. The effects last until the end of the phase.
    true
}

#[allow(dead_code)]
pub fn cowl_of_the_apostate(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Holy Triumvirate, and for the purpose of this rule, it must select either Caelysian Pantheon or Cult of Errahman at the start of step 7 of the Pre-Game Sequence (Spell Selection).
    true
}

#[allow(dead_code)]
pub fn rune_of_destruction(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with a weapon engraved with this Rune gain Multiple Wounds (D3).
    true
}

#[allow(dead_code)]
pub fn rune_of_smashing(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with a weapon engraved with this Rune that are allocated towards a model with Resilience 5 or more have their Strength set to 10 and their Armour Penetration set to 10.
    true
}

#[allow(dead_code)]
pub fn rune_of_penetration(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with a weapon engraved with one or more Runes of Penetration gain +3 Armour Penetration.
    true
}

#[allow(dead_code)]
pub fn rune_of_might(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // For each Rune of Might engraved on a weapon, attacks made with it gain +1 Strength and +1 Armour Penetration.
    true
}

#[allow(dead_code)]
pub fn rune_of_precision(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wielder of a weapon engraved with this Rune gains Lightning Reflexes.
    true
}

#[allow(dead_code)]
pub fn rune_of_craftsmanship(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // A weapon engraved with this Rune follows the rules for Great Weapons instead of the original weapon's rules (this does not prevent the weapon from being engraved with additional Runes).
    true
}

#[allow(dead_code)]
pub fn rune_of_quickening(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // For each Rune of Quickening engraved on a weapon, the wielder gains +3 Agility while using it.
    true
}

#[allow(dead_code)]
pub fn rune_of_fury(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // For each Rune of Fury engraved on a weapon, the wielder gains +1 Attack Value while using it.
    true
}

#[allow(dead_code)]
pub fn rune_of_lightning(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // If the wielder scores at least one successful hit with a weapon engraved with one or more Runes of Lightning (consider each set of simultaneous attacks separately), each enemy unit that was hit suffers an additional D3 hits for each instance of this Rune. The hits are considered Special Attacks and are resolved with Strength 4, Armour Penetration 1, and Magical Attacks.
    true
}

#[allow(dead_code)]
pub fn rune_of_returning(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // A weapon engraved with this Rune can be used as a Shooting Weapon with Aim 2+ and the following profile: Range 8in, Shots 1, Str as user, AP as user, Quick to Fire,Accurate,Reload!. Shooting Attacks made with this weapon are affected by all Runic Weapon Enchantments on the engraved weapon (even if the effects are normally restricted to Close Combat Attacks).
    true
}

#[allow(dead_code)]
pub fn rune_of_fire(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // May be activated at the start of any phase or Round of Combat. If so, attacks made with a weapon engraved with this Rune become Flaming Attacks until the end of the phase.
    true
}

#[allow(dead_code)]
pub fn rune_of_resistance(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Successful to-wound rolls against the model of the wearer of an armour engraved with this Rune must be rerolled.
    true
}

#[allow(dead_code)]
pub fn rune_of_steel(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer of an armour engraved with this Rune must reroll failed Armour Saves.
    true
}

#[allow(dead_code)]
pub fn rune_of_retribution(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Whenever the wearer of an armour engraved with one or more Runes of Retribution rolls a successful Shield Wall Aegis Save (including Shield Wall stacked with Rune of Shielding), the wearer immediately inflicts a hit with the Strength and Armour Penetration of the saved attack on the model that caused the wound, before any casualties are removed, distributed onto the model's Health Pool. This is considered a Special Attack.
    true
}

#[allow(dead_code)]
pub fn rune_of_iron(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer of an armour engraved with: A single Rune of Iron gains +1 Armour. Two or more Runes of Iron gains +2 Armour.
    true
}

#[allow(dead_code)]
pub fn rune_of_the_forge(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer of an armour engraved with this Rune gains Aegis (3+, against Flaming Attacks).
    true
}

#[allow(dead_code)]
pub fn rune_of_denial(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. The player may choose to use this Rune instead of performing a Dispelling Attempt. The spell is automatically dispelled.
    true
}

#[allow(dead_code)]
pub fn rune_of_devouring(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. The player may choose to use this Rune instead of performing a Dispelling Attempt. The spell is cast as normal but the Caster may not cast it again for the rest of the game. Spells dispelled by a Rune of Revocation and Attribute Spells are not affected.
    true
}

#[allow(dead_code)]
pub fn rune_of_harnessing(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The Channel value of enemy model parts within 24in of the bearer's model (the value within brackets) is reduced by 1, to a minimum of 0.
    true
}

#[allow(dead_code)]
pub fn rune_of_the_courage(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated at the start of any Round of Combat. For the duration of the phase, the bearer gains Stubborn.
    true
}

#[allow(dead_code)]
pub fn rune_of_dragons_breath(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Breath Attack (Str 4, AP 1, Flaming Attacks, Magical Attacks). A single friendly Rune of Dragon's Breath may be used per Round of Combat.
    true
}

#[allow(dead_code)]
pub fn rune_of_grounding(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated at the start of any Melee Phase. All spells with Duration One Turn that affect any of the following units come to an end: The bearer's unit Enemy units in base contact with the bearer
    true
}

#[allow(dead_code)]
pub fn rune_of_readiness(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated at the end of the Charge Phase, immediately after all Charge Moves have been resolved. If the bearer's unit was successfully Charged during this phase, it may perform a Combat Reform (following the normal rules for Combat Reforms).
    true
}

#[allow(dead_code)]
pub fn rune_of_storms(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated at the start of the opponent's Player Turn. Choose a single enemy unit within 24in of the bearer. The Advance Rate and March Rate of all models with Fly in that unit (both for Ground and Flying Movement) are halved, rounding fractions up. The effect lasts until the end of the Player Turn.
    true
}

#[allow(dead_code)]
pub fn rune_of_shielding(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Aegis (+1, max. 4+). The Aegis from this Rune only stacks with itself and/or Shield Wall.
    true
}

#[allow(dead_code)]
pub fn rune_of_mastery(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. A single Rune of Mastery may be activated immediately before the bearer attempts to cast a Bound Spell. Add (+2/+2) to the Power Level of this Bound Spell for this Casting Attempt.
    true
}

#[allow(dead_code)]
pub fn rune_of_mining(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // After Determining Deployment Zones (at the end of step 6 of the Pre-Game Sequence), choose a Terrain Feature on the Battlefield. As long the bearer is on the Battlefield, all friendly models may treat this as Open Terrain when making Advance Moves or March Moves, but must still follow the Unit Spacing rule at the end of their movement.
    true
}

#[allow(dead_code)]
pub fn rune_of_kinship(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Ambush and Scout.
    true
}

#[allow(dead_code)]
pub fn runic_standard_of_shielding(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // All friendly units within 6in of the bearer gain Aegis (5+, against Shooting Attacks).
    true
}

#[allow(dead_code)]
pub fn runic_standard_of_swiftness(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's unit gains Vanguard.
    true
}

#[allow(dead_code)]
pub fn runic_standard_of_dismay(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Units Charging the bearer's unit suffer -2in Advance Rate for their Charge Range roll.
    true
}

#[allow(dead_code)]
pub fn runic_standard_of_steadiness(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated at the start of any Movement Phase. The bearer's unit gains Quick to Fire until the end of the Player Turn.
    true
}

#[allow(dead_code)]
pub fn runic_standard_of_the_anvil(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Friendly units Charging enemy units Engaged in Combat with the bearer's unit must reroll failed Charge Range rolls in the Charge Phase.
    true
}

#[allow(dead_code)]
pub fn runic_standard_of_wisdom(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer may select a single Battle Rune during Spell Selection. This Battle Rune can be cast by the bearer and has Range Caster's Unit.
    true
}

#[allow(dead_code)]
pub fn runic_standard_of_the_hold(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // A unit with one or more Runic Standards of the Hold counts as having an additional Full Rank for the purpose of Steadfast and Disrupted.
    true
}

#[allow(dead_code)]
pub fn iron_husk(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model's Resilience is set to 6.
    true
}

#[allow(dead_code)]
pub fn mirrored_scales(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Each Close Combat Attack allocated towards the model for which a natural ‘1’ is rolled to hit is distributed onto the attacking model's Health Pool.
    true
}

#[allow(dead_code)]
pub fn kaleidoscopic_flesh(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Hard Target (1).
    true
}

#[allow(dead_code)]
pub fn mark_of_the_eternal_champion(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // If the bearer is not a Wizard, it becomes a Wizard Apprentice that does not select spells as normal but always knows Spear of Infinity (Hereditary Spell). If the bearer is already a Wizard, it knows Spear of Infinity in addition to its other spells and cannot select it during Spell Selection.
    true
}

#[allow(dead_code)]
pub fn hammer_hand(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains +1 Attack Value.
    true
}

#[allow(dead_code)]
pub fn unnatural_roots(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // A side with one or more models with Unnatural Roots Engaged in Combat when Combat Scores are calculated adds +1 to its Combat Score.
    true
}

#[allow(dead_code)]
pub fn brimstone_secretions(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Immune (Divine Attacks).
    true
}

#[allow(dead_code)]
pub fn centipede_legs(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains +2in March Rate
    true
}

#[allow(dead_code)]
pub fn chitinous_scales(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains +2 Armour, to a maximum of 3.
    true
}

#[allow(dead_code)]
pub fn dark_hide(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Scout with the following exception: it must be deployed fully inside the owner's Deployment Zone, and the owner must have deployed at least one unit normally.
    true
}

#[allow(dead_code)]
pub fn living_shield(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Parry.
    true
}

#[allow(dead_code)]
pub fn piercing_spike(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Close Combat Attacks made by the model gain +1 Armour Penetration.
    true
}

#[allow(dead_code)]
pub fn greenfire_eyes(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Units containing one or more models with Greenfire Eyes must reroll any natural rolls of ‘1’ when rolling for Charge Range and Pursuit Distance.
    true
}

#[allow(dead_code)]
pub fn venom_sacs(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Poison Attacks. If the model's Close Combat Attacks already were Poison Attacks from another source than this Manifestation, they wound automatically on a successful natural to-hit roll of '5' or '6', unless the target has Immune(Poison Attacks).
    true
}

#[allow(dead_code)]
pub fn digestive_vomit(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // If the model has previously been on the winning side of a combat while having this Manifestation, it gains +1 Strength and +1 Armour Penetration.
    true
}

#[allow(dead_code)]
pub fn broodmother(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // For each Health Point loss the model causes with Close Combat Attacks against enemy units, roll a D6 at the end of the Initiative Step. For each rolled 6+, the model's Health Pool Raises 1 Health Point.
    true
}

#[allow(dead_code)]
pub fn unhinging_jaw(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Failed to-wound rolls from Close Combat Attacks against Large or Gigantic models made by the model must be rerolled.
    true
}

#[allow(dead_code)]
pub fn segmented_shell(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // When the model suffers a wound from an attack with Multiple Wounds (X), reduce X by 1, to a minimum of 1.
    true
}

#[allow(dead_code)]
pub fn divining_snout(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Devastating Charge (+2in Adv) for Charges against units that contain at least one Special Item. The effects only apply if all models in the unit are affected by Divining Snout.
    true
}

#[allow(dead_code)]
pub fn smothering_coils(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains +1 to-wound with Close Combat Attacks against models with Scoring.
    true
}

#[allow(dead_code)]
pub fn mesmerising_plumage(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Enemy units in base contact with one or more models with this Manifestation suffer -1 Offensive Skill and -1 Defensive Skill.
    true
}

#[allow(dead_code)]
pub fn roaming_hands(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // While the unit is Engaged with an enemy unit's Flank or Rear Facing, the model gains +1 Strength and +1 Armour Penetration.
    true
}

#[allow(dead_code)]
pub fn hot_blood(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Devastating Charge (+2 Agi).
    true
}

#[allow(dead_code)]
pub fn chilling_yawn(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Enemy units in base contact with one or more models with this Manifestation suffer -2 Agility.
    true
}

#[allow(dead_code)]
pub fn sorcerous_antennae(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // At the start of Siphon the Veil in each of your Magic Phases, choose a single model part in each unit with one or more instances of this Manifestation. The chosen model part gains Channel (1) until the end of the Magic Phase.
    true
}

#[allow(dead_code)]
pub fn aura_of_despair(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Accurate.
    true
}

#[allow(dead_code)]
pub fn whipcrack_tail(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Lightning Reflexes.
    true
}

#[allow(dead_code)]
pub fn red_haze(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model's Close Combat Attacks gain +1 Strength and +1 Armour Penetration, but each of its Close Combat Attacks with a natural to-hit roll of ‘1’ is distributed onto the attacking model's Health Pool.
    true
}

#[allow(dead_code)]
pub fn incendiary_ichor(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Aegis (3+, against Flaming Attacks). All Melee Attacks (including Special Attacks) and Shooting Attacks made by the model with Incendiary Ichor become Flaming Attacks. The model automatically fails all Fortitude Saves.
    true
}

#[allow(dead_code)]
pub fn bronze_backbone(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Hatred.
    true
}

#[allow(dead_code)]
pub fn stiff_upper_lip(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Units with at least one model with this Manifestation gain Minimised (Discipline Tests).
    true
}

#[allow(dead_code)]
pub fn horns_of_hubris(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Vanguard (6in).
    true
}

#[allow(dead_code)]
pub fn koru_stone(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer's unit gains Rally Around the Flag (12in). If the model is removed as a casualty, the oppponent gains an additional 200 VP.
    true
}

#[allow(dead_code)]
pub fn obelisk_of_collaboration(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer’s unit gains Pack Hunter.
    true
}

#[allow(dead_code)]
pub fn serpents_nest_charm(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // While using this weapon, the wielder’s Attack Value is set to 6 and attacks made with this weapon become Poison Attacks .
    true
}

#[allow(dead_code)]
pub fn glory_of_the_dawn_age(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Attacks made with this weapon gain +1 Strength and +1 Armour Penetration . In addition, attacks made with this weapon for which a successful natural to-wound roll of 5+ was rolled are subject to the following rules: They gain Multiple Wounds (2) Unless the target has Immune (Lethal Strike), their Armour Penetration is always set to 10 and they ignore Fortitude Saves.
    true
}

#[allow(dead_code)]
pub fn alchemical_arrows(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // This weapon has Shots 4, Str 4, AP 1. If the weapon inflicts one or more hits, the Strength of all simultaneously made Shooting Attacks by the bearer’s unit with Magnetic Short Bows is set to 4.
    true
}

#[allow(dead_code)]
pub fn starfall_scales(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The wearer’s model gains Hard Target (1) and Immune (Flaming Attacks).
    true
}

#[allow(dead_code)]
pub fn vital_essence(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The bearer gains Fortitude (4+) and +1 Health Point.
    true
}

#[allow(dead_code)]
pub fn ancient_plaque(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Once per Magic Phase, the bearer may reroll a single Magic Dice when making a casting roll. This ability cannot be used if the spell was Miscast nor for Casting Attempts with only one Magic Dice.
    true
}

#[allow(dead_code)]
pub fn infiltrators_dart(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Right before the battle (during step 7 of the Deployment Phase Sequence), you must mark a single unit from your opponent's Army List with Prey Scent, even if the bearer is Ambushing.
    true
}

#[allow(dead_code)]
pub fn stampede_resonator_crystal(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated at the start of any Charge Phase. Choose one friendly Large Cavalry unit or Gigantic model within 18in of the bearer’s model and apply the following effects (all of them or none) until the end of the Melee Phase: Each model gains Impact Hits (X), where X is equal to its amount of Stomp Attacks. If a model already had Impact Hits, increase its number of Impact Hits by its amount of Stomp Attacks instead. The models cannot perform any Stomp Attacks.
    true
}

#[allow(dead_code)]
pub fn te_aupouri_smokestone(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // One use only. May be activated when a friendly unit fails a Break Test (after any rerolls). Until the start of the next friendly Movement Phase, enemy units within 18in of the bearer’s model gain Minimised (Charge Range, Pursuit Distance, Overrun Distance).
    true
}

#[allow(dead_code)]
pub fn celestial_astrolabe(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The Casting Values of all spells cast by friendly models is reduced by 1.
    true
}

#[allow(dead_code)]
pub fn carved_tablet(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // During Spell Selection, the model must choose one of the spells from Carved Wisdom (see Howdah Devices) that no model with Carved Wisdom chooses. The model can cast the chosen spell as a Bound Spell with Power Level (4/8).
    true
}

#[allow(dead_code)]
pub fn carved_wisdom(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // During Spell Selection, the model must choose one of the spells below. The model can cast the chosen spell as a Bound Spell with Power Level (4/8): Fate's Judgement (Divination) Master of Earth (Druidism) Molten Copper (Alchemy) Swarm of Insects (Shamanism) Touch of the Reaper (Evocation) Each spell can only be chosen by a single model with Carved Wisdom.
    true
}

#[allow(dead_code)]
pub fn engine_of_the_ancients(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Shooting Weapon. Range 12in, Shots 1, Str 6, AP 3, Area Attack (2×2), Lodestone, March and Shoot, Reload!. The attack never suffers negative to-hit modifiers. For the purpose of shooting this weapon, the model can draw Line of Sight in any direction, even outside its Front Arc.
    true
}

#[allow(dead_code)]
pub fn lodestone_shield(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Friendly units within 8in of the model gain Aegis (5+, against Shooting Attacks).
    true
}

#[allow(dead_code)]
pub fn magnetic_great_bow(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Shooting Weapon. Range 18in, Shots 2, Str 4 [5], AP 1 [3], Area Attack (1×5), Lodestone, March and Shoot, Reload!.
    true
}

#[allow(dead_code)]
pub fn monolith_of_vitalism(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model becomes the Battle Standard Bearer.
    true
}

#[allow(dead_code)]
pub fn suncatcher_crystal(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Grind Attacks (2D3) that are always resolved with Strength 4 and Armour Penetration 1. In addition, when calculating Combat Score, a side with one or more Suncatcher Crystals adds +1 to its Combat Score.
    true
}

#[allow(dead_code)]
pub fn venomous_fortress(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model’s base size is changed to 60×100 mm and it gains 6 additional Skink Riders. If applicable, Exclusive (Tegu Warriors, Tegu Guard) is replaced with Exclusive (Skink Warriors, Skink Hunters). In addition, model parts without Harnessed in the bearer’s unit gain Hatred and Poison Attacks.
    true
}

#[allow(dead_code)]
pub fn eidetic_mastery(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model knows two additional Learned Spells.
    true
}

#[allow(dead_code)]
pub fn telepathic_mastery(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // Once per friendly Magic Phase, the model may attempt to cast a single non-Hereditary Learned Spell that was successfully cast by an enemy Wizard during the preceding Magic Phase. If that spell is successfully cast, the model may cast an Attribute Spell that the enemy Wizard knows for the non-Hereditary Learned Spell, if available.
    true
}

#[allow(dead_code)]
pub fn veil_mastery(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains Channel (1). In addition, the owner gains 1 Veil Token every time the model successfully casts a non-Bound and non-Attribute Spell, after resolving the spell’s effect and any Attribute Spell.
    true
}

#[allow(dead_code)]
pub fn mind_shifting_mastery(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // After successfully casting a Learned Spell, the model may choose the Attribute Spell of Alchemy, Divination, Evocation, or Witchcraft. If the Learned Spell has an Attribute Spell that the owner wishes to cast (even if it is identical to the chosen Attribute Spell), the chosen Attribute Spell may be cast immediately before or after casting the Learned Spell's Attribute Spell. Otherwise, the chosen Attribute Spell is cast immediately after resolving the Learned Spell's effects.
    true
}

#[allow(dead_code)]
pub fn eternal_mastery(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model gains a +1 Casting Modifier.
    true
}

#[allow(dead_code)]
pub fn forbidden_mastery(
    _attacker: crate::regiment::Regiment,
    _defender: crate::regiment::Regiment,
) -> bool {
    // The model knows two additional Learned Spells that it must select from the Learned Spells 1, 2, 3, and 4 from Pyromancy, otherwise following the normal Spell Selection rules. In each Magic Phase, when the model successfully casts a spell from Pyromancy for the first time, the model's unit gains Maximised (Discipline Tests) until the start of the next friendly Magic Phase.
    true
}
