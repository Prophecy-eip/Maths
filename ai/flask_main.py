from flask import Flask, request, jsonify
import numpy as np
import main, os
import random, json

app = Flask(__name__)

@app.route('/predict', methods=['POST'])
def predict():
    """Forward the requested format the the IA with a minimum of 20 units in of the army

    Args (Body):
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
    my_json = request.json
    res = main.predict(my_json)

    # Convert float32 values to float64
    def convert_float32(obj):
        if isinstance(obj, np.float32):
            return float(obj)
        if isinstance(obj, dict):
            return {key: convert_float32(value) for key, value in obj.items()}
        elif isinstance(obj, list):
            return [convert_float32(item) for item in obj]
        else:
            return obj

    res_converted = convert_float32(res)
    return jsonify(res_converted)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=4242, debug=True)
