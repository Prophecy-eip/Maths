import re

stats_config = {
    'global': ['Ad', 'Ma', 'Di'],
    'defense': ['HP', 'Df', 'Re', 'Arm'],
    'offense': ['At', 'Of', 'St', 'AP', 'Ag'],
}


def handle_special_values(value):
    """This function handles the special values of the stats

    Args:
        value (str): The value to convert

    Returns:
        str: The converted value
    """

    def replace_dice_six(match):
        return str(int(match.group(1)) * 3) if match.group(1) else '3'

    def replace_dice_three(match):
        return str(int(match.group(1)) * 2) if match.group(1) else '2'

    value = value.replace('C', '0')
    value = re.sub('(\d)*D6', replace_dice_six, value)
    value = re.sub('(\d)*D3', replace_dice_three, value)
    return value


def numerise_field(value):
    """This function handles the non numeric values of the stats

    Args:
        value (str): The value to convert

    Returns:
        int: The converted value
    """
    if value.isnumeric():
        return int(value)
    if len(value) == 0:
        return 0

    value = handle_special_values(value)
    return int(eval(value))


def custom_load_unit_stat(stat, unit):
    """Fetch the substats of a stat from a unit descriptor

    Args:
        stat (str): The stat's name
        unit (dict): The unit descriptor

    Returns:
        dict: A dict with the substats values
    """
    result = {}

    try:
        for sub_stat in unit[stat]:
            result[sub_stat] = [
                numerise_field(unit[stat][sub_stat]) if sub_stat in unit[stat] else 0
            ]

        return result
    except KeyError:
        return {}


def load_unit_stat(stat, unit):
    """Fetch the substats of a stat from a unit descriptor

    Args:
        stat (str): The stat's name
        unit (dict): The unit descriptor

    Returns:
        list(int): A list with the substats values
    """
    result = []
    if stat in unit:
        for sub_stat in stats_config[stat]:
            result += [
                numerise_field(unit[stat][sub_stat]) if sub_stat in unit[stat] else 0
            ]
    else:
        for sub_stat in stats_config[stat]:
            result += [0]
    return result


def create_stat(unit):
    """Based on the unit descriptor, create a Stats object

    Args:
        unit (dict): The unit fetched from the json files

    Returns:
        Stat: A dataclass Stat that contains all the stats for the unit
    """
    fields = []
    for val in stats_config:
        fields.extend(stats_config[val])
    result = []
    for stat in stats_config:
        result += load_unit_stat(stat, unit)
    result = dict(list(zip(fields, result)))
    return result
