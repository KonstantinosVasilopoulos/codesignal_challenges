def solution(crypt, solution):
    # Iterate over the words of the cryptarithm and calculate each word's arithmetic
    arithemtics = []  # List containing the arithmetics in integer format
    for word in crypt:
        arithmetic = ''
        for char in word:
            arithmetic += get_mapping(char, solution)

        # Ensure that the arithmetic has no leading zeroes
        if has_leading_zeroes(arithmetic):
            return False

        # Append the integer format of the arithmetic to the list
        arithemtics.append(int(arithmetic))

    # Test whether the equation is valid
    return arithemtics[0] + arithemtics[1] == arithemtics[2]


def get_mapping(char: str, solution: list) -> str:
    """ Find and return the solution mapping for a provided character. """
    for mapping in solution:
        if char == mapping[0]:
            return mapping[1]

    return '?'  # No mapping found

def has_leading_zeroes(number: str) -> bool:
    """ Check if the given string has any leading zeroes. """
    return number.startswith('0') and len(number) > 1


if __name__ == '__main__':
    # Test the solution
    crypt = ["SEND", "MORE", "MONEY"]
    sol = [
        ['O', '0'],
        ['M', '1'],
        ['Y', '2'],
        ['E', '5'],
        ['N', '6'],
        ['D', '7'],
        ['R', '8'],
        ['S', '9'],
    ]
    assert solution(crypt, sol)

    crypt = ["TEN", "TWO", "ONE"]
    sol = [
        ['O', '1'],
        ['T', '0'],
        ['W', '9'],
        ['E', '5'],
        ['N', '4'],
    ]
    assert not solution(crypt, sol)
