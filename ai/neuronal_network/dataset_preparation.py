import json
import numpy as np
from keras.layers import Input, Dense, Flatten
from keras.models import Model

json_data = json.load(open('./trainning_data/trainning_data.json'))
armies_length = 0
nb_stats = 15


def format_json_match(match):
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
    army_max_cost = 4500
    threshold = 0.8
    # Remove None
    data = list(filter(lambda x: x is not None, data))
    # Remove empty units
    data = list(filter(lambda x: len(x['first_player_units']) != 0, data))
    data = list(filter(lambda x: len(x['second_player_units']) != 0, data))
    # Remove units with missing stats
    data = list(filter(lambda x: not any(
        len(x) != nb_stats for x in x['first_player_units']), data))
    data = list(filter(lambda x: not any(
        len(x) != nb_stats for x in x['second_player_units']), data))
    # Remove matchs with score != 20
    data = list(filter(
        lambda x: x['first_playser_score'] + x['second_player_score'] == 20, data))
    # Remove matchs with cost > 4500 or < 3600
    data = list(filter(lambda x: x['first_player_cost'] >
                army_max_cost and x['first_player_cost'] < threshold * army_max_cost, data))
    data = list(filter(lambda x: x['second_player_cost'] >
                army_max_cost and x['second_player_cost'] < threshold * army_max_cost, data))
    return data


def format_matchs(matchs):
    max_army_len = 0
    first_army_len = 0
    second_army_len = 0
    scores = []
    units = []

    for match in matchs:
        first_army_len = len(match['first_player_units'])
        second_army_len = len(match['second_player_units'])
        units.append([np.array(match['first_player_units']),
                     np.array(match['second_player_units'])])
        scores.append([match['first_player_score'],
                      match['second_player_score']])
        max_army_len = max(first_army_len, second_army_len, max_army_len)

    for match in units:
        match[0] = np.pad(
            match[0], ((0, max_army_len - len(match[0])), (0, 0)), 'constant')
        match[1] = np.pad(
            match[1], ((0, max_army_len - len(match[1])), (0, 0)), 'constant')
    return (np.array(units), np.array(scores), max_army_len)


def neuronal_network_build(shape):
    InputModel = Input(shape=shape)
    EncodedLayer = Dense(15, activation='relu')(InputModel)
    EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
    EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
    EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
    EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
    EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
    EncodedLayer = Flatten()(EncodedLayer)
    DecodedLayer = Dense(2, activation='relu')(EncodedLayer)
    AutoEncoder = Model(InputModel, DecodedLayer)
    AutoEncoder.compile(optimizer='adam', loss='mse')
    return AutoEncoder


if __name__ == '__main__':
    matchs = []
    nb_batch_size = 1
    nb_epoch = 600
    model = None

    for match in json_data:
        matchs.append(format_json_match(match))
    matchs = purge_data(matchs)
    (units, scores, armies_len) = format_matchs(matchs)
    armies_length = armies_len
    model = neuronal_network_build((2, armies_length, nb_stats))
    model.fit(units, scores, batch_size=nb_batch_size, epochs=nb_epoch)
    model.save('./trainning_data/model.h5')
