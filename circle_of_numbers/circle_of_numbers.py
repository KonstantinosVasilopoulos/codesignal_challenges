def solution(n, firstNumber):
    # Move a distance equal to half the size of the circle
    current = firstNumber
    for i in range(0, n // 2):
        current = next_node(n, current)

    return current


def next_node(n, current):
    """ Get the next node in a circle of n nodes. """
    if current == n - 1:
        return 0  # The first node of the circle
    
    return current + 1


if __name__ == '__main__':
    x = solution(10, 2)
    assert x == 7, f'solution = {x} must equal 7.'
    x = solution(10, 5)
    assert x == 0, f'solution = {x} must equal 0.'
