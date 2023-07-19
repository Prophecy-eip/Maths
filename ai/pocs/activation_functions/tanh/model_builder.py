import json
import numpy as np
from keras.layers import Input, Dense, Flatten
from keras.models import Model
import os

# The absolute path to the current file
ABSOLUTE_PATH = os.path.dirname(os.path.abspath(__file__))

if not ABSOLUTE_PATH.endswith('tanh'):
    ABSOLUTE_PATH = os.path.join(ABSOLUTE_PATH, 'ai', 'pocs', 'activation_functions', 'tanh')

# The data loaded from the json file
JSON_DATA = json.load(open(os.path.join(ABSOLUTE_PATH, 'trainning_data', 'trainning_data.json'), 'r'))

# Size of the biggest army
MAX_ARMY_SIZE = 0

# Number of stats for each unit
NB_STAT = 12


def format_json_match(match):
    """This function format a match from the json format to a more usable format

    Args:
        match (match): a match in the json provided

    Returns:
        {
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
        } | None: The match in a more usable format
    """
    first_player_units = []
    second_player_units = []

    for unit in match['first_player']['units']:
        if unit is not None:
            first_player_units.append(list(unit['stat'].values()))
    for unit in match['second_player']['units']:
        if unit is not None:
            second_player_units.append(list(unit['stat'].values()))

    if len(first_player_units) == 0 or len(second_player_units) == 0:
        return None
    return {
        'first_player_units': first_player_units,
        'second_player_units': second_player_units,
        'first_player_score': match['first_player']['score'],
        'second_player_score': match['second_player']['score'],
        'first_player_cost': match['first_player']['cost'],
        'second_player_cost': match['second_player']['cost']
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
    data = list(filter(lambda x: not any(
        len(x) != NB_STAT for x in x['first_player_units']), data))
    data = list(filter(lambda x: not any(
        len(x) != NB_STAT for x in x['second_player_units']), data))
    # Remove matchs with score != 20
    data = list(filter(
        lambda x: x['first_player_score'] + x['second_player_score'] == 20, data))
    return data


def format_matchs(matches):
    """This function create the inputs and outputs for the neuronal network out of the matches

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
    max_army_len = 0
    first_army_len = 0
    second_army_len = 0
    scores = []
    units = []

    for match in matches:
        first_army_len = len(match['first_player_units'])
        second_army_len = len(match['second_player_units'])
        units.append([np.array(match['first_player_units']),
                     np.array(match['second_player_units'])])
        scores.append(match['first_player_score'])
        max_army_len = max(first_army_len, second_army_len, max_army_len)

    for match in units:
        match[0] = np.pad(
            match[0], ((0, max_army_len - len(match[0])), (0, 0)), 'constant')
        match[1] = np.pad(
            match[1], ((0, max_army_len - len(match[1])), (0, 0)), 'constant')
    return (np.array(units), np.array(scores), max_army_len)


def neuronal_network_build(shape):
    """Build the neuronal network

    Args:
        shape (Keras.shape): The shape of the NN's input

    Returns:
        Keras.Model: The built model
    """
    InputModel = Input(shape=shape)
    EncodedLayer = Dense(15, activation='tanh')(InputModel)
    EncodedLayer = Dense(15, activation='tanh')(EncodedLayer)
    EncodedLayer = Dense(15, activation='tanh')(EncodedLayer)
    EncodedLayer = Dense(15, activation='tanh')(EncodedLayer)
    EncodedLayer = Dense(15, activation='tanh')(EncodedLayer)
    EncodedLayer = Dense(15, activation='tanh')(EncodedLayer)
    EncodedLayer = Flatten()(EncodedLayer)
    DecodedLayer = Dense(21, activation='softmax')(EncodedLayer)
    AutoEncoder = Model(InputModel, DecodedLayer)
    AutoEncoder.compile(optimizer='adam', loss='mse')
    return AutoEncoder


if __name__ == '__main__':
    matchs = []
    nb_batch_size = 1
    nb_epoch = 600
    model = None

    for match in JSON_DATA:
        matchs.append(format_json_match(match))
    matchs = purge_data(matchs)
    (units, scores, armies_len) = format_matchs(matchs)
    MAX_ARMY_SIZE = armies_len
    model = neuronal_network_build((2, MAX_ARMY_SIZE, NB_STAT))
    model.fit(units, scores, batch_size=nb_batch_size, epochs=nb_epoch)
    model.save('./trainning_data/model.h5')
