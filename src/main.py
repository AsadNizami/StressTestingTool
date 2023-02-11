import StressTesting
import generator as gen
import solution as sol
import bruteforce as bru
from file_path import GENERATOR_PATH, SOLUTION_OUTPUT, BRUTEFORCE_OUTPUT


"""
1) Use generator to generate the input
2) Write a brute force solution to solve a problem
3) Write a fast and optimal solution to compare the output with that of a brute force solution
"""

"""
    Example question:
    Input format: An integer
    Output format: Print True if the number if prime else False
"""

# Generating input
gen.run()
bad_output = list()
good_output = list()

# your solution
with open(GENERATOR_PATH, 'r') as f:
    for line in f.readlines():
        bad_output.append(sol.run(int(line)))

with open(SOLUTION_OUTPUT, 'w') as f:
    f.writelines('\n'.join(map(str, bad_output)))

with open(GENERATOR_PATH, 'r') as f:
    for line in f.readlines():
        good_output.append(bru.run(int(line)))


with open(BRUTEFORCE_OUTPUT, 'w') as f:
    f.writelines('\n'.join(map(str, good_output)))

ans = StressTesting.checker(BRUTEFORCE_OUTPUT, SOLUTION_OUTPUT)
print(ans)