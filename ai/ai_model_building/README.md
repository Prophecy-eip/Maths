# Prophecy - AI model building

This folder contains the code needed to create artificial intelligence models. These models are intended to be trained on specific data whose schema will be documentated below.

## Input data

This code was designed to be based on data formatted by the Prophecy team.
The data to transform will be retrieved from our [private github repository](https://github.com/Prophecy-eip/T9A-Records)
here is the schema of the data to be transform

```json
data: {
    "first_player": {
        "score": "int",
        "units": ["str"],
        "modifiers": ["str"]
    },
    "second_player": {
        "score": "int",
        "units": ["str"],
        "modifiers": ["str"]
    },
    "map": "int"
}
```

## Output data

The data transformed by the code will have the following scheme :

```json
data: {
    "first_player": {
        "score": "int",
        "units": [{
          "name": "str",
          "stat": {
            "advance": "int",
            "march": "int",
            "discipline": "int",
            "health_points": "int",
            "defense": "int",
            "resilience": "int",
            "armour": "int",
            "attack": "int",
            "offensive": "int",
            "strength": "int",
            "armour_penetration": "int",
            "agility": "int",
            "aegis": "int",
            "ballistic_skill": "int",
            "shots": "int"
          }
        }],
        "modifiers": ["int"]
    },
    "second_player": {
        "score": "int",
        "units": [{
          "name": "str",
          "stat": {
            "advance": "int",
            "march": "int",
            "discipline": "int",
            "health_points": "int",
            "defense": "int",
            "resilience": "int",
            "armour": "int",
            "attack": "int",
            "offensive": "int",
            "strength": "int",
            "armour_penetration": "int",
            "agility": "int",
            "aegis": "int",
            "ballistic_skill": "int",
            "shots": "int"
          }
        }],
        "modifiers": ["int"]
    },
    "map": "int"
}
```

We add to the unit's name their statistics.

The modifiers are replaced by python's integer because they go through a hash function. The point is to make a tokenization so that the AI can easily use those values.

## How to use

This folder come with a bash script: "generate_models.sh".
This script will download The prophecy repository that contains all of our data and create the models.
It will be available in a file called "trainning_data.json" in the folder "trainning_data"
To launch the process:

```bash
${AI_MODEL_PATH}/generate_models.sh
cd trainning_data
```

with AI_MODEL_PATH the path to the folder where belongs this README
