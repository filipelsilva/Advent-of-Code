import sys
import re

with open(sys.argv[1], "r") as file:
    rules = file.readlines()

count = 0
colors = ["shiny gold"]

for i in range(2):
    for rule in rules:
        for color in colors:
            if (re.search(color + " bags contain .+ " + color, rule))
                lookup = re.findall("[0-9]\s.+\sbag.", rule)[0]
                lookup = re.split("\sbags,|\sbag,|\sbags|\sbag", lookup)
                lookup = [el.strip() for el in lookup if el]
                
#                if lookup:
#                    new = rule[:lookup.span()[0] - 1]
#                    if new not in colors:
#                        colors += [new]
#print(len(colors) - 1)
