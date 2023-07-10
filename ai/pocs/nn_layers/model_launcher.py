import numpy as np
import tensorflow as tf
from tensorflow import keras
import json
import sys

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
        match['second_player']['score']
    )
    return sample



def format_value(value):
    """build the model's trainning data from the dataset

    Args:
        value (
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
    scores = []
    # This file is an old poc, so the army length is hardcoded. I decided to keep those files are they are, since they are not used anymore
    final_len = 22
    units = [np.array(value[0]), np.array(value[1])]
    scores = np.array([value[2], value[3]])
    units[0] = np.pad(units[0], ((0, final_len - len(units[0])), (0, 0)), 'constant')
    units[1] = np.pad(units[1], ((0, final_len - len(units[1])), (0, 0)), 'constant')
    return (np.array([units]), np.array(scores))



def main():
    if len(sys.argv) != 3:
        print('Usage: python3 model_launcher.py <model_path> <match_path>')
        sys.exit(1)
    model_path = sys.argv[1]
    match_path = sys.argv[2]
    model = keras.models.load_model(model_path)
    match = json.load(open(match_path))
    values = match_to_data(match)
    x_test, y_test = format_value(values)

    if model is None:
        print('Model not found')
        sys.exit(1)
    print('Predicted result:')
    print(model.predict(x_test))
    print(f'Actual result: {y_test}')

if __name__ == '__main__':
    main()
