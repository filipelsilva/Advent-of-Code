import sys

with open(sys.argv[1], "r") as file:
    answers = file.read()

groups = [group.splitlines() for group in answers.split("\n\n")]
count = 0
for group in groups:
    letters = []
    for line in group:
        for char in line:
            if char not in letters:
                letters += [char]
    count += len(letters)
print(count)
