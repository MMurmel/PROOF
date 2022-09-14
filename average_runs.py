#!/usr/bin/env python3

import sys
import os

import matplotlib.pyplot as plt

base_dir = sys.argv[1]

file_1 = (os.path.join(base_dir, "run-1/metrics/metrics.csv"))
file_2 = (os.path.join(base_dir, "run-2/metrics/metrics.csv"))
file_3 = (os.path.join(base_dir, "run-3/metrics/metrics.csv"))

file_a = (os.path.join(base_dir, "average_metrics.csv"))

with open(file_1, "r") as f1, open(file_2, "r") as f2, open(file_3, "r") as f3, open(file_a, "w") as fa:
    files = [f1, f2, f3]

    file_to_list_of_lines = lambda file: file.readlines()[2:-1]
    files = list(map(file_to_list_of_lines, files))
    line_to_tuple = lambda line: tuple(line.rstrip().split(','))
    list_of_lines_to_list_of_tuples = lambda lol: list(map(line_to_tuple, lol))

    almost_there = list(zip(*list(map(list_of_lines_to_list_of_tuples, files))))

    parse_single_tuple = lambda x: tuple(map(float, x))
    parse_tuple_of_tuples = lambda x: tuple(map(parse_single_tuple, x))

    almost_there_numbers = list(map(parse_tuple_of_tuples, almost_there))

    my_sum = lambda x: (x[0][0] + x[1][0] + x[2][0], x[0][1] + x[1][1] + x[2][1],x[0][2] + x[1][2] + x[2][2])
    average_sum = lambda x: tuple(map(lambda y: y/3, my_sum(x)))
    averaged_csv_values = list(map(average_sum, almost_there_numbers))

    columns = list(zip(*averaged_csv_values))
    iterations = columns[0]
    times = columns[1]
    values = columns[2]

    # Single plots
    plt.plot(iterations, times)
    plt.show()
    plt.plot(iterations, values)
    plt.show()

    # Together
    fig, ax1 = plt.subplots()
    ax1.plot(iterations, times)
    ax2 = ax1.twinx()
    ax2.plot(iterations, values, color="red")
    fig.tight_layout()
    plt.show()

