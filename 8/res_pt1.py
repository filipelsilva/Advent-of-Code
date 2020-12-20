import sys

with open(sys.argv[1], "r") as file:
    commands = file.readlines()

i = 0
accumulator = 0
lines = []

while (i < len(commands) and i not in lines):
    lines += [i]
    command = commands[i]
    if command[:3] == "acc":
        delta = int(command[4:])
        accumulator += delta
        i += 1
    elif command[:3] == "jmp":
        delta = int(command[4:])
        i += delta
    elif command[:3] == "nop":
        i += 1

print(accumulator, i)
