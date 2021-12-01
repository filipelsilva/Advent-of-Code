
#def getNumAdjacentOccupied(y, x, mapa):
#    count = 0
#    for j in range(y-1, y+2):
#        for i in range(x-1, x+2):
#            if 0 <= i < len(mapa[0]) and 0 <= j < len(mapa):
#                if mapa[j][i] == "#" and (j != y or i != x):
#                    count += 1
#    return count

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

layout = [[".",".","."],
          ["L","L","#"],
          ["#","#","#"]]

for y in range(len(layout)):
    for x in range(len(layout[0])):
        print(getNumAdjacentOccupied(y,x,layout))
