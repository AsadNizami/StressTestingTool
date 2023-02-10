from random import randint


GENERATOR_PATH = "data/input.txt"

def run():
    test_cases = list()

    for _ in range(10):
        cases = randint(1, 10**9) % 10**5
        test_cases.append(cases)

    with open(GENERATOR_PATH, 'w') as f:
        f.writelines('\n'.join(map(str, test_cases)))


if __name__ == '__main__':
    run()
