from keras.layers import Input, Dense, Flatten
from keras.models import Model
import numpy as np
import json

json_data = json.load(
    open('./trainning_data/trainning_data.json'))

nb_stats = 15
max_army_size = 22


def clean_data(data):
    for match in data:
        if len(match[0]) == 0 or len(match[1]) == 0:
            data.remove(match)
            continue
        if int(match[2]) + int(match[3]) != 20:
            data.remove(match)
            continue
        if any(len(x) != nb_stats for x in match[0]) or any(len(x) != nb_stats for x in match[1]):
            data.remove(match)
            continue
    return data


def match_to_data(match):
    first_x = []
    second_x = []

    for unit in match['first_player']['units']:
        if unit is not None:
            first_x.append(list(unit['stat'].values()))
    for unit in match['second_player']['units']:
        if unit is not None:
            second_x.append(list(unit['stat'].values()))

    sample = (
        first_x,
        second_x,
        match['first_player']['score'],
        match['second_player']['score']
    )
    return sample


def format_data(data):
    units = []
    scores = []
    first_len = 0
    second_len = 0
    max_len = 0

    for match in data:
        first_len = len(match[0])
        second_len = len(match[1])
        if first_len == 0 or second_len == 0:
            print('empty match: ', match)
            continue
        units.append([np.array(match[0]), np.array(match[1])])
        scores.append(np.array([match[2], match[3]]))
        max_len = max(first_len, second_len, max_len)

    for match in units:
        match[0] = np.pad(
            match[0], ((0, max_len - len(match[0])), (0, 0)), 'constant')
        match[1] = np.pad(
            match[1], ((0, max_len - len(match[1])), (0, 0)), 'constant')
    print('units len: ', len(units))
    print('scores len: ', len(scores))
    return (np.array(units), np.array(scores))


if __name__ == '__main__':

    data = [match_to_data(match) for match in json_data]
    data = clean_data(data)
    print('data len: ', len(data))
    (x_train, y_train) = format_data(data)
    InputModel = Input(shape=(2, max_army_size, nb_stats))

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

    nb_batch_size = 1
    nb_epoch = 600

    AutoEncoder.fit(x_train, y_train, batch_size=nb_batch_size,
                    epochs=nb_epoch, shuffle=True, validation_data=(x_train, y_train))

    AutoEncoder.save('./train_results/models.h5')
