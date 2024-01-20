from collections import deque

def place(s, n):
    valids = []
    s += '.'
    for i in range(len(s) - n):
        if '.' in s[i:i+n] or '#' in s[:i]:
            continue
        elif s[i+n] in '.?':
            valids.append(s[i+n+1:-1])
    return valids
    
def count(s, rules):
    rules = deque(rules)
    valids = [s]
    newvalids = []
    while rules:
        n = rules.popleft()
        for i, valid in enumerate(valids):
            newvalids.extend(place(valid, n))
        valids = newvalids
        newvalids = []
    valids = [valid for valid in valids if '#' not in valid]
    return len(valids)

def parse_line(line):
    s, rules = line.split()
    rules = deque(int(x) for x in rules.split(','))
    return count(s, rules)

with open('input.txt') as f:
    s = sum(parse_line(line) for line in f)
print(s)
