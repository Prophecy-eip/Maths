# The goal of this file is to create a Model class that will represent a unit
# The units represented will be used as perceptrons in the neural network

from dataclasses import dataclass
from numpy import uint32
from utils import concatenate_dictionaries, safe_add_signed_unsigned
import numpy as np
import unittest


# Buff the stats with a Global Modifier
@dataclass
class _BaseGlobalModifiers:
    advance: uint32 = 0
    march: uint32 = 0
    discipline: uint32 = 0

# Buff the stats with an offensive modifier
@dataclass
class _BaseOffensiveModifiers:
    attack: uint32 = 0
    offensive: uint32 = 0
    strength: uint32 = 0
    armour_penetration: uint32 = 0
    agility: uint32 = 0

# Buff the stats with a defensive modifier
@dataclass
class _BaseDefensiveModifiers:
    health_points: uint32 = 0
    defense: uint32 = 0
    resilience: uint32 = 0
    armour: uint32 = 0

@dataclass
class _BaseWeaponModifiers:
    armour_penetration: uint32 = 0
    strength: uint32 = 0
    shots: uint32 = 0

# Gather all the modifiers into a dataclass
@dataclass
class Modifiers :
    global_modifiers: _BaseGlobalModifiers = _BaseGlobalModifiers()
    offensive_modifiers: _BaseOffensiveModifiers = _BaseOffensiveModifiers()
    defensive_modifiers: _BaseDefensiveModifiers = _BaseDefensiveModifiers()
    weapon_modifiers: _BaseWeaponModifiers = _BaseWeaponModifiers()

    def get_values(self, data_class_instance):
        values = dict()
        for attr in dir(data_class_instance):
            if not attr.startswith('__') and not attr.startswith('get'):
                value = getattr(data_class_instance, attr)
                values[attr] = value
        return values

## Class containing all the statistics in the game for a unit
@dataclass
class Stats:
    # advance (usize): The distance the Model can advance per turn
    advance: uint32
    # march (usize): The distance the Model can forcefully advance per turn
    march: uint32
    # discipline (usize): The discipline of the Model
    discipline: uint32
    # health_points (usize): The number of hit the Model can endure before being removed from the Regiment
    health_points: uint32
    # defense (usize): The defense of the Model
    defense: uint32
    # resilience (usize): The resilience of the Model
    resilience: uint32
    # armour (usize): The armour of the Model
    armour: uint32
    # aegis (usize): The special armour of the Model
    aegis: uint32
    # attack (usize): The number of attack the Model do
    attack: uint32
    # offensive (usize): The offensive of the Model
    offensive: uint32
    # strength (usize): The strength of the Model
    strength: uint32
    # amour_penetration (usize): The strength of the Model
    armour_penetration: uint32
    # agility (usize): The agility of the Model
    agility: uint32
    # ballistic_skill (usize): The ballistic skill of the Model
    ballistic_skill: uint32
    # shots (usize): The number of shots the Model can do depending on the weapon
    shots: uint32 = 0

    # This function will take up any type of modifiers as long as it has the 'Modifiers' type, then apply the stats changes if there is any
    def safe_apply_modifiers(self, modifiers : Modifiers):
        global_values = modifiers.get_values(modifiers.global_modifiers)
        offensive_values = modifiers.get_values(modifiers.offensive_modifiers)
        defensive_values = modifiers.get_values(modifiers.defensive_modifiers)
        weapon_values = modifiers.get_values(modifiers.weapon_modifiers)

        stats_modifiers_values = concatenate_dictionaries(global_values, offensive_values, defensive_values, weapon_values)
        for key, val, in stats_modifiers_values.items():
            setattr(self, key, safe_add_signed_unsigned(getattr(self, key), val)) # a safe_addition on the last parameters here to update the class


# Create a new model for the units using the Stats and a list of modifiers
@dataclass
class Model:
    stats : Stats
    boosted_stats : Stats
    modifiers : list[Modifiers]
    banner_bearer: bool

## UNIT TESTING
class TestInitModifiers(unittest.TestCase):

    def test_all_stats_init(self):
        s = Stats(
            advance=5,
            march=5,
            discipline=5,
            health_points=5,
            defense=5,
            resilience=5,
            armour=5,
            aegis=5,
            attack=5,
            offensive=5,
            strength=5,
            armour_penetration=5,
            agility=4,
            ballistic_skill=5
        )
        self.assertEqual(s.advance, 5)
        self.assertEqual(s.march, 5)
        self.assertEqual(s.agility, 4)
        self.assertEqual(s.discipline, 5)

    def test_global_modifiers(self):
        global_mod1 = _BaseGlobalModifiers(
            advance=2,
            march=3,
            discipline=3
        )
        m = Modifiers(
            global_modifiers=global_mod1
        )
        self.assertEqual(m.global_modifiers.advance, 2)
        self.assertEqual(m.global_modifiers.march, 3)
        self.assertEqual(m.global_modifiers.discipline, 3)

    def test_all_stats_and_modifiers(self):
        s = Stats(
            advance=5,
            march=5,
            discipline=5,
            health_points=5,
            defense=5,
            resilience=5,
            armour=5,
            aegis=5,
            attack=5,
            offensive=5,
            strength=5,
            armour_penetration=5,
            agility=4,
            ballistic_skill=5
        )
        global_mod1 = _BaseGlobalModifiers(
            advance=2,
            march=3,
            discipline=3
        )
        m = Modifiers(
            global_modifiers=global_mod1
        )

        s.safe_apply_modifiers(m)
        self.assertEqual(s.advance, 7)
        self.assertEqual(s.march, 8)
        self.assertEqual(s.discipline, 8)


if __name__ == '__main__':
    unittest.main()
