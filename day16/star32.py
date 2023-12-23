def update(pattern, state):
    maxrow, maxcol = len(pattern), len(pattern[0])
    direction, (row, col) = state
    newstates = []
    c = pattern[row][col]
    if c == '-':
        if direction in 'UDL' and col > 0:
            newstates.append(('L', (row, col-1)))
        if direction in 'UDR' and col < maxcol - 1:
            newstates.append(('R', (row, col+1)))
    elif c == '|':
        if direction in 'LRU' and row > 0:
            newstates.append(('U', (row-1, col)))
        if direction in 'LRD' and row < maxrow - 1:
            newstates.append(('D', (row+1, col)))
    elif c == '/':
        if direction == 'R' and row > 0:
            newstates.append(('U', (row-1, col)))
        elif direction == 'L' and row < maxrow - 1:
            newstates.append(('D', (row+1, col)))
        elif direction == 'D' and col > 0:
            newstates.append(('L', (row, col-1)))
        elif direction == 'U' and col < maxcol - 1:
            newstates.append(('R', (row, col+1)))
    elif c == '\\':
        if direction == 'L' and row > 0:
            newstates.append(('U', (row-1, col)))
        elif direction == 'R' and row < maxrow - 1:
            newstates.append(('D', (row+1, col)))
        elif direction == 'U' and col > 0:
            newstates.append(('L', (row, col-1)))
        elif direction == 'D' and col < maxcol - 1:
            newstates.append(('R', (row, col+1)))
    elif c == '.':
        if direction == 'U' and row > 0:
            newstates.append(('U', (row-1, col)))
        elif direction == 'D' and row < maxrow - 1:
            newstates.append(('D', (row+1, col)))
        elif direction == 'L' and col > 0:
            newstates.append(('L', (row, col-1)))
        elif direction == 'R' and col < maxcol - 1:
            newstates.append(('R', (row, col+1)))
    return newstates

def propagate(pattern, state0):
    seen = set()
    states = [state0]
    while states:
        state = states.pop()
        seen.add(state)
        newstates = update(pattern, state)
        newstates = [newstate for newstate in newstates if newstate not in seen]
        states.extend(newstates)
    return seen

def energized(seen):
    s = set(state[1] for state in seen)
    return len(s)

def search_edges(pattern):
    maxrow, maxcol = len(pattern), len(pattern[0])
    states = ([('R', (row, 0)) for row in range(maxrow)]
              + [('L', (row, maxcol-1)) for row in range(maxrow)]
              + [('D', (0, col)) for col in range(maxcol)]
              + [('U', (maxrow-1, col)) for col in range(maxcol)])
    return max(energized(propagate(pattern, state)) for state in states)

with open('input.txt') as f:
    data = [[c for c in line.strip()] for line in f.readlines()]
print(search_edges(data))
