import json
import os
from stat_parser import create_stat, stats_config, custom_load_unit_stat
import pathlib
import copy

UNITS_FOLDER = str(pathlib.Path(__file__).parent.resolve()
                   ) + '/units_descriptor'

UNITS_FILES = [file for file in os.listdir(
    UNITS_FOLDER) if file.endswith('.json')]


def unit_builder(file_unit: dict):
    sub_unit_fields = {}
    results = []
    reffined = {}
    branching_list = []
    stats = create_stat(
        file_unit)
    results.append({'name': file_unit['name'], 'stat': stats,
                   'cost': file_unit['cost'] if 'cost' in file_unit else 0})
    unit_fields = file_unit.keys()
    for field in stats_config:
        sub_unit_fields[field] = list(filter(lambda x: x.startswith(
            field + 'name') and x != field + 'name', unit_fields))
    for field in sub_unit_fields:
        for sub_unit in sub_unit_fields[field]:
            branching = sub_unit.split(field + 'name')[1]
            if branching not in branching_list:
                branching_list.append(branching)

    def func(x): return x if x + 'name' + \
        branch in sub_unit_fields[x] else None
    for branch in branching_list:
        reffined[branch] = list(filter(func, sub_unit_fields))

    for branch in reffined:
        unit = copy.deepcopy(stats)
        for field in reffined[branch]:
            values = custom_load_unit_stat(field + branch, file_unit)

            for i in values:
                unit[i] = values[i]
        results.append({'name': branch, 'stat': unit,
                       'cost': file_unit['cost'] if 'cost' in file_unit else 0})
    return results


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
                result += unit_builder(file_content[unit])
                # result.append(
                #     {'name': file_content[unit]['name'], 'stat': create_stat(file_content[unit]), 'cost': file_content[unit]['cost'] if 'cost' in file_content[unit] else 0})
    return result


UNIT_LIST = fetch_game_units()
