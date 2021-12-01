with open('input', 'r') as file:
    numbers = list(map(int, file.readlines()))

count = 0
for i in range(1, len(numbers)):
    if numbers[i] > numbers[i-1]:
        count += 1

print(count)

count = 0
for i in range(2, len(numbers)):
    if sum(numbers[i-1:i+2]) > sum(numbers[i-2:i+1]):
        count += 1

print(count)
