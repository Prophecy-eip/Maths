import json
import os
from stat_parser import create_stat
import pathlib

UNITS_FOLDER = str(pathlib.Path(__file__).parent.resolve()
                   ) + '/units_descriptor'

UNITS_FILES = [file for file in os.listdir(
    UNITS_FOLDER) if file.endswith('.json')]


def fetch_game_units():
    """Fetch the units from the json files present in the units_descriptor folder

    Returns:
        list(dict({
            "name": str,
            "stat": Stat
            })) -> A list of units with their name and their stats
    """
    result = []

    for file in UNITS_FILES:
        with open(os.path.join(UNITS_FOLDER, file)) as f:
            file_content = json.load(f)
            for unit in file_content:
                result.append(
                    {'name': file_content[unit]['name'], 'stat': create_stat(file_content[unit])})
    return result


UNIT_LIST = fetch_game_units()
