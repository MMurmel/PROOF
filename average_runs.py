#!/usr/bin/env python3

import sys
import os

import matplotlib.pyplot as plt

# How many runs should be averaged over.
runcount = 10
missing = 0

# The base folder of the run.
base_dir = sys.argv[1]

# An optional headline for the plots of a run
headline = ""
with open(os.path.join(base_dir, "headline",), "r") as headline_file:
    headline = str(headline_file.read()).rstrip()


files = []
# Add all the run CSVs.
for i in range(1,runcount+1):
    try:
        files.append(open(os.path.join(base_dir, f"run-{i}/metrics/metrics.csv"), "r"))
    except:
        print(f"Could not find metrics file for run {i}")
        missing += 1

runcount -= missing
print(f"Found metrics of {runcount} runs. Starting averaging.")

# Where the averaged metrics will be outputted.
file_a = (os.path.join(base_dir, "averaging_results"))

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

# Accumulate times:
acc_t = 0
times_acc = []
for i in range(len(times)):
    acc_t += times[i]
    times_acc.append(acc_t)

with open(file_a, "w") as fa:
    fa.write(f"Average for config: {headline}\n")
    fa.write(f"∅ Total iterations: {iterations[-1]}\n")
    fa.write(f"∅ Total time: {times_acc[-1]}s\n")
    fa.write(f"∅ time per iteration: {times_acc[-1]/iterations[-1]}s\n")
    fa.write(f"∅ ending R-value: {values[-1]}\n")
    fa.write(f"∅ total r-value decrease: {values[0]-values[-1]}\n")
    fa.write(f"∅ r-decrease per iteration: {(values[0]-values[-1])/iterations[-1]}\n")
    fa.write(f"∅ r-decrease per second: {(values[0]-values[-1])/times_acc[-1]}\n")

### Single plots
font = {'family': 'DejaVu Sans',
        'weight': 'medium',
        'size': 15,
        }
time_color = "#bccf02"
value_color= "#03305d"

# Time per iteration relative
plt.title(f"{headline}", fontdict=font)
plt.xlabel("Iteration", fontdict=font)

plt.plot(iterations, times, color=time_color, label="iteration time / s")

plt.legend()
plt.tight_layout()
plt.savefig(os.path.join(base_dir, "time_rel.jpg"), dpi=200)
plt.clf()

# Time per iteration absolute
plt.title(f"{headline}", fontdict=font)
plt.xlabel("Iteration", fontdict=font)

plt.plot(iterations, times, color=time_color, label="iteration time / s")

plt.ylim(0,25)
plt.xlim(-100,10100)
plt.legend()
plt.tight_layout()
plt.savefig(os.path.join(base_dir, "time_abs.jpg"), dpi=200)
plt.clf()

# Time accumulated relative
plt.title(f"{headline}", fontdict=font)
plt.xlabel("Iteration", fontdict=font)

plt.plot(iterations, times_acc, color=time_color, label="total time / s")

plt.legend()
plt.tight_layout()
plt.savefig(os.path.join(base_dir, "time_acc_rel.jpg"), dpi=200)
plt.clf()

# Time accumulated absolute
plt.title(f"{headline}", fontdict=font)
plt.xlabel("Iteration", fontdict=font)

plt.plot(iterations, times_acc, color=time_color, label="total time / s")

plt.ylim(0,1000)
plt.xlim(-100,10100)
plt.legend()
plt.tight_layout()
plt.savefig(os.path.join(base_dir, "time_acc_abs.jpg"), dpi=200)
plt.clf()

# Regularizer value relative
plt.title(f"{headline}", fontdict=font)
plt.xlabel("Iteration", fontdict=font)

plt.plot(iterations, values, color=value_color, label="regularization value")

plt.legend()
plt.tight_layout()
plt.savefig(os.path.join(base_dir, "value_rel.jpg"), dpi=200)
plt.clf()

# Regularizer value absolute
plt.title(f"{headline}", fontdict=font)
plt.xlabel("Iteration", fontdict=font)

plt.plot(iterations, values, color=value_color, label="regularization value")

plt.ylim(0,25500)
plt.xlim(-100,10100)
plt.legend()
plt.tight_layout()
plt.savefig(os.path.join(base_dir, "value_abs.jpg"), dpi=200)
plt.clf()

# Together relative
plt.xlabel("Iteration", fontdict=font)

fig, ax1 = plt.subplots()
ax1.set_title(f"{headline}", fontdict=font)
ax2 = ax1.twinx()

p1, = ax1.plot(iterations, times, color=time_color, label="iteration time / s")
p2, = ax2.plot(iterations, values, color=value_color, label="regularization value")
ax1.legend(handles=[p1,p2])
ax1.tick_params(axis='y', colors=time_color)
ax2.tick_params(axis='y', colors=value_color)

fig.tight_layout()
fig.savefig(os.path.join(base_dir, "both_rel.jpg"), dpi=200)
plt.clf()

# Together absolute
plt.xlabel("Iteration", fontdict=font)

fig, ax1 = plt.subplots()
ax1.set_title(f"{headline}", fontdict=font)
ax2 = ax1.twinx()

p1, = ax1.plot(iterations, times, color=time_color, label="iteration time / s")
p2, = ax2.plot(iterations, values, color=value_color, label="regularization value")
ax1.legend(handles=[p1,p2])
ax1.tick_params(axis='y', colors=time_color)
ax2.tick_params(axis='y', colors=value_color)
ax1.set_ylim(0,25)
ax2.set_ylim(0,25500)
plt.xlim(-100,10100)

fig.tight_layout()
fig.savefig(os.path.join(base_dir, "both_abs.jpg"), dpi=200)
plt.clf()
