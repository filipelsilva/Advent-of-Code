with open('input', 'r') as file:
    file_lines = list(map(lambda x: x[:-1], file.readlines()))

coords = []
for i in range(len(file_lines)):
    tmp = list(map(lambda x: x.split(','), file_lines[i].split(" -> ")))
    tmp = list(map(lambda x: list(map(int, x)), tmp))
    coords.append(tmp)

# coords = [[[0,9],[5,9]],[[8,0],[0,8]],[[9,4],[3,4]],[[2,2],[2,1]],[[7,0],[7,4]],[[6,4],[2,0]],[[0,9],[2,9]],[[3,4],[1,4]],[[0,0],[8,8]],[[5,5],[8,2]]]

x_max = max(list(map(lambda x: max(x[0][0], x[1][0]), coords)))
y_max = max(list(map(lambda x: max(x[0][1], x[1][1]), coords)))

# part 1

diagram = [[0 for _ in range(x_max + 1)] for _ in range(y_max + 1)]

for coord in coords:
    p1 = coord[0]
    p2 = coord[1]
    if p1[0] == p2[0]:
        begin = min(p1[1], p2[1])
        end = max(p1[1], p2[1]) + 1
        for y in range(begin, end):
            diagram[y][p1[0]] += 1
    elif p1[1] == p2[1]:
        begin = min(p1[0], p2[0])
        end = max(p1[0], p2[0]) + 1
        for x in range(begin, end):
            diagram[p1[1]][x] += 1

count = 0
for line in diagram:
    for num in line:
        if num > 1:
            count += 1
    # print(line)

print(count)

# part2

diagram = [[0 for _ in range(x_max + 1)] for _ in range(y_max + 1)]

for coord in coords:
    p1 = coord[0]
    p2 = coord[1]
    if p1[0] == p2[0]:
        begin = min(p1[1], p2[1])
        end = max(p1[1], p2[1]) + 1
        for y in range(begin, end):
            diagram[y][p1[0]] += 1
    elif p1[1] == p2[1]:
        begin = min(p1[0], p2[0])
        end = max(p1[0], p2[0]) + 1
        for x in range(begin, end):
            diagram[p1[1]][x] += 1
    else:
        step_x, step_y = 1, 1
        if p1[0] > p2[0]:
            step_x = -1
        if p1[1] > p2[1]:
            step_y = -1
        for x, y in zip(range(p1[0], p2[0] + step_x, step_x), range(p1[1], p2[1] + step_y, step_y)):
            diagram[y][x] += 1

count = 0
for line in diagram:
    for num in line:
        if num > 1:
            count += 1
    # print(line)

print(count)
