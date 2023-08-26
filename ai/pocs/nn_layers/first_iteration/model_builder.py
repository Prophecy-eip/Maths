from keras.layers import Input, Dense
from keras.models import Model
import numpy as np
import json
import os

# The absolute path to the current file
ABSOLUTE_PATH = os.path.dirname(os.path.abspath(__file__))

# If the file is not in the nn_layers folder, we add it
if not ABSOLUTE_PATH.endswith('first_iteration'):
    ABSOLUTE_PATH = os.path.join(
        ABSOLUTE_PATH, 'ai', 'pocs', 'nn_layers', 'first_iteration'
    )

# The data loaded from the json file
JSON_DATA = json.load(
    open(os.path.join(ABSOLUTE_PATH, 'trainning_data', 'trainning_data.json'), 'r')
)

# Number of stats for each unit
NB_STAT = 15

# Size of the biggest army
MAX_ARMY_SIZE = 22

# Number of batch to train on
BATCH_SIZE = 1

# Number of epoch to train on
EPOCH = 600


def match_to_data(match):
    """format the json data to a more usable format

    Args:
        match (match): A match from the json data

    Returns:
        (
            list(dict({
            "name": str,
            "stat": Stat
            })),
            list(dict({
            "name": str,
            "stat": Stat
            })),
            int,
            int)
        ): The formatted match
    """
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
        match['second_player']['score'],
    )
    return sample


def clean_data(data):
    """Remove invalid data from the dataset

    Args:
        data (list((
            list(dict({
            "name": str,
            "stat": Stat
            })),
            list(dict({
            "name": str,
            "stat": Stat
            })),
            int,
            int)
        ))): The dataset

    Returns:
        list((
            list(dict({
            "name": str,
            "stat": Stat
            })),
            list(dict({
            "name": str,
            "stat": Stat
            })),
            int,
            int)
        )): The dataset without invalid data
    """
    for match in data:
        if len(match[0]) == 0 or len(match[1]) == 0:
            data.remove(match)
            continue
        if int(match[2]) + int(match[3]) != 20:
            data.remove(match)
            continue
        if any(len(x) != NB_STAT for x in match[0]) or any(
            len(x) != NB_STAT for x in match[1]
        ):
            data.remove(match)
            continue
    return data


def format_data(data):
    """build the model's trainning data from the dataset

    Args:
        data (
            list(dict({
                "name": str,
                "stat": Stat
                })),
                list(dict({
                "name": str,
                "stat": Stat
                })),
                int,
                int)
        ): The dataset formatted

    Returns:
        (np.array(np.array(int)), np.array(int)): A tuple with 1) The units to train on and 2) The scores to train on
    """
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
        match[0] = np.pad(match[0], ((0, max_len - len(match[0])), (0, 0)), 'constant')
        match[1] = np.pad(match[1], ((0, max_len - len(match[1])), (0, 0)), 'constant')
    return (np.array(units), np.array(scores))


if __name__ == '__main__':
    data = [match_to_data(match) for match in JSON_DATA]
    data = clean_data(data)
    (x_train, y_train) = format_data(data)
    InputModel = Input(shape=(2, MAX_ARMY_SIZE, NB_STAT))
    EncodedLayer = Dense(15, activation='relu')(InputModel)
    EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
    EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
    EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
    EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
    EncodedLayer = Dense(15, activation='relu')(EncodedLayer)
    DecodedLayer = Dense(2, activation='relu')(EncodedLayer)
    AutoEncoder = Model(InputModel, DecodedLayer)
    AutoEncoder.compile(optimizer='adam', loss='mse')
    AutoEncoder.fit(
        x_train,
        y_train,
        batch_size=BATCH_SIZE,
        epochs=EPOCH,
        shuffle=True,
        validation_data=(x_train, y_train),
    )
    AutoEncoder.save(os.path.join(ABSOLUTE_PATH, 'train_results', 'models.h5'))
