import re
import sys

fields = set(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])

def check_field(field):
    value = field[4:]
    if re.search("byr:[0-9]{4}", field) and 1920 <= int(value) <= 2002:
        return "byr"
    if re.search("iyr:[0-9]{4}", field) and 2010 <= int(value) <= 2020:
        return "iyr"
    if re.search("eyr:[0-9]{4}", field) and 2020 <= int(value) <= 2030:
        return "eyr"
    if re.search("hgt:[0-9]{3}cm", field) and 150 <= int(value[:-2]) <= 193:
        return "hgt"
    if re.search("hgt:[0-9]{2}in", field) and 59 <= int(value[:-2]) <= 76:
        return "hgt"
    if re.search("hcl:#([0-9]|[a-f]){6}", field):
        return "hcl"
    if re.search("ecl:(amb|blu|brn|gry|grn|hzl|oth)", field):
        return "ecl"
    if re.search("pid:[0-9]{9}", field):
        return "pid"
    return False

with open("input", "r") as file:
    txt = file.readlines()

count = 0
check = set()

for passport in txt:
    if passport == '\n':
        # print(fields, check)
        if fields == check:
            count += 1

        check = set()
        continue

    test = passport.rstrip("\n").split(" ")
    # print(test)
    for el in test:
        tmp = check_field(el.rstrip("\n")) 
        if tmp != False:
            check.add(tmp)
    # print(passport)

# print(fields, check)
if fields == check:
    count += 1

print(count)
