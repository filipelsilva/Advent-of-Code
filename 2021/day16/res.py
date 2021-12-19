with open('input', 'r') as file:
    packets = bin(int(file.read()[:-1], 16))[2:]

# print(packets)

sum_versions = 0

version = packets[:3]
typeID = packets[3:6]
