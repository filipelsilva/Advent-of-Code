import sys

with open(sys.argv[1], "r") as file:
    commands = file.readlines()

i = 0
accumulator = 0
lines = []

while (i < len(commands)):
    lines += [i]
    command = commands[i][:3]
    delta = int(commands[i][4:])
    if command == "acc":
        accumulator += delta
        i += 1
    elif command == "jmp":
        i += delta
    elif command == "nop":
        i += 1

#print(accumulator)
