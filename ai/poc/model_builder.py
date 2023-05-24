from keras.layers import Input, Dense, Flatten
from keras.models import Model
import numpy as np
import json

json_data = json.load(
    open('./trainning_data/trainning_data.json'))

data = []


def clean_data(data):
    for match in data:
        if match[0] == 0 or match[1] == 0:
            data.remove(match)
            continue
        if match[2] == 0 or match[3] == 0:
            data.remove(match)
            continue
        if any(len(x) != 15 for x in match[0]) or any(len(x) != 15 for x in match[1]):
            data.remove(match)
            continue
    print(len(data))
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

for match in json_data:
    sample = match_to_data(match)
    data.append(sample)


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
            continue
        units.append([np.array(match[0]), np.array(match[1])])
        scores.append(np.array([match[2], match[3]]))
        max_len = max(first_len, second_len, max_len)

    for match in units:
        match[0] = np.pad(match[0], ((0, max_len - len(match[0])), (0, 0)), 'constant')
        match[1] = np.pad(match[1], ((0, max_len - len(match[1])), (0, 0)), 'constant')
    return (np.array(units), np.array(scores))


data = clean_data(data)
(x_train, y_train) = format_data(data)


if __name__ == '__main__':
    InputModel = Input(shape=(2,22,15))
    EncodedLayer = Dense(15, activation='relu')(InputModel)
    EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
    EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
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

    AutoEncoder.save('./train_results/model.h5')