from unit_parser import UNIT_LIST


def unit_binder(unit_name):
    for unit in UNIT_LIST:
        if unit['name'] == unit_name:
            return unit
    return None


def army_builder(player_resume):
    modifiers = []
    units = []
    score = 0
    score = player_resume['score']
    modifiers = [hash(modifier) for modifier in player_resume['modifier']]
    units = [unit_binder(unit) for unit in player_resume['units']]
    return {'score': score, 'modifiers': modifiers, 'units': units}


def match_builder(match_resume):
    first_player = army_builder(match_resume['first_player'])
    second_player = army_builder(match_resume['second_player'])
    map = match_resume['map']
    return {'first_player': first_player, 'second_player': second_player, 'map': map}


def build_trainning_data(matchs):
    result = []
    for match in matchs:
        result.append(match_builder(match))
    return result
