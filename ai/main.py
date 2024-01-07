import os
from keras.models import load_model
from ai_model_building.army_builder import army_builder
from ai_model_building.utils import removekey
from neuronal_network.model_builder import format_json_match, format_matchs, purge_data

# The absolute path to the current file
ABSOLUTE_PATH = os.path.dirname(os.path.abspath(__file__))

# if not ABSOLUTE_PATH.endswith('ai'):
#     ABSOLUTE_PATH = os.path.join(ABSOLUTE_PATH, 'ai')

MODEL_PATH = os.path.join(
    ABSOLUTE_PATH, 'neuronal_network', 'trainning_data', 'model.h5'
)


def format_request(request):
    """Format the request to a format usable by our model

    Args:
        request (dict(
                "first_player": list(
                    dict("name": str, "modifiers": list(str))
                    ),
                "second_player": list(
                    dict("name": str, "modifiers": list(str))
                    )
            )): The http request's body

    Returns: (np.array): The request formatted
    """
    first_player_units = [unit['name'] for unit in request['first_player']]
    first_player_modifiers = [
        modifier for unit in request['first_player'] for modifier in unit['modifiers']
    ]
    second_player_units = [unit['name'] for unit in request['second_player']]
    second_player_modifiers = [
        modifier for unit in request['second_player'] for modifier in unit['modifiers']
    ]
    first_army = army_builder(
        {'units': first_player_units, 'modifiers': first_player_modifiers, 'score': 0}
    )
    second_army = army_builder(
        {'units': second_player_units, 'modifiers': second_player_modifiers, 'score': 0}
    )
    removekey(first_army, 'score')
    removekey(second_army, 'score')
    result = format_json_match(
        {'first_player': first_army, 'second_player': second_army, 'map': 0}
    )
    # result = purge_data([result])
    # if len(result) == 0:
        # raise Exception('Invalid match')
    (X, _, _) = format_matchs(result)
    return X


def predict(request):
    """Compute and return a prediction from the existing model

    Args:
        request (dict(
                "first_player": list(
                    dict("name": str, "modifiers": list(str))
                    ),
                "second_player": list(
                    dict("name": str, "modifiers": list(str))
                    )
            )): The input value

    Returns: (list(int)): The model's prediction)
    """
    model = load_model(MODEL_PATH)
    if model is None:
        print('Model not found')
        return
    x_value = format_request(request)
    result = model.predict(x_value)
    return list(result[0])
