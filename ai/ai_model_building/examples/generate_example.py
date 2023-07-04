import json
import sys

sys.path.append('../')

from army_builder import build_trainning_data
from utils import EnhancedJSONEncoder

if __name__ == '__main__':
    match_data = json.load(open('match.example.json'))

    input_data = build_trainning_data([match_data])

    with open('input.example.json', 'w') as f:
        json.dump(input_data, f, cls=EnhancedJSONEncoder)
