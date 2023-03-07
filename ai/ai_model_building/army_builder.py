from unit_parser import UNIT_LIST


def unit_binder(unit_name):
    """Fetch the unit descriptor from the unit name

    Args:
        unit_name (str): The name of the unit

    Returns:
        dict({name: str, stat: Stat}): The unit descriptor
    """
    for unit in UNIT_LIST:
        if unit['name'] == unit_name:
            return unit
    return None


def army_builder(player_resume):
    """Create a representation of an army from the player resume.
        This representation is used as model input

    Args:
        player_resume (dict): The player army resume

    Returns:
        dict: The army representation
    """
    modifiers = []
    units = []
    score = 0
    score = player_resume['score']
    modifiers = [hash(modifier) for modifier in player_resume['modifier']]
    units = [unit_binder(unit) for unit in player_resume['units']]
    return {'score': score, 'modifiers': modifiers, 'units': units}


def match_builder(match_resume):
    """Create a representation of a match from the match resume.
     This representation is used as model input

    Args:
        match_resume (Object): The match resume

    Returns:
        dict: The match representation
    """
    first_player = army_builder(match_resume['first_player'])
    second_player = army_builder(match_resume['second_player'])
    map = match_resume['map']
    return {'first_player': first_player, 'second_player': second_player, 'map': map}


def build_trainning_data(matchs):
    """This function take as parameter a list of matchs and return it build with the dataclass created so it can be used as model input

    Args:
        matchs (list): The list of matchs to build

    Returns:
        list: The list of matchs built
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
