import sys
import string

with open(sys.argv[1], "r") as file:
    answers = file.read()

groups = [group.splitlines() for group in answers.split("\n\n")]
count = 0
for group in groups:
    letters = dict.fromkeys(string.ascii_lowercase, 0)
    numPerson = len(group)
    for line in group:
        for char in line:
            letters[char] += 1
    for el in letters:
        if letters[el] == numPerson:
            count += 1
print(count)
