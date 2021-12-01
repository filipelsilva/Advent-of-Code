import sys
import copy

def split(word):
    return [char for char in word]

def getOccupiedSeat(y, x, vector, mapa):
    j = y
    i = x
    while 0 <= i < len(mapa[0]) and 0 <= j < len(mapa):
        if mapa[j][i] == "#" and (j != y or i != x):
            return True
        if mapa[j][i] == "L" and (j != y or i != x):
            return False
        j += vector[0]
        i += vector[1]
    return False

def getNumAdjacentOccupied(y, x, mapa):
    count = 0
    vectors = [(0,1),(0,-1),(1,0),(-1,0),(1,1),(-1,1),(1,-1),(-1,-1)]
    for el in vectors:
        if getOccupiedSeat(y, x, el, mapa):
            count += 1
    return count

def printSeats():
    for el in layout:
        print("".join(el))
    print("\n")

def countOccupied():
    count = 0
    for y in range(len(layout)):
        for x in range(len(layout[0])):
            if layout[y][x] == "#":
                count += 1
    return count

with open(sys.argv[1], "r") as file:
    layout = list(map(split, file.read().split()))

layoutCopy = []
while layoutCopy != layout:
    layoutCopy = copy.deepcopy(layout)
    print(getNumAdjacentOccupied(0,0,layout))
    printSeats()
    for y in range(len(layoutCopy)):
        for x in range(len(layoutCopy[0])):
            if layoutCopy[y][x] == "L" and getNumAdjacentOccupied(y, x, layoutCopy) == 0:
                layout[y][x] = "#"
            elif layoutCopy[y][x] == "#" and getNumAdjacentOccupied(y, x, layoutCopy) >= 5:
                layout[y][x] = "L"
print(countOccupied())
