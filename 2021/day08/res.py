import itertools

mapping = {0: set('abcefg'), 1: set('cf'), 2: set('acdeg'), 3: set('acdfg'), 4: set('bcdf'), 5: set('abdfg'), 6: set('abdefg'), 7: set('acf'), 8: set('abcdefg'), 9: set('abcdfg')}

with open('input', 'r') as file:
	patterns = file.readlines()

# patterns = ["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n", "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n", "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n", "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n", "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n", "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n", "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n", "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n", "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n", "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n"]

patterns = list(map(lambda x: x[:-1].split(' | '), patterns))
patterns = list(map(lambda x: [x[0].split(' '), x[1].split(' ')], patterns))

# print(patterns)

# part 1

count = 0
for el in patterns:
	for digits in el[1]:
		if len(digits) in (2, 3, 4, 7):
			count += 1

print(count)

# part 2

permutations = list(itertools.permutations(list("abcdefg")))

for perm in permutations:
	changed = {"a": "", "b": "", "c": "", "d": "", "e": "", "f": "", "g": ""}
	for old, new in zip("abcdefg", perm):
		changed[old] = new
	print(changed)
