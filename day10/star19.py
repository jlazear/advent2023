m = []
coord = None
with open("input.txt") as f:
    for line in f:
        m.append([c for c in line.strip()])

for row in range(len(m)):
    for col in range(len(m[0])):
        if m[row][col] == 'S':
            coord = (row, col)

d = {'up': {'|': 'up',
            '7': 'left',
            'F': 'right',
            'S': 'up'},
        'right': {'-': 'right',
                '7': 'down',
                'J': 'up'},
        'down': {'|': 'down',
                'L': 'right',
                'J': 'left'},
        'left': {'-': 'left',
                'L': 'up',
                'F': 'down'}}

def next_coord(coord, direction, m):
    row, col = coord
    if direction == 'up':
        nextcoord = (row-1, col)
    elif direction == 'right':
        nextcoord = (row, col+1)
    elif direction == 'down':
        nextcoord = (row+1, col)
    else:  # direction == 'left'
        nextcoord = (row, col-1)
    
    nextrow, nextcol = nextcoord
    c = m[nextrow][nextcol]
    nextdirection = d[direction][c]
    return nextcoord, nextdirection, c

direction = 'up'
c = '0'
loop = []
while c != 'S':
    loop.append((coord, direction, c))
    coord, direction, c = next_coord(coord, direction, m)

print(len(loop)//2)
