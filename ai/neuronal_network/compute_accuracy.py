import json
import numpy as np
from keras.models import load_model
import random
import os
from nn_builder import format_json_match, purge_data, format_matches

# The absolute path to the current file
ABSOLUTE_PATH = os.path.dirname(os.path.abspath(__file__))

if not ABSOLUTE_PATH.endswith('neuronal_network'):
    ABSOLUTE_PATH = os.path.join(ABSOLUTE_PATH, 'ai', 'neuronal_network')

# The data loaded from the json file
DATA = json.load(
    open(os.path.join(ABSOLUTE_PATH, 'trainning_data', 'trainning_data.json'), 'r')
)

# The model to use
MODEL = load_model(os.path.join(ABSOLUTE_PATH, 'trainning_data', 'model.h5'))

# The number of matchs to use for the test
TEST_SIZE = 100


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
        points.append(prediction[0])
    return points


def compute_distance(points, scores):
    """Build a list of distances between the predictions and the scores

    Args:
        points (list(list(int))): The list of predictions
        scores (list(list(int))): The list of scores

    Returns:
        list(int): The list of distances between the predictions and the actual scores
    """
    distances = []
    for i in range(len(points)):
        distances.append(abs(points[i][0] - scores[i][0]))
    return distances


if __name__ == '__main__':
    test_dataset = build_test_dataset()
    # last parameter here is the size of the largest army. Useless here
    (units, scores, _, _) = format_matches(test_dataset)

    predictions = compute_predictions(MODEL, units)
    distances = compute_distance(predictions, scores)
    print(sum(distances) / len(distances))
