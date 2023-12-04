def parse_line(line):
    balls_min = {'red': 0, 'green': 0, 'blue': 0}
    game, n_game, *draws = line.split()
    for i in range(len(draws)//2):
        n_ball = int(draws[2*i])
        color = draws[2*i+1].strip(',;')
        balls_min[color] = max(balls_min[color], n_ball)
    return balls_min['red']*balls_min['green']*balls_min['blue']

with open('input.txt') as f:
    print(sum((parse_line(line) for line in f)))
