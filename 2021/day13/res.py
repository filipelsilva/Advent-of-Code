with open('input', 'r') as file:
    lines = file.readlines()

points = []
folds = []

for i in range(len(lines)):
    if lines[i] != "\n" and lines[i][:4] != "fold":
        points.append(list(map(int, lines[i][:-1].split(","))))
    elif lines[i][:4] == "fold":
        if "x" in lines[i]:
            folds.append(["x", int(lines[i].split("=")[1])])
        elif "y" in lines[i]:
            folds.append(["y", int(lines[i].split("=")[1])])

# points = [[6,10],[0,14],[9,10],[0,3],[10,4],[4,11],[6,0],[6,12],[4,1],[0,13],[10,12],[3,4],[3,0],[8,4],[1,10],[2,14],[8,10],[9,0]]
# folds = [["y", 7], ["x", 5]]

# print(points)
# print(folds)

paper = [["." for _ in range(max(list(map(lambda x: x[0], points))) + 1)] for _ in range(max(list(map(lambda x: x[1], points))) + 1)]
for x, y in points:
    paper[y][x] = "#"

# for el in paper:
#     print(''.join(el))

counts = []
for direction, point in folds:
    if direction == "x":
        new_paper = list(map(lambda x: x[:point], paper))
        for y in range(len(paper)):
            for x in range(len(paper[y][point:])):
                if paper[y][x+point] == "#":
                    new_paper[y][len(new_paper[0]) - x] = "#"
    else:
        new_paper = paper[:point]
        for y in range(len(paper[point:])):
            for x in range(len(paper[y])):
                if paper[y+point][x] == "#":
                    new_paper[len(new_paper)-y][x] = "#"

    paper = new_paper

    # print("SEPARATOR", end='\n\n')
    # for el in paper:
    #     print(''.join(el))

    count = 0
    for el in paper:
        count += el.count("#")
    counts.append(count)

print(counts[0])

print("write it yourself, stop being lazy")
for el in paper:
    print(''.join(el))
