with open('input', 'r') as file:
    numbers = list(map(lambda x: x[:-1], file.readlines()))

# Test
# numbers = ['00100', '11110', '10110', '10111', '10101', '01111', '00111', '11100', '10000', '11001', '00010', '01010']

gamma = ''
epsilon = ''

for i in range(len(numbers[0])):
    digits = list(map(lambda x: x[i], numbers))
    if digits.count('0') > digits.count('1'):
        gamma += '0'
        epsilon += '1'
    else:
        gamma += '1'
        epsilon += '0'

gamma_dec = int(gamma, 2)
epsilon_dec = int(epsilon, 2)
print(gamma_dec * epsilon_dec)

oxygen = ''
co2 = ''

for i in range(len(numbers[0])):
    numbers_oxy = []
    for el in numbers:
        if el[:i] == oxygen:
            numbers_oxy += [el]
    digits_oxy = list(map(lambda x: x[i], numbers_oxy))
    if len(numbers_oxy) == 1:
        oxygen = numbers_oxy[0]
        break
    elif digits_oxy.count('0') > digits_oxy.count('1'):
        oxygen += '0'
    else:
        oxygen += '1'

for i in range(len(numbers[0])):
    numbers_co2 = []
    for el in numbers:
        if el[:i] == co2:
            numbers_co2 += [el]
    digits_co2 = list(map(lambda x: x[i], numbers_co2))
    if len(numbers_co2) == 1:
        co2 = numbers_co2[0]
        break
    elif digits_co2.count('0') > digits_co2.count('1'):
        co2 += '1'
    else:
        co2 += '0'

oxygen_dec = int(oxygen, 2)
co2_dec = int(co2, 2)
print(oxygen_dec * co2_dec)
