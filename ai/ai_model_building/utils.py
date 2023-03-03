#Utils and miscellaneous set of functions in python to help building the AI model

#This function will concatenate any number of dictionary
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
