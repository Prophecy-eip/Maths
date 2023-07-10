from unit_parser import UNIT_LIST
from utils import removekey


def unit_binder(unit_name):
    """Fetch the unit descriptor from the unit name

    Args:
        unit_name (str): The name of the unit

    Returns:
        dict({name: str, stat: Stat, cost: int}): The unit descriptor
    """
    for unit in UNIT_LIST:
        if unit['name'] == unit_name:
            return unit
    raise Exception('Unknown unit ' + unit_name)


def army_builder(player_resume):
    """Create a representation of an army from the player resume.
        This representation is used as model input

    Args:
        player_resume (dict({
            "score": int,
            "units": list(str),
            "modifiers": list(str)
            })) -> The player army resume

    Returns:
        dict({
            "score": int,
            "modifiers": list(int),
            "units": list(dict({name: str, stat: Stat}))
            "cost": int
            }) -> The army representation
    """
    modifiers = []
    cost = 0
    units = []
    score = 0
    score = player_resume['score']

    modifiers = [hash(modifier) for modifier in player_resume['modifier']]
    for unit in player_resume['units']:
        try:
            val = unit_binder(unit)
            cost += int(val['cost'])
            removekey(val, 'cost')
            units.append(val)
        except Exception as e:
            continue
    return {'score': score, 'modifiers': modifiers, 'units': units, 'cost': cost}


def match_builder(match_resume):
    """Create a representation of a match from the match resume.
     This representation is used as model input

    Args:
        match_resume (dict({
            "first_player": dict({
                "score": int,
                "units": list(str),
                "modifiers": list(str)
            }),
            "second_player": dict({
                "score": int,
                "units": list(str),
                "modifiers": list(str)
                }),
            "map": int
            })) -> The match resume

    Returns:
        dict({
            "first_player": dict({
                "score": int,
                "modifiers": list(int),
                "units": list(dict({name: str, stat: Stat})),
                "cost": int
                }),
            "second_player": dict({
                "score": int,
                "modifiers": list(int),
                "units": list(dict({name: str, stat: Stat})),
                "cost": int
                }),
            "map": int
            }) -> The match representation
    """
    first_player = army_builder(match_resume['first_player'])
    second_player = army_builder(match_resume['second_player'])
    map = match_resume['map']
    return {'first_player': first_player, 'second_player': second_player, 'map': map}


def build_trainning_data(matchs):
    """This function take as parameter a list of matchs and return it build with the dataclass created so it can be used as model input

    Args:
        matchs (list(dict({
            "first_player": dict({
                "score": int,
                "units": list(str),
                "modifiers": list(str)
            }),
            "second_player": dict({
                "score": int,
                "units": list(str),
                "modifiers": list(str)
                }),
            "map": int
            }))) -> The list of matchs to build

    Returns:
        list(dict({
            "first_player": dict({
                "score": int,
                "modifiers": list(int),
                "units": list(dict({name: str, stat: Stat})),
                "cost": int
                }),
            "second_player": dict({
                "score": int,
                "modifiers": list(int),
                "units": list(dict({name: str, stat: Stat})),
                "cost": int
                }),
            "map": int
            })) -> The list of matchs built
    """
    result = []
    for match in matchs:
        result.append(match_builder(match))
    return result


if __name__ == '__main__':
    import json
    import sys
    import os
    from utils import EnhancedJSONEncoder

    try:
        print(UNIT_LIST[0])
        print('Tried')
        trainning_folder = 'trainning_data'
        dataset_path = sys.argv[1]
        dataset = json.load(open(dataset_path))
        trainning_data = build_trainning_data(dataset)
        os.mkdir(trainning_folder)
        with open(trainning_folder + '/trainning_data.json', 'w') as f:
            json.dump(trainning_data, f, cls=EnhancedJSONEncoder)
        print('Trainning data created')
    except Exception as e:
        print('Error while creating trainning data')
        print(e)
        exit(84)
