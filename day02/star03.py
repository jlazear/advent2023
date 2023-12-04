balls_max = {'red': 12, 'green': 13, 'blue': 14}

def parse_line(line):
    game, n_game, *draws = line.split()
    for i in range(len(draws)//2):
        n_ball = int(draws[2*i])
        color = draws[2*i+1].strip(',;')
        if n_ball > balls_max[color]:
            return 0
    n = int(n_game.strip(':'))
    return n

s = 0
with open('input.txt') as f:
    for line in f:
        s += parse_line(line)

print(s)
