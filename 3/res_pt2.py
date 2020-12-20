with open("input", "r") as file:
    grid = file.readlines()

def find_trees(grid, right, down):
    count = 0
    x = 0
    for y in range(0, len(grid), down):
        if grid[y][x] == "#":
            count += 1
        x = (x + right) % (len(grid[0]) - 1)
    return count

finds = [find_trees(grid,1,1),find_trees(grid,3,1),find_trees(grid,5,1),find_trees(grid,7,1),find_trees(grid,1,2)]
res = 1
for el in finds:
    res *= el
print(res)
