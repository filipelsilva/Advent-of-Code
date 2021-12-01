with open("input", "r") as file:
    passwords = file.readlines();

def check(password):
    minimum, maximum, char, passw = parse(password)
    count = 0
    for el in passw:
        if el == char:
            count += 1
    return minimum <= count <= maximum

def parse(password):
    i = 0
    minimum = 0
    maximum = 0
    while (password[i] != "-"):
        minimum = minimum * 10 + int(password[i])
        i += 1
    i += 1
    while (password[i] != " "):
        maximum = maximum * 10 + int(password[i])
        i += 1
    i += 1
    char = password[i]
    passw = password[i+1:]
    return minimum, maximum, char, passw


count = 0
for el in passwords:
    if check(el):
        count += 1

print(count)
