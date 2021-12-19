with open('input', 'r') as file:
    template = file.readline()[:-1]
    file.readline()
    connections = list(map(lambda x: x[:-1].split(" -> "), file.readlines()))

# print(template)
# print(connections)

template = "NNCB"
connections = [["CH", "B"],["HH", "N"],["CB", "H"],["NH", "C"],["HB", "C"],["HC", "B"],["HN", "C"],["NN", "C"],["BH", "H"],["NC", "B"],["NB", "B"],["BN", "B"],["BB", "N"],["BC", "B"],["CC", "N"],["CN", "C"]]
connections = dict((values[0], values[1]) for values in connections)

count = dict((letter, 0) for letter in list(map(lambda x: x[1], connections)))

# def countLetters(string, step):
#     for i in range(1, len(string)):
#         count[string[i-1]] += 1

# A fazer:
# dict com {par: count de vezes que aparece o par}
# recursivo
# ir buscar dados para recursão ao dicionário com {par: letra do meio}


print(max(count.values()) - min(count.values()))
