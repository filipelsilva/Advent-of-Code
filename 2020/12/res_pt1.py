import sys

directions = ["north", "east", "south", "west"]

with open(sys.argv[1], "r") as file:
    actions = file.readlines()

face = 1 # directions[1]
ns = 0
we = 0

for el in actions:
    action = el[0]
    value = int(el[1:])

    if action == "N":
        ns += value
    if action == "S":
        ns -= value
    if action == "E":
        we += value
    if action == "W":
        we -= value
    if action == "L":
        face = (face - value//90) % 4
    if action == "R":
        face = (face + value//90) % 4
    if action == "F":
        if face == 0:
            ns += value
        if face == 1:
            we += value
        if face == 2:
            ns -= value
        if face == 3:
            we -= value

md = abs(ns) + abs(we)
print(md)
