with open('input', 'r') as file:
	energy = list(map(lambda x: list(map(int, list(x[:-1]))), file.readlines()))

# energy = [[5,4,8,3,1,4,3,2,2,3],[2,7,4,5,8,5,4,7,1,1],[5,2,6,4,5,5,6,1,7,3],[6,1,4,1,3,3,6,1,4,6],[6,3,5,7,3,8,5,4,7,8],[4,1,6,7,5,2,4,6,4,5],[2,1,7,6,8,4,1,7,2,1],[6,8,8,2,8,8,1,1,3,4],[4,8,4,6,8,4,8,5,5,4],[5,2,8,3,7,5,1,5,2,6]]

# print("STEP 0")
# for line in energy:
# 	print(line)

count = 0
flashed = []

def flash(i, j):
	for el in flashed:
		if el[0] == i and el[1] == j:
			return
	flashed.append([i, j],)
	global count
	count += 1
	adjacent = [[i-1,j-1],[i-1,j],[i-1,j+1],[i,j-1],[i,j+1],[i+1,j-1],[i+1,j],[i+1,j+1]]
	for el in adjacent:
		if 0 <= el[0] < len(energy) and 0 <= el[1] < len(energy[el[0]]):
			energy[el[0]][el[1]] += 1
			if energy[el[0]][el[1]] > 9:
				flash(el[0], el[1])

step = 0 

# for -> part 1, while -> part 2

while len(flashed) != 100:
# for step in range(100):
	step += 1
	for i in range(len(energy)):
		energy[i] = list(map(lambda x: x + 1, energy[i]))

	flashed = []
	for i in range(len(energy)):
		for j in range(len(energy[i])):
			if energy[i][j] > 9:
				flash(i, j)

	for i in range(len(energy)):
		for j in range(len(energy[i])):
			if energy[i][j] > 9:
				energy[i][j] = 0
	

	# print(f"STEP {step+1}")
	# for line in energy:
	# 	print(line)

print(count)
print(step)
