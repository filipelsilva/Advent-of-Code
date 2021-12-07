import statistics

with open('input', 'r') as file:
	positions = file.read()[:-1]

positions = list(map(int, positions.split(',')))

# positions = [16,1,2,0,4,2,7,1,2,14]

# print(positions)

def getDistanceP1(positions, align):
	return sum(list(map(lambda x: abs(align - x), positions)))

print(getDistanceP1(positions, int(statistics.median(positions))))

def getDistanceP2(positions, align):
	return sum(list(map(lambda x: abs(align - x)*(abs(align - x) + 1)/2, positions)))

minimum = 100000000000000000000000
for pos in range(max(positions)):
	tmp = getDistanceP2(positions, pos)
	if tmp < minimum:
		minimum = tmp

print(minimum)
