import json
import random
import os

# The absolute path to the current file
ABSOLUTE_PATH = os.path.dirname(os.path.abspath(__file__))

if not ABSOLUTE_PATH.endswith('nn_layers'):
    ABSOLUTE_PATH = os.path.join(ABSOLUTE_PATH, 'ai', 'pocs', 'nn_layers')

if __name__ == '__main__':
    data = json.load(
        open(os.path.join(ABSOLUTE_PATH, 'trainning_data', 'trainning_data.json')))
    index = random.randint(0, len(data) - 1)
    match = data[index]
    dump = json.dumps(match, indent=4, sort_keys=True)
    with open(os.path.join(ABSOLUTE_PATH, 'match_sample.json'), 'w') as f:
        f.write(dump)
