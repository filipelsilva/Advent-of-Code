import sys

def find_sum(vector, num):
    for i in range(len(vector)):
        for j in range(len(vector)):
            if i != j and vector[i] + vector[j] == num:
                return True
    return False

def find_number(numbers, preamble_size):
    for i in range(25, len(numbers)):
        if not find_sum(numbers[i-25: i], numbers[i]):
            return numbers[i]
    return False

def find_largest(numbers):
    ret = numbers[0]
    for el in numbers:
        if el > ret:
            ret = el
    return ret

def find_smallest(numbers):
    ret = numbers[0]
    for el in numbers:
        if el < ret:
            ret = el
    return ret

def find_weakness(numbers, target):
    for i in range(len(numbers)):
        total = 0
        j = i
        while j < len(numbers) and total < target:
            total += numbers[j]
            j += 1
        if total == target:
            return find_smallest(numbers[i:j+1]) + find_largest(numbers[i:j+1])

with open(sys.argv[1], "r") as file:
    numbers = file.readlines()

for i in range(len(numbers)):
    numbers[i] = int(numbers[i])

number = find_number(numbers, 25)
weakness = find_weakness(numbers, number)

print(weakness)
