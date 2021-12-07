with open('input', 'r') as file:
	lanternfish = list(file.read())[:-1]

lanternfish = list(map(int, filter(lambda x: x != ',', lanternfish)))

# lanternfish = [3,4,3,1,2]

# print(lanternfish)

# old

# def nextnumber(x):
#     if x > 6:
#         return x - 1
#     else:
#         return (x - 1) % 7

# length = 0
# for el in lanternfish:
#     tmp = [el]
#     for day in range(80):
#         count = tmp.count(0)
#         tmp = list(map(nextnumber, tmp))
#         for i in range(count):
#             tmp.append(8)
#         # print(tmp)
#     length += len(tmp)

# print(length)

# new

def generator(timer, day, total_days, table):
	if day > total_days:
		return 0 
	ret = 1
	for i in range(day+timer+1, total_days+1, 7):
		if table[i] == -1:
			table[i] = generator(8, i, total_days, table)
		ret += table[i]
	return ret

def getAnswer(max_days):
	table = [-1 for _ in range(max_days + 1)]
	for i in reversed(range(len(table))):
		table[i] = generator(8, i, max_days, table)

	count = 0
	for el in lanternfish:
		count += generator(el, 0, max_days, table)

	return count

print(getAnswer(80))
print(getAnswer(256))
