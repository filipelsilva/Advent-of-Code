with open("input", "r") as file:
    txt = file.read()

passports = txt.split("\n\n")
fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
count = 0

for el in passports:
    flag = True
  
    for field in fields[:-1]:
        if field not in el:
            flag = False
    
    if flag:
        count += 1

print(count)
