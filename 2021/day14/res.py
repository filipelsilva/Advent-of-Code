with open('input', 'r') as file:
    template = file.readline()[:-1]
    file.readline()
    connections = list(map(lambda x: x[:-1].split(" -> "), file.readlines()))

# print(template)
# print(connections)

# template = "NNCB"
# connections = [["CH", "B"],["HH", "N"],["CB", "H"],["NH", "C"],["HB", "C"],["HC", "B"],["HN", "C"],["NN", "C"],["BH", "H"],["NC", "B"],["NB", "B"],["BN", "B"],["BB", "N"],["BC", "B"],["CC", "N"],["CN", "C"]]

match = list(map(lambda x: x[0], connections))

print(template)

for step in range(40):
    new_template = template[0]
    for i in range(1, len(template)):
        for j in range(len(match)):
            # print(f"{template[i-1:i+1]} == {match[i]}")
            if template[i-1:i+1] == match[j]:
                new_template += connections[j][1]
        new_template += template[i]
        # print(new_template)
    template = new_template
    # print(template)

count = dict((value, template.count(value)) for value in template)
print(max(count.values()) - min(count.values()))
