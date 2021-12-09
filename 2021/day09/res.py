with open('input', 'r') as file:
	heightmap = list(map(lambda x: list(map(int, list(x[:-1]))), file.readlines()))

# heightmap = [[2,1,9,9,9,4,3,2,1,0],[3,9,8,7,8,9,4,9,2,1],[9,8,5,6,7,8,9,8,9,2],[8,7,6,7,8,9,6,7,8,9],[9,8,9,9,9,6,5,6,7,8]]

# print(heightmap)

# part 1

def isLowPoint(x, y, heightmap):
	point = heightmap[y][x]
	if y == 0:
		up = 10
	else:
		up = heightmap[y-1][x]
	if y == len(heightmap) - 1:
		down = 10
	else:
		down = heightmap[y+1][x]
	if x == 0:
		left = 10
	else:
		left = heightmap[y][x-1]
	if x == len(heightmap[y]) - 1:
		right = 10
	else:
		right = heightmap[y][x+1]
	return point < up and point < down and point < left and point < right

sum_risk = 0
for y in range(len(heightmap)):
	for x in range(len(heightmap[y])):
		if isLowPoint(x, y, heightmap):
			sum_risk += 1 + heightmap[y][x]

print(sum_risk)

# part 2
visited_all = []
def getBasin(x, y, heightmap, visited):
	visited_all.append([x, y],)
	new_visited = visited
	if y < 0 or y > len(heightmap) - 1 or x < 0 or x > len(heightmap[y]) - 1:
		return 0
	if heightmap[y][x] == 9:
		return 0
	for el in visited:
		if el[0] == x and el[1] == y:
			return 0
	new_visited.append([x, y],)
	ret = 1
	ret += getBasin(x-1, y, heightmap, new_visited)
	ret += getBasin(x+1, y, heightmap, new_visited)
	ret += getBasin(x, y-1, heightmap, new_visited)
	ret += getBasin(x, y+1, heightmap, new_visited)
	return ret

largest = []
for y in range(len(heightmap)):
	for x in range(len(heightmap[y])):
		flag = 1
		for el in visited_all:
			if el[0] == x and el[1] == y:
				flag = 0
		if flag == 1:
			tmp = getBasin(x, y, heightmap, list())
			largest.append(tmp)

max1 = max(largest)
largest.remove(max1)
max2 = max(largest)
largest.remove(max2)
max3 = max(largest)
largest.remove(max3)
print(max1 * max2 * max3)
