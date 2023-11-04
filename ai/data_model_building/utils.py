# Utils and miscellaneous set of functions in python to help building the AI model

import dataclasses
import json

def concatenate_dictionaries(*dicts):
    """This function will concatenate any number of dictionary

    Args:
        *dicts (dict): Any number of dictionaries to concatenate

    Returns:
        dict: A dictionary containing all the key-value pairs of the input dictionaries
    """
    result = {}
    for d in dicts:
        result.update(d)
    return result


def safe_add_signed_unsigned(a: int, rhs: int) -> int:
    """This function will add two integers but making sure that the result is unsigned

    Args:
        a (int): The first integer
        rhs (int): The second integer

    Returns:
        int: The sum of the two integers
    """
    if rhs < 0:
        return a - abs(rhs) if a > abs(rhs) else 0
    else:
        return a + rhs if a + rhs >= 0 else 0


class EnhancedJSONEncoder(json.JSONEncoder):
    """This class is a JSON encoder that can encode dataclass objects

    Args:
        json (json.JSONEncoder): The JSON encoder to extend
    """
    def default(self, o):
        """Encode a dataclass object

        Args:
            o (class): The object to encode

        Returns:
            dict | Any: The encoded object
        """
        if dataclasses.is_dataclass(o):
            return dataclasses.asdict(o)
        return super().default(o)


def removekey(d, key):
    """This function will remove a key from a dictionary and return the result

    Args:
        d (dict): The dictionary to remove the key from
        key (String): The key to remove

    Raises:
        KeyError: In case the key is not found in the dictionary

    Returns:
        dict: The dictionary without the key
    """
    try:
        r = dict(d)
        del r[key]
    except KeyError:
        raise KeyError(f'Key {key} not found in dictionary')
    return r
