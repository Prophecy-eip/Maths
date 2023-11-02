# This function will align two list of the same size by adding a default value at the end of the smallest one
def align_lists(list1, list2, default_value):
    if len(list1) > len(list2):
        list2 += default_value * (len(list1) - len(list2))
    elif len(list1) < len(list2):
        list1 += default_value * (len(list2) - len(list1))
    return list1, list2
