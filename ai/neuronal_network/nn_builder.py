import json
import numpy as np
from utils import align_lists
from keras.layers import Input, Dense, Flatten
from keras.models import Model
import os

# The absolute path to the current file
ABSOLUTE_PATH = os.path.dirname(os.path.abspath(__file__))

if not ABSOLUTE_PATH.endswith('neuronal_network'):
    ABSOLUTE_PATH = os.path.join(ABSOLUTE_PATH, 'ai', 'neuronal_network')

# The data loaded from the json file
JSON_DATA = json.load(
    open(os.path.join(ABSOLUTE_PATH, 'trainning_data', 'trainning_data.json'), 'r')
)

# Size of the biggest army
MAX_ARMY_SIZE = 0

# Number of stats for each unit
NB_STAT = 12


def compute_nb_stat(p1, p2):
    """This function compute the highest number between the number of stats a unit can have and the max number of modifiers a unit handles

    Args:
        p1 (list(dict({
            "stat": list(int),
            "modifiers": list(int)
            }))): first player's army
        p2 (list(dict({
            "stat": list(int),
            "modifiers": list(int)
            }))): second player's army

    Returns:
        int: The highest number between the number of stats a unit can have and the max number of modifiers a unit handles
    """
    len1 = NB_STAT
    len2 = NB_STAT

    for unit in p1:
        if len(unit['stat']) > len1:
            len1 = len(unit['stat'])
    for unit in p2:
        if len(unit['stat']) > len2:
            len2 = len(unit['stat'])
    return max(len1, len2)


def pad_army(player, max_len):
    """This function pad the matrix representing the units forming a player army

    Args:
        army (list(dict({
            "stat": list(int),
            "modifiers": list(int)
            }))): The army to pad
        max_len (int): The max length of informations to store per unit

    Returns:
        army (list(dict({
            "stat": list(int),
            "modifiers": list(int)
            }))): The army padded
    """
    for unit in player:
        unit['stat'] += [0] * (max_len - len(unit['stat']))
        unit['modifiers'] += [0] * (max_len - len(unit['modifiers']))
    return player


def format_json_match(match):
    """This function format a match from the json format to a more usable format

    Args:
        match (dict({
            "first_player": dict({
                "score": int,
                "modifiers": list(int),
                "units": list(dict({name: str, stat: Stat})),
                "cost": int
                }),
            "second_player": dict({
                "score": int,
                "modifiers": list(int),
                "units": list(dict({name: str, stat: Stat})),
                "cost": int
                }),
            "map": int
            })): a match in the json provided

    Returns:
        {
            'first_player_units': list(dict({
            "stat": list(int),
            "modifiers": list(int)
            })),
            })),
            'second_player_units': list(dict({
            "stat": list(int),
            "modifiers": list(int)
            })),
            'first_player_score': int,
            'second_player_score': int,
            'first_player_cost': int,
            'second_player_cost': int
        } | None: The match in a more usable format
    """

    def extract_player_data(player):
        player_units = []
        for unit in player['units']:
            if unit is not None:
                player_units.append(
                    {
                        'stat': list(unit['stat'].values()),
                        'modifiers': player['modifiers'],
                    }
                )
        for unit in player_units:
            aligned_version = align_lists(unit['stat'], unit['modifiers'], [0])
            unit['stat'] = aligned_version[0]
            unit['modifiers'] = aligned_version[1]
        return player_units

    first_player_units = extract_player_data(match['first_player'])
    second_player_units = extract_player_data(match['second_player'])

    if len(first_player_units) == 0 or len(second_player_units) == 0:
        return None
    return {
        'first_player_units': first_player_units,
        'second_player_units': second_player_units,
        'first_player_score': match['first_player']['score'],
        'second_player_score': match['second_player']['score'],
        'first_player_cost': match['first_player']['cost'],
        'second_player_cost': match['second_player']['cost'],
    }


def purge_data(data):
    """This function purge the data from the json file, removing None, empty units, units with missing stats and matchs with score != 20

    Args:
        data (list({
            'first_player_units': list(dict({
            "name": str,
            "stat": Stat,
            "cost": int
            })),
            'second_player_units': list(dict({
            "name": str,
            "stat": Stat,
            "cost": int
            })),
            'first_player_score': int,
            'second_player_score': int,
            'first_player_cost': int,
            'second_player_cost': int
        })): The data to purge

    Returns:
        (list({
            'first_player_units': list(dict({
            "name": str,
            "stat": Stat,
            "cost": int
            })),
            'second_player_units': list(dict({
            "name": str,
            "stat": Stat,
            "cost": int
            })),
            'first_player_score': int,
            'second_player_score': int,
            'first_player_cost': int,
            'second_player_cost': int
        })): The purged data
    """
    # Remove None
    data = list(filter(lambda x: x is not None, data))
    # Remove empty units
    data = list(filter(lambda x: len(x['first_player_units']) != 0, data))
    data = list(filter(lambda x: len(x['second_player_units']) != 0, data))
    # Remove units with missing stats
    data = list(
        filter(
            lambda x: not any(
                len(x['stat']) < NB_STAT for x in x['first_player_units']
            ),
            data,
        )
    )
    data = list(
        filter(
            lambda x: not any(
                len(x['stat']) < NB_STAT for x in x['second_player_units']
            ),
            data,
        )
    )
    # Remove matchs with score != 20
    data = list(
        filter(lambda x: x['first_player_score'] + x['second_player_score'] == 20, data)
    )
    return data


def format_matches(matches):
    """This function create the inputs for the neuronal network out of the matches

    Args:
        matchs (list({
            'first_player_units': list(dict({
            "name": str,
            "stat": Stat,
            "cost": int
            })),
            'second_player_units': list(dict({
            "name": str,
            "stat": Stat,
            "cost": int
            })),
            'first_player_score': int,
            'second_player_score': int,
            'first_player_cost': int,
            'second_player_cost': int
        })): The matches to format

    Returns:
        (np.array, np.array, int): The inputs, the outputs and the max length of the armies
    """
    armies_len = 22
    equipment_len = 1000  # Arbitrary value for now

    for match in matches:
        armies_len = max(
            armies_len,
            len(match['first_player_units']),
            len(match['second_player_units']),
        )
        equipment_len = max(
            equipment_len,
            compute_nb_stat(match['first_player_units'], match['second_player_units']),
        )

    for match in matches:
        match['first_player_units'] = pad_army(
            match['first_player_units'], equipment_len
        )
        match['second_player_units'] = pad_army(
            match['second_player_units'], equipment_len
        )

    for match in matches:
        match['first_player_units'] += [
            {'stat': [0] * equipment_len, 'modifiers': [0] * equipment_len}
        ] * (armies_len - len(match['first_player_units']))
        match['second_player_units'] += [
            {'stat': [0] * equipment_len, 'modifiers': [0] * equipment_len}
        ] * (armies_len - len(match['second_player_units']))

    units = []
    scores = []

    for match in matches:
        units.append(
            [
                np.array(
                    [
                        [x['stat'] for x in match['first_player_units']],
                        [x['modifiers'] for x in match['first_player_units']],
                    ]
                ),
                np.array(
                    [
                        [x['stat'] for x in match['second_player_units']],
                        [x['modifiers'] for x in match['second_player_units']],
                    ]
                ),
            ]
        )
        scores.append([match['first_player_score']])
    return (np.array(units), np.array(scores), armies_len, equipment_len)


def build_neuronal_network(shape):
    """Build the neuronal network

    Args:
        shape (Keras.shape): The shape of the NN's input

    Returns:
        Keras.Model: The built model
    """
    InputModel = Input(shape=shape)
    EncodedLayer = Dense(20, activation='softmax')(InputModel)
    EncodedLayer = Dense(20, activation='softmax')(EncodedLayer)
    EncodedLayer = Dense(20, activation='softmax')(EncodedLayer)
    EncodedLayer = Dense(20, activation='softmax')(EncodedLayer)
    EncodedLayer = Dense(20, activation='softmax')(EncodedLayer)
    EncodedLayer = Dense(20, activation='softmax')(EncodedLayer)
    EncodedLayer = Flatten()(EncodedLayer)
    DecodedLayer = Dense(1)(EncodedLayer)
    AutoEncoder = Model(InputModel, DecodedLayer)
    AutoEncoder.compile(optimizer='adam', loss='mse', metrics=['accuracy'])
    return AutoEncoder


if __name__ == '__main__':
    matchs = []
    nb_batch_size = 1
    nb_epoch = 600
    model = None

    for match in JSON_DATA:
        matchs.append(format_json_match(match))
    matchs = purge_data(matchs)
    (units, scores, armies_len, equipment_len) = format_matches(matchs)
    MAX_ARMY_SIZE = armies_len
    model = build_neuronal_network((2, 2, MAX_ARMY_SIZE, equipment_len))
    model.fit(units, scores, batch_size=nb_batch_size, epochs=nb_epoch)
    model.save('./trainning_data/model.h5')
