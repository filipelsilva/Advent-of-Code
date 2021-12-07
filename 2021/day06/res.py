with open('input', 'r') as file:
	lanternfish = list(file.read())[:-1]

lanternfish = list(map(int, filter(lambda x: x != ',', lanternfish)))

lanternfish = [3,4,3,1,2]

# print(lanternfish)

# def nextnumber(x):
#     if x > 6:
#         return x - 1
#     else:
#         return (x - 1) % 7

# length = 0
# for el in lanternfish:
#     tmp = [el]
#     for day in range(256):
#         count = tmp.count(0)
#         tmp = list(map(nextnumber, tmp))
#         for i in range(count):
#             tmp.append(8)
#         # print(tmp)
#     length += len(tmp)

# print(length)

count = 0
def generator(timer, day, total_days):
	print(f"generator(8, {day}, {total_days})")
	print((total_days - day - timer)//7 + 1)
	if day+timer >= total_days:
		return 0
	ret = (total_days - day - timer)//7 + 1
	for i in range(day+timer+1, total_days, 7):
		ret += generator(8, i, total_days)
	return ret

# count = 0
# for el in lanternfish:
# 	count += generator(el, 0, 18) + 1
# 	print(generator(el, 0, 18))

print(generator(4, 0, 18))

# print(count)

# 4
# 4
# 4 -> 3
# 6
# 5
# 28

# 5 + 5 + 5 + 7

#  0: 3 4 3 1 2
#  1: 2 3 2 0 1
#  2: 1 2 1 6 0 8
#  3: 0 1 0 5 6 7 8
#  4: 6 0 6 4 5 6 7 8 8
#  5: 5 6 5 3 4 5 6 7 7 8
#  6: 4 5 4 2 3 4 5 6 6 7
#  7: 3 4 3 1 2 3 4 5 5 6
#  8: 2 3 2 0 1 2 3 4 4 5
#  9: 1 2 1 6 0 1 2 3 3 4 8
# 10: 0 1 0 5 6 0 1 2 2 3 7 8
# 11: 6 0 6 4 5 6 0 1 1 2 6 7 8 8 8
# 12: 5 6 5 3 4 5 6 0 0 1 5 6 7 7 7 8 8
# 13: 4 5 4 2 3 4 5 6 6 0 4 5 6 6 6 7 7 8 8
# 14: 3 4 3 1 2 3 4 5 5 6 3 4 5 5 5 6 6 7 7 8
# 15: 2 3 2 0 1 2 3 4 4 5 2 3 4 4 4 5 5 6 6 7
# 16: 1 2 1 6 0 1 2 3 3 4 1 2 3 3 3 4 4 5 5 6 8
# 17: 0 1 0 5 6 0 1 2 2 3 0 1 2 2 2 3 3 4 4 5 7 8
# 18: 6 0 6 4 5 6 0 1 1 2 6 0 1 1 1 2 2 3 3 4 6 7 8 8 8 8
