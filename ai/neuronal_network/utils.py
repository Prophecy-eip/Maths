def align_lists(list1, list2, default_value):
    """This function will align two list of the same size by adding a default value at the end of the smallest one

    Args:
        list1 (list): The first list to align
        list2 (list): The second list to align
        default_value (any): The default value to add at the end of the smallest list

    Returns:
        (list, list): The two list aligned
    """
    if len(list1) > len(list2):
        list2 += default_value * (len(list1) - len(list2))
    elif len(list1) < len(list2):
        list1 += default_value * (len(list2) - len(list1))
    return list1, list2
