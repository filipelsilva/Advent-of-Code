import sys
from functools import reduce

def chinese_remainder(n, a):
    sum = 0
    prod = reduce(lambda a, b: a*b, n)
    for n_i, a_i in zip(n, a):
        p = prod // n_i
        sum += a_i * mul_inv(p, n_i) * p
    return sum % prod

def mul_inv(a, b):
    b0 = b
    x0, x1 = 0, 1
    if b == 1: return 1
    while a > 1:
        q = a // b
        a, b = b, a%b
        x0, x1 = x1 - q * x0, x0
    if x1 < 0: x1 += b0
    return x1

with open(sys.argv[1], "r") as file:
    txt = file.readlines()
buses = txt[1].split(",")

n = [] # the buses
a = [] # the results
for i in range(len(buses)):
    if buses[i] != "x":
        n += [int(buses[i])]
        a += [i]

print(chinese_remainder(n, a))
