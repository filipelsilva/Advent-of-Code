def find_sum(vector, num):
    for i in range(len(expenses)):
        el = expenses[i]
        for j in range(len(expenses)):
            if i != j and el + expenses[j] == num:
                return el * expenses[j]

with open("input", "r") as file:
    expenses = file.readlines();

for i in range(len(expenses)):
    expenses[i] = int(expenses[i])

print(find_sum(expenses, 2020))
