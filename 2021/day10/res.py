with open('input', 'r') as file:
	navigation = list(map(lambda x: x[:-1], file.readlines()))

navigation = ["[({(<(())[]>[[{[]{<()<>>","[(()[<>])]({[<{<<[]>>(","{([(<{}[<>[]}>{[]{[(<()>","(((({<>}<{<{<>}{[]{[]{}","[[<[([]))<([[{}[[()]]]","[{[{({}]{}}([{[{{{}}([]","{<[[]]>}<{[{[{[]{()[[[]","[<(<(<(<{}))><([]([]()","<{([([[(<>()){}]>(<<{{","<{([{{}}[<[[[<>{}]]]>[]]"]

# print(navigation)

# part 1
score = 0
scores = []
scores_p1 = {')': 3, ']': 57, '}': 1197, '>': 25137}
scores_p2 = {')': 1, ']': 2, '}': 3, '>': 4}

for line in navigation:
	flag = 0
	complete = ""
	chars = []

	for el in line:
		if el in "([<{":
			chars.append(el)
		else:
			tmp = chars.pop()
			if tmp == "(" and el != ")":
				flag = 1
				score += scores_p1[el]
			elif tmp == "[" and el != "]":
				flag = 1
				score += scores_p1[el]
			elif tmp == "{" and el != "}":
				flag = 1
				score += scores_p1[el]
			elif tmp == "<" and el != ">":
				flag = 1
				score += scores_p1[el]

	if flag == 0:
		while chars != []:
			tmp = chars.pop()
			if tmp == "(":
				complete += ")"
			elif tmp == "[":
				complete += "]"
			elif tmp == "{":
				complete += "}"
			elif tmp == "<":
				complete += ">"
		tmp = 0
		for el in complete:
			tmp *= 5
			tmp += scores_p2[el]
		scores.append(tmp)

print(score)
print(sorted(scores)[len(scores)//2])
