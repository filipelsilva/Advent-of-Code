def find_sum(vector, num):
    for i in range(len(expenses)):
        for j in range(len(expenses)):
            for k in range(len(expenses)):
                if i != j != k and expenses[i] + expenses[j] + expenses[k] == num:
                    return expenses[i] * expenses[j] * expenses[k]

with open("input", "r") as file:
    expenses = file.readlines();

for i in range(len(expenses)):
    expenses[i] = int(expenses[i])

print(find_sum(expenses, 2020))
