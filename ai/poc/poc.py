import numpy as np
import tensorflow as tf
from tensorflow import keras
import json
from model_builder import match_to_data

model = keras.models.load_model('./train_results/model.h5')
match = json.load(open('./match_sample.json'))


values = match_to_data(match)


def format_value(value):
    scores = []
    final_len = 22
    units = [np.array(value[0]), np.array(value[1])]
    scores = np.array([value[2], value[3]])
    units[0] = np.pad(units[0], ((0, final_len - len(units[0])), (0, 0)), 'constant')
    units[1] = np.pad(units[1], ((0, final_len - len(units[1])), (0, 0)), 'constant')
    return (np.array([units]), np.array(scores))

x_test, y_test = format_value(values)

print(model.predict(x_test))
