import sys

def find_sum(vector, num):
    for i in range(len(vector)):
        for j in range(len(vector)):
            if i != j and vector[i] + vector[j] == num:
                return True
    return False

def find_weakness(numbers, preamble_size):
    for i in range(25, len(numbers)):
        if not find_sum(numbers[i-25: i], numbers[i]):
            return numbers[i]
    return False

with open(sys.argv[1], "r") as file:
    numbers = file.readlines()

for i in range(len(numbers)):
    numbers[i] = int(numbers[i])

print(find_weakness(numbers, 25))
