from collections import Counter

def parse_line(line):
    winningstr, havestr = line.split(':')[1].split('|')
    winning = set(winningstr.split())
    have = set(havestr.split())
    both = have.intersection(winning)
    return len(both)

with open('input.txt') as f:
    cards = f.readlines()

c = Counter(range(len(cards)))

for i in c.keys():
    for j in range(parse_line(cards[i])):
        c[i+j+1] += c[i]

print(sum(c.values()))