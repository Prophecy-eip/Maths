import json
import numpy as np
from keras.models import load_model
import random
import os
import sys

sys.path.append(os.path.join(os.path.dirname(__file__), '..', '..', '..', '..'))

from ai.neuronal_network.model_builder import format_json_match, purge_data, format_matchs

# The absolute path to the current file
ABSOLUTE_PATH = os.path.dirname(os.path.abspath(__file__))
print(ABSOLUTE_PATH)

if not ABSOLUTE_PATH.endswith('tanh'):
    ABSOLUTE_PATH = os.path.join(ABSOLUTE_PATH, 'ai', 'pocs', 'activation_functions', 'tanh')

# The data loaded from the json file
DATA = json.load(open(os.path.join(ABSOLUTE_PATH, 'trainning_data', 'trainning_data.json'), 'r'))

# The model to use
MODEL = load_model(os.path.join(ABSOLUTE_PATH, 'trainning_data', 'model.h5'))

# The number of matchs to use for the test
TEST_SIZE = 1000


def build_test_dataset():
    """This function build a test dataset from the trainning data

    Returns:
        list({
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
        } | None): A list of matchs according to the format of the trainning data
    """
    test_dataset = []
    for i in range(TEST_SIZE):
        match = format_json_match(random.choice(DATA))
        if match is not None:
            test_dataset.append(match)
    test_dataset = purge_data(test_dataset)
    return test_dataset


def compute_predictions(model, units):
    """This function compute the predictions of the model for a list of units

    Args:
        model (keras.model): The model to use
        units (list(unit)): The list of units to predict

    Returns:
        list(list(int)): The list of predictions
    """
    points = []
    for x in units:
        prediction = model.predict(np.array([x]))
        points.append(list(prediction[0]).index(max(prediction[0])))
    return points



if __name__ == '__main__':
    test_dataset = build_test_dataset()
    # last parameter here is the size of the largest army. Useless here
    (units, scores, _) = format_matchs(test_dataset)
    scores = [x[0] for x in scores]
    distances = []

    predictions = compute_predictions(MODEL, units)
    for i in range(len(predictions)):
        distances.append(abs(predictions[i] - scores[i]))
    print(sum(distances) / len(distances))
    # distances = compute_distance(predictions, scores)
    # print(sum(distances) / len(distances))
