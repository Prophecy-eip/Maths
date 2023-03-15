from keras.layers import Input, Dense
from keras.models import Model
import numpy as np
import json

class army:
    def __init__(self, modifiers, units):
        self.modifiers = modifiers
        self.units = units

    def __repr__(self):
        return 'modifiers: {}\nunits: {}'.format(self.modifiers, self.units)

scores = []
armies = []

# load_data from json file
# load_data returns a tuple of Numpy arrays: (x_train, y_train), (x_test, y_test).
def load_data():
    # with open('ai_model_building/examples/input.example.json') as json_file:
    with open('ai_model_building/trainning_data/trainning_data.json') as json_file:
        data = json.load(json_file)
        for i in range(len(data)):
            first_player = data[i]['first_player']
            second_player = data[i]['second_player']
            first_player_score = data[i]['first_player']['score']
            second_player_score = data[i]['second_player']['score']
            scores.append((first_player_score, second_player_score))
            first_player_army = army(first_player['modifiers'], first_player['units'])
            second_player_army = army(second_player['modifiers'], second_player['units'])
            armies.append((first_player_army, second_player_army))
            # [0][0][0] -> first player modifiers
            # [0][0][1] -> first player units
            # [0][1][0] -> second player modifiers
            # [0][1][1] -> second player units
        scores_np = np.array(scores)
        scores_np = scores_np.astype('int32')
        armies_np = np.array(armies)
        armies_np = armies_np.astype('object')
        return (armies_np, scores_np)

(x_train, y_train) = load_data()
InputModel = Input(shape=(40, 15,))
EncodedLayer = Dense(15, activation='relu')(InputModel)
EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
DecodedLayer = Dense(2, activation='relu')(EncodedLayer)
AutoEncoder = Model(InputModel, DecodedLayer)
AutoEncoder.compile(optimizer='adam', loss='mse')

nb_batch_size = 1
nb_epoch = 300

AutoEncoder.fit(x_train, x_train, batch_size=nb_batch_size, epochs=nb_epoch, shuffle=True, validation_data=(x_train, x_train))
