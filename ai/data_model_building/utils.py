# Utils and miscellaneous set of functions in python to help building the AI model

import dataclasses
import json

# This function will concatenate any number of dictionary
def concatenate_dictionaries(*dicts):
    result = {}
    for d in dicts:
        result.update(d)
    return result


# This function return the sum of two int but making sure that the result is unsigned
def safe_add_signed_unsigned(a: int, rhs: int) -> int:
    if rhs < 0:
        return a - abs(rhs) if a > abs(rhs) else 0
    else:
        return a + rhs if a + rhs >= 0 else 0


# Serve as a dataclass encoder for json
class EnhancedJSONEncoder(json.JSONEncoder):
    def default(self, o):
        if dataclasses.is_dataclass(o):
            return dataclasses.asdict(o)
        return super().default(o)


# This function will remove a key from a dictionary and return the result


def removekey(d, key):
    try:
        r = dict(d)
        del r[key]
    except KeyError:
        raise KeyError(f'Key {key} not found in dictionary')
    return r
