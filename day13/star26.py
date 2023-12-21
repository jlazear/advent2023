import numpy as np

def convert_pattern(pattern):
    pattern = pattern.split()
    nrows = len(pattern)
    ncols = len(pattern[0])
    mya = np.zeros((nrows, ncols), dtype=int)
    for i, row in enumerate(pattern):
        for j, c in enumerate(row):
            mya[i, j] = 1 if c == '#' else 0
    return mya

def check_horizontal_reflection(pattern, after):
    maxrow = pattern.shape[0]
    maxcol = pattern.shape[1]
    nrows = min(after + 1, maxrow - after - 1)
    top = pattern[after+1-nrows:after+1][::-1]
    bottom = pattern[after+1:after+1+nrows]
    deltas = np.abs(np.sum(top-bottom, axis=0))
    if np.sum(deltas == 0) == maxcol-1 and np.sum(deltas == 1) == 1:
        return 100*(after+1)
    return 0

def check_vertical_reflection(pattern, after):
    maxrow = pattern.shape[0]
    maxcol = pattern.shape[1]
    nrows = min(after + 1, maxcol - after - 1)
    left = pattern[:, after+1-nrows:after+1][:, ::-1]
    right = pattern[:, after+1:after+1+nrows]
    deltas = np.abs(np.sum(left-right, axis=1))
    if np.sum(deltas == 0) == maxrow-1 and np.sum(deltas == 1) == 1:
        return (after+1) 
    return 0

def evaluate_pattern(pattern):
    nrows, ncols = pattern.shape
    for i in range(nrows-1):
        s = check_horizontal_reflection(pattern, i)
        if s != 0:
            return s
    for i in range(ncols-1):
        s = check_vertical_reflection(pattern, i)
        if s != 0:
            return s
    return 0


with open('input.txt') as f:
    patterns = f.read().split('\n\n')
patterns = [convert_pattern(pattern) for pattern in patterns]

s = sum(evaluate_pattern(pattern) for pattern in patterns)
print(s)