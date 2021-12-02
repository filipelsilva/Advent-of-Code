with open('input', 'r') as file:
    coords = list(map(lambda x: x[:-1], file.readlines()))

directions = {'forward': 0, 'up': 0, 'down': 0}

for coord in coords:
    for direction in directions:
        if coord[:len(direction)] == direction:
            directions[direction] += int(coord[len(direction)+1])

horizontal = directions['forward']
depth = directions['down'] - directions['up']
print(horizontal * depth)

directions = ['forward', 'up', 'down']

aim = 0
horizontal = 0
depth = 0
for coord in coords:
    for direction in directions:
        if coord[:len(direction)] == direction:
            if direction == 'forward':
                x = int(coord[len(direction)+1])
                horizontal += x
                depth += (aim * x)
            elif direction == 'up':
                aim -= int(coord[len(direction)+1])
            elif direction == 'down':
                aim += int(coord[len(direction)+1])

print(horizontal * depth)
