def nearest_above(x0, xs):
    for i, x in enumerate(xs):
        if x > x0:
            return i
    return len(xs)

def nearest_below(x0, xs):
    for i in range(len(xs)):
        n = len(xs) - 1 - i
        x = xs[n]
        if x < x0:
            return n
    return -1

def expanded_L1_norm(galaxy0, galaxy1, emptyrows, emptycols):
    x0, y0 = galaxy0
    x1, y1 = galaxy1
    if x1 < x0:
        x0, x1 = x1, x0
    if y1 < y0:
        y0, y1 = y1, y0
    nx = nearest_below(x1, emptyrows) - nearest_above(x0, emptyrows) + 1
    ny = nearest_below(y1, emptycols) - nearest_above(y0, emptycols) + 1
    return (x1 - x0 + nx) + (y1 - y0 + ny)

def distance_between(index1, index2, galaxies, emptyrows, emptycols):
    galaxy1 = galaxies[index1-1]
    galaxy2 = galaxies[index2-1]
    return expanded_L1_norm(galaxy1, galaxy2, emptyrows, emptycols)

galaxies = []
rows = set()
cols = set()
maxrow = 0
maxcol = 0
with open("input.txt") as f:
    for row, line in enumerate(f):
        maxrow = row
        for col, c in enumerate(line):
            maxcol = col
            if c == '#':
                galaxies.append((row, col))
                rows.add(row)
                cols.add(col)

emptyrows = sorted(list(set(range(maxrow+1)) - rows))
emptycols = sorted(list(set(range(maxcol+1)) - cols))

s = 0
for i, galaxy1 in enumerate(galaxies):
    for j, galaxy2 in enumerate(galaxies[i+1:]):
        distance = expanded_L1_norm(galaxy1, galaxy2, emptyrows, emptycols)
        s += distance
print(s)
