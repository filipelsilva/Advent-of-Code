import re
import sys

def check_presence(passport):
    fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    for field in fields:
        if len(re.findall(field, passport)) != 1:
            return False
    return True

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
    if re.search("cid:", field):
        return "cid"
    return False

def check_passport(passport):
    fields = ""
    for el in re.split(r"\s|\n", passport):
        res = check_field(el)
        if res != False:
            fields += res
    return check_presence(fields)

with open(sys.argv[1], "r") as file:
    txt = file.read()

passports = re.split(r"\n\n", txt)
count = 0

for passport in passports:
    if check_passport(passport):
        count += 1

print(count)
