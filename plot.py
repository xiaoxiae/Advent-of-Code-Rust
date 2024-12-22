
import json
import matplotlib.pyplot as plt
import numpy as np

# Load the JSON file
with open("timing_results.json", "r") as file:
    data = json.load(file)

# Prepare the data for plotting
days = [entry["day"] for entry in data]
part1_times = [entry["times"]["1"][1] * 1000 for entry in data]
part2_times = [entry["times"]["2"][1] * 1000 for entry in data]

# Create the grouped bar plot
x = np.arange(len(days))  # Positions for the groups
width = 0.35  # Width of the bars

fig, ax = plt.subplots(figsize=(10, 6))
bar1 = ax.bar(x - width/2, part1_times, width, label="Part 1", color="skyblue")
bar2 = ax.bar(x + width/2, part2_times, width, label="Part 2", color="orange")

# Add labels, title, and legend
ax.set_xlabel("Days")
ax.set_ylabel("Time (ms)")
ax.set_title("Execution Times for Parts 1 and 2")
ax.set_xticks(x)
ax.set_xticklabels([f"Day {day}" for day in days])
ax.set_xticks(ax.get_xticks(), ax.get_xticklabels(), rotation=45, ha='right')
ax.legend()

# Show the plot
plt.tight_layout()
plt.show()
