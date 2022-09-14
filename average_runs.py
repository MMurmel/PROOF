#!/usr/bin/env python3

import sys
import os

import matplotlib.pyplot as plt

# How many runs should be averaged over.
runcount = 5

# The base folder of the run.
base_dir = sys.argv[1]
files = []
# Add all the run CSVs.
for i in range(1,runcount+1):
    files.append(open(os.path.join(base_dir, f"run-{i}/metrics/metrics.csv"), "r"))

# Where the averaged metrics will be outputted.
file_a = (os.path.join(base_dir, "average_metrics.csv"))

# Read all lines ignoring the first two (name and iteration zero) and the last one (uneven iteration)
file_to_list_of_lines = lambda file: file.readlines()[2:-1]
lines = list(map(file_to_list_of_lines, files))
line_to_tuple = lambda line: tuple(line.rstrip().split(','))
list_of_lines_to_list_of_tuples = lambda lol: list(map(line_to_tuple, lol))
pre_parsed_lines = list(zip(*list(map(list_of_lines_to_list_of_tuples, lines))))

# Parse everything to floats
parse_single_tuple = lambda x: tuple(map(float, x))
parse_tuple_of_tuples = lambda x: tuple(map(parse_single_tuple, x))
parsed_lines = list(map(parse_tuple_of_tuples, pre_parsed_lines))

# Average over them elementwise
average_sum = lambda x: tuple(map(lambda value: value/runcount, map(sum, zip(*x))))
averaged_csv_values = list(map(average_sum, parsed_lines))

# Dewrangle everything into separet lists.
columns = list(zip(*averaged_csv_values))
iterations = columns[0]
times = columns[1]
values = columns[2]

# TODO write back averaged data
#with open(file_a, "w") as fa:

### Single plots
font = {'family': 'DejaVu Sans',
        'weight': 'medium',
        'size': 15,
        }
# Time per iteration
plt.plot(iterations, times)
plt.title("Run time", fontdict=font)
plt.xlabel("Iteration", fontdict=font)
plt.ylabel("time (s)", fontdict=font)
plt.subplots_adjust(left=0.15)
plt.show()

# Regularizer value per iteration
plt.plot(iterations, values, color="red")
plt.title("Decrease in Regularization Value", fontdict=font)
plt.xlabel("Iteration", fontdict=font)
plt.ylabel("R", fontdict=font)
plt.subplots_adjust(left=0.15)
plt.show()

# Together
fig, ax1 = plt.subplots()
ax1.plot(iterations, times)
ax2 = ax1.twinx()
ax2.plot(iterations, values, color="red")
fig.tight_layout()
plt.show()
