from random import randint
from file_path import GENERATOR_PATH

def run():
    test_cases = list()

    for _ in range(10):
        cases = randint(1, 10**9) % 10**5
        test_cases.append(cases)

    with open(GENERATOR_PATH, 'w') as f:
        f.writelines('\n'.join(map(str, test_cases)))


if __name__ == '__main__':
    run()
