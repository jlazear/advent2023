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

def get_adjacent(coord, direction, c, parity, m):
    maxrow = len(m)
    maxcol = len(m[0])
    row, col = coord
    adjcoords = []
    if c == '|':
        if parity == 'R':
            if direction == 'up':
                adjcoords.append((row, col+1))
            else:
                adjcoords.append((row, col-1))
        else:
            if direction == 'up':
                adjcoords.append((row, col-1))
            else:
                adjcoords.append((row, col+1))
    elif c == '-':
        if parity == 'R':
            if direction == 'right':
                adjcoords.append((row+1, col))
            else:
                adjcoords.append((row-1, col))
        else:
            if direction == 'right':
                adjcoords.append((row-1, col))
            else:
                adjcoords.append((row+1, col))
    elif c == 'J':
        if ((parity == 'R' and direction == 'up')
            or (parity == 'L' and direction == 'left')):
            adjcoords.append((row+1, col))
            adjcoords.append((row, col+1))
    elif c == 'L':
        if ((parity == 'L' and direction == 'up')
            or (parity == 'R' and direction == 'right')):
            adjcoords.append((row+1, col))
            adjcoords.append((row, col-1))
    elif c == 'F':
        if ((parity == 'L' and direction == 'right')
            or (parity == 'R' and direction == 'down')):
            adjcoords.append((row-1, col))
            adjcoords.append((row, col-1))
    elif c == '7':
        if ((parity == 'L' and direction == 'down')
            or (parity == 'R' and direction == 'left')):
            adjcoords.append((row-1, col))
            adjcoords.append((row, col+1))
    adjcoords = [(adjrow, adjcol) for (adjrow, adjcol) in adjcoords
                 if (0 <= adjrow < maxrow) and (0 <= adjcol < maxcol)]
    return adjcoords
        
def get_neighbors(coord, m):
    maxrow = len(m)-1
    maxcol = len(m[0]) - 1
    row, col = coord
    neighbors = []
    if row > 0:
        neighbors.append((row-1, col))
    if row < maxrow:
        neighbors.append((row+1, col))
    if col > 0:
        neighbors.append((row, col-1))
    if col < maxcol:
        neighbors.append((row, col+1))
    return neighbors
        
def dfs(start, m, loop_coords, all_coords):
    coords = set()
    stack = [start]
    while stack:
        coord = stack.pop()
        coords.add(coord)
        neighbors = get_neighbors(coord, m)
        neighbors = [n for n in neighbors if n not in loop_coords
                                         and n not in coords
                                         and n not in all_coords]
        stack.extend(neighbors)
    return coords
    
m = []
coord = None
with open("input.txt") as f:
    for line in f:
        m.append([c for c in line.strip()])

for row in range(len(m)):
    for col in range(len(m[0])):
        if m[row][col] == 'S':
            coord = (row, col)


direction = 'up'
c = '0'
loop = []
coords = set()
rights = set()
lefts = set()
while c != 'S':
    loop.append((coord, direction, c))
    coords.add(coord)
    left = get_adjacent(coord, direction, c, 'L', m)
    right = get_adjacent(coord, direction, c, 'R', m)
    coord, direction, c = next_coord(coord, direction, m)
    if left:
        lefts.update(left)
    if right:
        rights.update(right)

lefts = lefts - coords
rights = rights - coords

all_lefts = set()
for left in lefts:
    new_lefts = dfs(left, m, coords, all_lefts)
    all_lefts.update(new_lefts)

all_rights = set()
for right in rights:
    new_rights = dfs(right, m, coords, all_rights)
    all_rights.update(new_rights)

print(f"{len(all_lefts) = }")
print(f"{len(all_rights) = }")
