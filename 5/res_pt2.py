import sys

def getID(row, column):
    return row * 8 + column

def getSeatPosition(seat):
    begin = 0
    end = 128
    for char in seat[:7]:
        if char == "F":
            end = (end+begin)//2
        elif char == "B":
            begin = (end+begin)//2
    row = begin
    begin = 0
    end = 8
    for char in seat[7:]:
        if char == "L":
            end = (end+begin)//2
        elif char == "R":
            begin = (end+begin)//2
    column = begin
    return row, column

with open(sys.argv[1], "r") as file:
    seats = file.readlines()

pos = []
ids = []
missing = []
possible = []
possible_ids = []

for seat in seats:
    r, c = getSeatPosition(seat)
    pos += [(r, c)]
    ids += [getID(r, c)]

for row in range(1, 127):
    for column in range(0, 8):
        if (row, column) not in pos:
            missing += [(row, column)]

for seat in missing:
    if getID(seat[0], seat[1]) not in ids:
        possible += [seat]
        possible_ids += [getID(seat[0], seat[1])]

print(missing, possible, possible_ids)
