import sys

with open(sys.argv[1], "r") as file:
    adapters = list(map(int, file.readlines()))
adapters.sort()

adapters = [0] + adapters + [adapters[-1] + 3]
one = 0
two = 0
three = 0

for i in range(1, len(adapters)):
    print(adapters[i-1], adapters[i])
    if adapters[i] - adapters[i-1] == 1:
        one += 1
    elif adapters[i] - adapters[i-1] == 2:
        two += 1
    elif adapters[i] - adapters[i-1] == 3:
        three += 1

print(one * three)
