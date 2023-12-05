def parse_line(line):
    winningstr, havestr = line.split(':')[1].split('|')
    winning = set(winningstr.split())
    have = set(havestr.split())
    both = have.intersection(winning)
    if len(both) == 0:
        return 0
    else:
        return 2**(len(both) - 1)

with open('input.txt') as f:
    print(sum(parse_line(line) for line in f))
