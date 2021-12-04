with open('input', 'r') as file:
    bingo = list(map(lambda x: x[:-1], file.readlines()))

numbers = list(map(int, bingo[0].split(',')))

board = []
boards = []
for line in bingo[2:]: 
    line_board = []
    if line == '':
        boards.append(board)
        board = []
        continue
    for i in range(0, len(line), 3):
        line_board.append(int(line[i:i+2]))
    board.append(line_board)

boards.append(board)

# numbers = [7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1]

# boards = [[[22,13,17,11, 0],[ 8, 2,23, 4,24],[21, 9,14,16, 7],[ 6,10, 3,18, 5],[ 1,12,20,15,19]],
# [[ 3,15, 0, 2,22],[ 9,18,13,17, 5],[19, 8, 7,25,23],[20,11,10,24, 4],[14,21,16,12, 6]],
# [[14,21,17,24, 4],[10,16,15, 9,19],[18, 8,23,26,20],[22,11,13, 6, 5],[ 2, 0,12, 3, 7]]]

def check(table, numbers):
    for line in table:
        if set(line).difference(set(numbers)) == set():
            return True

    for i in range(len(table[0])):
        if set(map(lambda x: x[i], table)).difference(set(numbers)) == set():
            return True

    return False

def score(table, numbers):
    ret = 0
    for line in table:
        for number in line:
            if number not in set(numbers):
                ret += number
    return ret

def part1(boards, numbers):
    for i in range(5, len(numbers)):
        drawn_numbers = numbers[:i]
        for board in boards:
            if check(board, drawn_numbers):
                return score(board, drawn_numbers) * drawn_numbers[-1]

print(part1(boards, numbers))

def part2(boards, numbers):
    number = -1
    drawn = []
    checked_boards = []
    for i in range(5, len(numbers)):
        drawn_numbers = numbers[:i]
        for board in boards:
            if board not in checked_boards:
                if check(board, drawn_numbers):
                    drawn = drawn_numbers
                    number = drawn_numbers[-1]
                    checked_boards.append(board)
    return score(checked_boards[-1], drawn) * number

print(part2(boards, numbers))
