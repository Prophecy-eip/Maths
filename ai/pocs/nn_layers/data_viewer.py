import json
import random

data = json.load(open('./trainning_data/trainning_data.json'))

index = random.randint(0, len(data) - 1)

match = data[index]

dump = json.dumps(match, indent=4, sort_keys=True)

with open('match_sample.json', 'w') as f:
    f.write(dump)
