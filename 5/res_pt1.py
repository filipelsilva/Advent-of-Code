import sys

def getID(row, column):
    return row * 8 + column

def biggest(list):
    biggest = list[0]
    for el in list:
        if el > biggest:
            biggest = el
    return biggest

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

ids = []

for seat in seats:
    r, c = getSeatPosition(seat)
    ids += [getID(r, c)]

print(ids)
print(biggest(ids))
