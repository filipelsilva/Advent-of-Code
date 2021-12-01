import sys

def check(time):
    for i in range(len(buses)):
        if buses[i] == -1:
            continue
        elif (time + i) % buses[i] != 0:
            return False
    return True

with open(sys.argv[1], "r") as file:
    txt = file.readlines()

buses = []
for el in txt[1].split(","):
    if el != "x":
        buses += [int(el)]
    else:
        buses += [-1]

time = 0
while True:
    if check(time):
        break
    else:
        time += 1

print(time)
