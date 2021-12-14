with open('input', 'r') as file:
    template = file.readline()[:-1]
    file.readline()
    connections = list(map(lambda x: x[:-1].split(" -> "), file.readlines()))

# print(template)
# print(connections)

template = "NNCB"
connections = [["CH", "B"],["HH", "N"],["CB", "H"],["NH", "C"],["HB", "C"],["HC", "B"],["HN", "C"],["NN", "C"],["BH", "H"],["NC", "B"],["NB", "B"],["BN", "B"],["BB", "N"],["BC", "B"],["CC", "N"],["CN", "C"]]

conns = dict((value[0], value[0][0] + value[1] + value[0][1]) for value in connections)
# print(conns)

def getIter(step, template):
    next_template = template[0]
    for i in range(step, len(template)):
        # print(template[i-step:i+1], step+1)
        if template[i-step:i+1] in conns:
            next_template += conns[template[i-step:i+1]][1:]
        else:
            next_template += getIter(step-1, template[i-step:i+1])
    return next_template

for step in range(2):
    # print(step+1)
    tmp = dict()
    for el in conns:
        tmp.update({conns[el]: getIter(step+1, conns[el])})
    conns.update(tmp)
    print(conns)

# print(max([len(el) for el in conns]))
# final = getIter(max([len(el) for el in conns] + [len(template)]), template)
final = getIter(4, template)
print(final)

# count = dict((value, template.count(value)) for value in template)
# print(max(count.values()) - min(count.values()))
