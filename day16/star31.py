def energized(directions):
    return sum([sum([len(d) != 0 for d in row]) for row in directions])

def update_cell(c, directions):
    newdirections = set()
    if c == '-':
        if 'U' in directions or 'D' in directions:
            newdirections.add('L')
            newdirections.add('R')
        if 'L' in directions:
            newdirections.add('L')
        if 'R' in directions:
            newdirections.add('R')
    elif c == '|':
        if 'L' in directions or 'R' in directions:
            newdirections.add('U')
            newdirections.add('D')
        if 'U' in directions:
            newdirections.add('U')
        if 'D' in directions:
            newdirections.add('D')
    elif c == '/':
        if 'R' in directions:
            newdirections.add('U')
        if 'L' in directions:
            newdirections.add('D')
        if 'U' in directions:
            newdirections.add('R')
        if 'D' in directions:
            newdirections.add('L')
    elif c == '\\':
        if 'R' in directions:
            newdirections.add('D')
        if 'L' in directions:
            newdirections.add('U')
        if 'U' in directions:
            newdirections.add('L')
        if 'D' in directions:
            newdirections.add('R')
    elif c == '.':
        newdirections.update(directions)
    return newdirections

def update_cells(data, directions):
    nrow = len(directions)
    ncol = len(directions[0])
    for row in range(nrow):
        for col in range(ncol):
            c = data[row][col]
            directions[row][col] = update_cell(c, directions[row][col])

def propagate(directions):
    nrow = len(directions)
    ncol = len(directions[0])
    newdirections = [[set() for _ in row] for row in directions]
    newdirections[0][0].add('R')
    for row in range(nrow):
        for col in range(ncol):
            d = directions[row][col]
            if 'U' in d and row > 0:
                newdirections[row-1][col].add('U')
            if 'D' in d and row < nrow-1:
                newdirections[row+1][col].add('D')
            if 'L' in d and col > 0:
                newdirections[row][col-1].add('L')
            if 'R' in d and col < ncol-1:
                newdirections[row][col+1].add('R')
    return newdirections

def update(data, directions):
    update_cells(data, directions)
    directions = propagate(directions)
    return directions

with open('input.txt') as f:
    data = [[c for c in line.strip()] for line in f.readlines()]
directions = [[set() for _ in row] for row in data]
directions[0][0].add('R')

h = -1
newh = hash(str(directions))
while h != newh:
    h = newh
    directions = update(data, directions)
    newh = hash(str(directions))
    
print(energized(directions))
