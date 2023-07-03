import json
import os
from stat_parser import create_stat, stats_config, custom_load_unit_stat
import pathlib
import copy

UNITS_FOLDER = str(pathlib.Path(__file__).parent.resolve()
                   ) + '/units_descriptor'

UNITS_FILES = [file for file in os.listdir(
    UNITS_FOLDER) if file.endswith('.json')]

def build_subunits(file_unit, subunits):
    """Generate the subunits of a unit

    Args:
        file_unit (dict): The base unit descriptor
        subunits (dict): A dict with for each subunit, the list of its fields

    Returns:
        list(dict({
            "name": str,
            "stat": Stat,
            "cost": int
            })) -> A list of units with their name and their stats
    """
    stats = create_stat(file_unit)
    results = []

    for branch in subunits:
        unit = copy.deepcopy(stats)
        for field in subunits[branch]:
            values = custom_load_unit_stat(field + branch, file_unit)
            for i in values:
                unit[i] = values[i]
        results.append({'name': branch, 'stat': unit,
                       'cost': file_unit['cost'] if 'cost' in file_unit else 0})
    return results


def unit_builder(file_unit: dict):
    """Create a representation of a unit or group of units from the unit descriptor.

    Args:
        file_unit (dict): The unit descriptor

    Returns:
        list(dict({
            "name": str,
            "stat": Stat,
            "cost": int
            })) -> A list of units with their name and their stats
    """
    sub_unit_fields = {}
    result = []
    subunits = {}
    branching_list = []
    stats = create_stat(file_unit)
    does_unit_exist = lambda x: x if x + 'name' +  branch in sub_unit_fields[x] else None

    # Save base unit
    result.append({'name': file_unit['name'], 'stat': stats,
                   'cost': file_unit['cost'] if 'cost' in file_unit else 0})

    # Find all subunits's fields
    for field in stats_config:
        sub_unit_fields[field] = list(filter(lambda x: x.startswith(
            field + 'name') and x != field + 'name', list(file_unit.keys())))

    # Find all subunits's names
    for field in sub_unit_fields:
        for sub_unit in sub_unit_fields[field]:
            branching_list.append(sub_unit.split(field + 'name')[1])
    branching_list = list(set(branching_list))

    # Save for all subunits the modified stats
    for branch in branching_list:
        subunits[branch] = list(filter(does_unit_exist, sub_unit_fields))

    # Create the subunits
    return result + build_subunits(file_unit, subunits)


def fetch_game_units():
    """Fetch the units from the json files present in the units_descriptor folder

    Returns:
        list(dict({
            "name": str,
            "stat": Stat,
            "cost": int
            })) -> A list of units with their name and their stats
    """
    result = []

    for file in UNITS_FILES:
        with open(os.path.join(UNITS_FOLDER, file)) as f:
            file_content = json.load(f)
            for unit in file_content:
                r = unit_builder(file_content[unit])
                for u in r:
                    print('Added unit: ' + u['name'])
                    print('cost: ' + str(u['cost']))
                result += r
                # result.append(
                #     {'name': file_content[unit]['name'], 'stat': create_stat(file_content[unit]), 'cost': file_content[unit]['cost'] if 'cost' in file_content[unit] else 0})
    return result


UNIT_LIST = fetch_game_units()
