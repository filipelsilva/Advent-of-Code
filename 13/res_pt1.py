import sys

with open(sys.argv[1], "r") as file:
    txt = file.readlines()

time = int(txt[0])
buses = txt[1].split(",")
buses = [int(el) for el in buses if el != "x"]

earliest = time
busID = 0
while busID == 0:
    for el in buses:
        if earliest % el == 0:
            busID = el
            break
    earliest += 1

print(busID, earliest - 1)
print(busID * (earliest - 1 - time))
