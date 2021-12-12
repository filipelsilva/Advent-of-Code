with open('input', 'r') as file:
    connections = list(map(lambda x: x[:-1].split("-"), file.readlines()))

# for el in connections:
#     print(el)

# connections = [["start", "A"],["start", "b"],["A", "c"],["A", "b"],["b", "d"],["A", "end"],["b", "end"]]
# connections = [["dc", "end"],["HN", "start"],["start", "kj"],["dc", "start"],["dc", "HN"],["LN", "dc"],["HN", "end"],["kj", "sa"],["kj", "HN"],["kj", "dc"]]
# connections =[["fs", "end"],["he", "DX"],["fs", "he"],["start", "DX"],["pj", "DX"],["end", "zg"],["zg", "sl"],["zg", "pj"],["pj", "he"],["RW", "he"],["fs", "DX"],["pj", "RW"],["zg", "RW"],["start", "pj"],["he", "WI"],["zg", "he"],["pj", "fs"],["start", "RW"]]

cave = {el: [] for el, _ in connections}
cave.update({el: [] for _, el in connections})

for el in connections:
    cave[el[0]].append(el[1])
    cave[el[1]].append(el[0])

# for el in cave:
#     print(el, cave[el])

# part 1
paths = []

def search_p1(current_cave, visited):
    if current_cave == "end":
        paths.append(visited + [current_cave,])
        # print(visited + [current_cave,])
    tmp = visited + [current_cave,]
    for el in cave[current_cave]:
        if el.isupper() or (el.islower() and el not in visited):
            search_p1(el, tmp)

search_p1("start", [])
# for el in paths:
#     print(el)
print(len(paths))

# part 2

paths = []

def test(visited, el):
    if el.isupper():
        return True
    else:
        if visited.count(el) == 0:
            return True
        elif visited.count(el) == 1:
            for cave in list(filter(lambda x: str(x).islower(), visited)):
                if cave != el and visited.count(cave) > 1:
                    return False
            return True
        else:
            return False

# print(test(['start', 'HN', 'kj', 'HN', 'dc', 'dc', 'HN', 'end'], 'kj')) 

def search_p2(current_cave, visited):
    if current_cave == "end" and visited.count("end") == 0:
        paths.append(visited + [current_cave,])
        # print(visited + [current_cave,])
    tmp = visited + [current_cave,]
    for el in cave[current_cave]:
        if el != "start" and test(visited + [current_cave,], el):
            search_p2(el, tmp)

search_p2("start", [])
# for el in paths:
#     print(el)
print(len(paths))
