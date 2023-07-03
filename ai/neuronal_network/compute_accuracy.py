import json
import numpy as np
from keras.models import load_model
import random
from ai.neuronal_network.model_builder import format_json_match, purge_data, format_matchs

data = json.load(open('./trainning_data/trainning_data.json', 'r'))

model = load_model('./trainning_data/model.h5')

test_size = 1000


def compute_points_distance(p1, p2):
    return np.sqrt((p1[0] - p2[0])**2 + (p1[1] - p2[1])**2)


def build_test_dataset():
    test_dataset = []
    for i in range(test_size):
        match = format_json_match(random.choice(data))
        if match is not None:
            test_dataset.append(match)
    test_dataset = purge_data(test_dataset)
    return test_dataset


def compute_predictions(model, units):
    points = []
    for x in units:
        prediction = model.predict(np.array([x]))
        points.append(prediction[0])
    return points


def compute_distance(points, scores):
    distances = []
    for i in range(len(points)):
        distances.append(compute_points_distance(points[i], scores[i]))
    return distances


test_dataset = build_test_dataset()
(units, scores, _) = format_matchs(test_dataset)

predictions = compute_predictions(model, units)
distances = compute_distance(predictions, scores)
print(sum(distances) / len(distances))
