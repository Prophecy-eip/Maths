# from keras.layers import Input, Dense, Flatten
# from keras.models import Model
# import numpy as np
# import json

# json_data = json.load(
#     open('./trainning_data/trainning_data.json'))

# data = []


# def clean_data(data):
#     for match in data:
#         if match[0] == 0 or match[1] == 0:
#             data.remove(match)
#             continue
#         if match[2] == 0 or match[3] == 0:
#             data.remove(match)
#             continue
#         if any(len(x) != 15 for x in match[0]) or any(len(x) != 15 for x in match[1]):
#             data.remove(match)
#             continue
#     return data


# def match_to_data(match):
#     first_x = []
#     second_x = []

#     for unit in match['first_player']['units']:
#         if unit is not None:
#             first_x.append(list(unit['stat'].values()))
#     for unit in match['second_player']['units']:
#         if unit is not None:
#             second_x.append(list(unit['stat'].values()))

#     sample = (
#         first_x,
#         second_x,
#         match['first_player']['score'],
#     )
#     return sample

# for match in json_data:
#     sample = match_to_data(match)
#     data.append(sample)

# def format_data(data):
#     units = []
#     scores = []
#     first_len = 0
#     second_len = 0
#     max_len = 0

#     for match in data:
#         first_len = len(match[0])
#         second_len = len(match[1])
#         if first_len == 0 or second_len == 0:
#             print('empty match: ', match)
#             continue
#         units.append([np.array(match[0]), np.array(match[1])])
#         scores.append(np.array(match[2]))
#         max_len = max(first_len, second_len, max_len)

#     for match in units:
#         match[0] = np.pad(match[0], ((0, max_len - len(match[0])), (0, 0)), 'constant')
#         match[1] = np.pad(match[1], ((0, max_len - len(match[1])), (0, 0)), 'constant')
#     print('units len: ', len(units))
#     print('scores len: ', len(scores))
#     return (np.array(units), np.array(scores))


# data = clean_data(data)
# (x_train, y_train) = format_data(data)


# if __name__ == '__main__':
#     InputModel = Input(shape=(1,22,15))
#     EncodedLayer = Dense(units=20, activation='softmax')(InputModel)
#     EncodedLayer = Dense(units=20, activation='softmax')(EncodedLayer)
#     EncodedLayer = Dense(units=20, activation='softmax')(EncodedLayer)
#     EncodedLayer = Dense(units=20, activation='softmax')(EncodedLayer)
#     EncodedLayer = Dense(units=20, activation='softmax')(EncodedLayer)
#     EncodedLayer = Dense(units=20, activation='softmax')(EncodedLayer)
    # DecodedLayer = Flatten()(EncodedLayer)
    # AutoEncoder = Model(InputModel, DecodedLayer)
    # AutoEncoder.compile(optimizer='adam', loss='categorical_crossentropy', metrics=['accuracy'])

    # nb_batch_size = 1
    # nb_epoch = 600

    # AutoEncoder.fit(x_train, y_train, batch_size=nb_batch_size,
    #                 epochs=nb_epoch, shuffle=True, validation_data=(x_train, y_train))

    # AutoEncoder.save('./train_results/models.h5')


import json
import numpy as np
from keras.layers import Input, Dense, Flatten
from keras.models import Model

json_data = json.load(open('./trainning_data/trainning_data.json'))
armies_length = 0
nb_stats = 12


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
    if match ['first_player']['score'] + match['second_player']['score'] != 20:
        return None
    return {
        'first_player_units': first_player_units,
        'second_player_units': second_player_units,
        'first_player_score': match['first_player']['score'],
        'first_player_cost': match['first_player']['cost'],
        'second_player_cost': match['second_player']['cost']
    }


def purge_data(data):
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
        scores.append(match['first_player_score'])
        max_army_len = max(first_army_len, second_army_len, max_army_len)

    for match in units:
        match[0] = np.pad(
            match[0], ((0, max_army_len - len(match[0])), (0, 0)), 'constant')
        match[1] = np.pad(
            match[1], ((0, max_army_len - len(match[1])), (0, 0)), 'constant')
    return (np.array(units), np.array(scores), max_army_len)


def neuronal_network_build(shape):
    InputModel = Input(shape=shape)
    EncodedLayer = Dense(units=20, activation='softmax')(InputModel)
    EncodedLayer = Dense(units=20, activation='softmax')(EncodedLayer)
    EncodedLayer = Dense(units=20, activation='softmax')(EncodedLayer)
    EncodedLayer = Dense(units=20, activation='softmax')(EncodedLayer)
    EncodedLayer = Dense(units=20, activation='softmax')(EncodedLayer)
    EncodedLayer = Dense(units=20, activation='softmax')(EncodedLayer)
    EncodedLayer = Flatten()(EncodedLayer)
    DecodedLayer = Dense(units=1, activation='softmax')(EncodedLayer)
    AutoEncoder = Model(InputModel, DecodedLayer)
    AutoEncoder.compile(optimizer='adam', loss='categorical_crossentropy', metrics=['accuracy'])
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
    print(model.summary())
    print('first score: ', scores[0])
    print('score shape: ', scores.shape)
    model.fit(units, scores, batch_size=nb_batch_size, epochs=nb_epoch)
    model.save('./trainning_data/model.h5')
