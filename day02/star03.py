balls_max = {'red': 12, 'green': 13, 'blue': 14}

def parse_line(line):
    game, n_game, *draws = line.split()
    for i in range(len(draws)//2):
        n_ball = int(draws[2*i])
        color = draws[2*i+1].strip(',;')
        if n_ball > balls_max[color]:
            return 0
    return int(n_game.strip(':'))

with open('input.txt') as f:
    print(sum((parse_line(line) for line in f)))