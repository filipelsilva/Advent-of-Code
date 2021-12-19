def getAdjacent(x, y):
    ret = []
    if -1 < x < len(connections[0]) and -1 < y < len(connections):
        if y < len(connections) - 1:
            ret += [x,y+1],
        if x < len(connections[0]) - 1:
            ret += [x+1,y],
    return ret

with open('input', 'r') as file:
    connections = list(map(lambda x: list(map(lambda y: int(y), list(x[:-1]))), file.readlines()))

connections = [[1,1,6,3,7,5,1,7,4,2],[1,3,8,1,3,7,3,6,7,2],[2,1,3,6,5,1,1,3,2,8],[3,6,9,4,9,3,1,5,6,9],[7,4,6,3,4,1,7,1,1,1],[1,3,1,9,1,2,8,1,3,7],[1,3,5,9,9,1,2,4,2,1],[3,1,2,5,4,2,1,6,3,9],[1,2,9,3,1,3,8,5,2,1],[2,3,1,1,9,4,4,5,8,1]]

# print(connections)

ceiling = float("inf")
cost = [[ceiling for _ in range(len(connections[0]))] for _ in range(len(connections))]
cost[0][0] = 0

visited = []
queue = [[0,0]]

while cost[-1][-1] == float("inf"):
    xmin = -1
    ymin = -1
    minimum = ceiling
    for x, y in queue:
        if cost[y][x] < minimum:
            minimum = cost[y][x]
            xmin = x
            ymin = y
    adjacent = getAdjacent(xmin, ymin)
    visited.append([xmin, ymin],)
    queue.remove([xmin, ymin],)
    for x, y in adjacent:
        # if [x, y] not in visited:
        queue.append([x, y],)
        new_cost = cost[ymin][xmin] + connections[y][x]
        if new_cost < cost[y][x]:
            cost[y][x] = new_cost
    print("SEPARATOR")
    for line in cost:
        print(line)

print(cost[-1][-1])
