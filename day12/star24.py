from collections import deque
from functools import cache

@cache
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
        sumrules = sum(rules)
        print(f"{sumrules = } {len(rules) = } {len(valids) = }")  #DELME
        for i, valid in enumerate(valids):
            toadd = place(valid, n)
            # if len(valids) == 500:
            #     breakpoint()  #DELME
            # print(f"\t{i = } {toadd = }")
            # for v in toadd:
            #     print(f"{v} {v.count('#')} {v.count('?')} {v.count('#') + v.count('?')} {sumrules}")
            ntoadd0 = len(toadd)  #DELME
            toadd = [v for v in toadd if v.count('#') + v.count('?') >= sumrules]
            ntoadd1 = len(toadd)  #DELME
            if ntoadd1 - ntoadd0 > 0:  #DELME
                print(f"removed {ntoadd1 - ntoadd0}")  #DELME
            newvalids.extend(toadd)
        valids = newvalids
        newvalids = []
    valids = [valid for valid in valids if '#' not in valid]
    return len(valids)

def parse_line(line, fold_factor=5):
    s, rules = line.split()
    s = '?'.join([s]*fold_factor)
    rules = deque(int(x) for x in rules.split(',')) * fold_factor
    return count(s, rules)

with open('input.txt') as f:
    s = 0
    for i, line in enumerate(f):
        print(f"{i = } {line = }")  #DELME
        ss = parse_line(line, 5)
        s += ss
    # s = sum(parse_line(line, 5) for line in f)
print(s)
