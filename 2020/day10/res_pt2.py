import sys
import itertools

def check(vector):
    for i in range(1, len(adapters)):
        if adapters[i] - adapters[i-1] > 3:
            return False
    return True

with open(sys.argv[1], "r") as file:
    adapters = list(map(int, file.readlines()))
adapters.sort()
adapters = [0] + adapters + [adapters[-1] + 3]

for el in itertools.permutations(adapters):
    if check(adapters):
        count += 1
print(count)
