import re
import sys

fields = set(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])

def check_field(field):
    tipo = field[:3]
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
    if tipo == "hcl":
        test = 1
        if value[0] != "#":
            test = 0
        for el in value[1:]:
            if el not in set(["0","1","2","3","4","5","6","7","8","9","a","b","c","d","e","f"]):
                test = 0
        if test == 1:
            return "hcl"
    if tipo == "ecl" and value in set(["amb","blu","brn","gry","grn","hzl","oth"]):
        return "ecl"
    if tipo == "pid":
        test = 1
        if len(value) != 9:
            test = 0
        for el in value:
            if el not in set(["0","1","2","3","4","5","6","7","8","9"]):
                test = 0
        if test == 1:
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

# print(fields, check)
if fields == check:
    count += 1

print(count)
