import sys

with open(sys.argv[1], "r") as file:
    actions = file.readlines()

ship = [0, 0] # ns, we
waypoint = [1, 10] # ns, we

for el in actions:
    action = el[0]
    value = int(el[1:])

    if action == "N":
        waypoint[0] += value
    if action == "S":
        waypoint[0] -= value
    if action == "E":
        waypoint[1] += value
    if action == "W":
        waypoint[1] -= value
    if action == "L": # 1,10 -> 10,-1 -> -1,-10 -> -10,1 -> ...
        for i in range(value//90):
            nsw = waypoint[0]
            wew = waypoint[1]
            waypoint[0] = wew
            waypoint[1] = -nsw
    if action == "R": # 1,10 -> -10,1 -> -1,-10 -> 10,-1 -> ...
        for i in range(value//90):
            nsw = waypoint[0]
            wew = waypoint[1]
            waypoint[0] = -wew
            waypoint[1] = nsw
    if action == "F":
        nsw = value * waypoint[0]
        wew = value * waypoint[1]
        ship[0] += nsw
        ship[1] += wew


md = abs(ship[0]) + abs(ship[1])
print(md)
