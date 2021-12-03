with open('input', 'r') as file:
    numbers = list(map(lambda x: x[:-1], file.readlines()))

# Test
# numbers = ['00100', '11110', '10110', '10111', '10101', '01111', '00111', '11100', '10000', '11001', '00010', '01010']

gamma = ''
epsilon = ''

for i in range(len(numbers[0])):
    digits = list(map(lambda x: x[i], numbers))
    number_0s = digits.count('0')
    number_1s = digits.count('1')
    if number_0s > number_1s:
        gamma += '0'
        epsilon += '1'
    else:
        gamma += '1'
        epsilon += '0'

gamma_dec = int(gamma, 2)
epsilon_dec = int(epsilon, 2)

print("Gamma: {}".format(gamma_dec))
print("Epsilon: {}".format(epsilon_dec))
print(gamma_dec * epsilon_dec)
    
