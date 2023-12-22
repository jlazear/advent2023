from functools import cache
import numpy as np

@cache
def weight(d, n):
    return sum(n-i for i in range(d))
        
with open('input.txt') as f:
    data = np.array([[c for c in line.strip()] for line in f.readlines()])

maxrow, maxcol = data.shape

depths = []
for col in range(maxcol):
    depth = 0
    start = 0
    for i, row in enumerate(range(maxrow)):
        d = data[row, col]
        if d == 'O':
            depth += 1
        elif d == '#':
            depths.append((start, depth))
            start = i+1
            depth = 0
    if depth > 0:
        depths.append((start, depth))
    
print(sum(weight(d, maxrow-n) for (n, d) in depths))