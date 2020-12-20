import sys
import re

with open(sys.argv[1], "r") as file:
    rules = file.readlines()

colors = ["shiny gold"]

for i in range(2):
    for rule in rules:
        for color in colors:
            lookup = re.search("bags contain .+ " + color, rule)
            if lookup:
                new = rule[:lookup.span()[0] - 1]
                if new not in colors:
                    colors += [new]
print(len(colors) - 1)
