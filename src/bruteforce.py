import sys
sys.stdin = open('data/input.txt', 'r')


def run(n):
    for i in range(2, n):
        if n % i == 0:
            return False

    return True

