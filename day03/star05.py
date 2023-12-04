numbers = []
digits = '0123456789'

def parse_line(line, row, m):
    n = 0
    locs = []
    for col, c in enumerate(line):
        if c in digits:
            n = 10*n + int(c)
            locs.append((row, col))
        else:
            if n != 0:
                numbers.append((n, get_neighbors(locs, m)))
                n = 0
                locs = []
    if n != 0:
        numbers.append((n, get_neighbors(locs, m)))

def get_neighbors(locs, m):
    xs = [loc[1] for loc in locs]
    y = locs[0][0]
    xmin = min(xs)
    xmax = max(xs)
    mylen = len(m)
    mxlen = len(m[0])

    neighbors = []
    if xmin-1 >= 0:
        neighbors.append((y, xmin-1))
    if xmax + 1 < mxlen:
        neighbors.append((y, xmax+1))
    if y+1 < mylen:
        neighbors.extend([(y+1, x) for x in range(max(0, xmin-1), min(xmax+2, mxlen))])
    if y-1 >= 0:
        neighbors.extend([(y-1, x) for x in range(max(0, xmin-1), min(xmax+2, mxlen))])
    return neighbors

def print_neighbors(neighbors, m):
    xs = [neighbor[1] for neighbor in neighbors]
    ys = [neighbor[0] for neighbor in neighbors]
    xmin, xmax = min(xs), max(xs)
    ymin, ymax = min(ys), max(ys)
    s = []
    for y in range(ymin, ymax+1):
        sy = []
        for x in range(xmin, xmax+1):
            sy.append(m[y][x])
        s.append(''.join(sy))
    s = '\n'.join(s)
    print(s)

def is_valid(neighbors, m):
    for x, y in neighbors:
        try:
            c = m[x][y]
            if c != '.' and not c in digits:
                return True
        except IndexError:
            continue
    return False

with open("input.txt") as f:
    m = f.read()
m = m.split('\n')
for row, line in enumerate(m):
    parse_line(line, row, m)
print(sum([number for number, neighbors in numbers if is_valid(neighbors, m)]))
