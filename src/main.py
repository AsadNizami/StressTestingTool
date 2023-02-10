import StressTesting
from generator import run, GENERATOR_PATH


"""
    Example question:
    Input format: An integer
    Output format: Print True if the number if prime else False
"""


SOLUTION_PATH = "data/bad_sol.txt"
run()
response = StressTesting.checker(GENERATOR_PATH, "data/bad_sol.txt")
print(response)
