import re
from model import Stats

stats_config = {
    'global': ['Ad', 'Ma', 'Di'],
    'defense': ['HP', 'Df', 'Re', 'Arm'],
    'offense': ['At', 'Of', 'St', 'AP', 'Ag']
}

stat_model_param = ['advance', 'march', 'discipline', 'health_points', 'defense',
                    'resilience', 'armour', 'attack', 'offensive', 'strength', 'armour_penetration', 'agility']


def handle_special_values(value):
    def replace_dice_six(match): return str(
        int(match.group(1)) * 3) if match.group(1) else '3'
    def replace_dice_three(match): return str(
        int(match.group(1)) * 2) if match.group(1) else '2'
    value = value.replace('C', '0')
    value = re.sub('(\d)*D6', replace_dice_six, value)
    value = re.sub('(\d)*D3', replace_dice_three, value)
    return value


def numerise_field(value):
    if value.isnumeric():
        return int(value)
    if (len(value) == 0):
        return 0

    value = handle_special_values(value)
    return int(eval(value))


def load_unit_stat(stat, unit):
    result = []
    if stat in unit:
        for sub_stat in stats_config[stat]:
            result += [numerise_field(
                unit[stat][sub_stat]) if sub_stat in unit[stat] else 0]
    else:
        for sub_stat in stats_config[stat]:
            result += [0]
    return result


def create_stat(unit):
    result = []
    for stat in stats_config:
        result += load_unit_stat(stat, unit)
    result = dict(list(zip(stat_model_param, result)))
    return Stats(**result)
