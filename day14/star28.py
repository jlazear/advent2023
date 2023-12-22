import numpy as np

with open('input.txt') as f:
    data = np.array([[c for c in line.strip()] for line in f.readlines()])

maxrow, maxcol = data.shape
score = np.zeros(data.shape)
for i in range(maxrow):
    score[i, :] = maxrow - i

def reconstruct(h):
    return np.frombuffer(h, dtype='<U1').reshape((data.shape))

def print_pattern(pattern, prefix=''):
    print(f"{prefix}" + f'\n{prefix}'.join(''.join(c for c in row) for row in pattern))

def score_pattern(pattern, score=score, c='O'):
    return int(np.sum(score[pattern == c]))

def roll(pattern, direction='up'):
    if direction == 'left':
        pattern = np.rot90(pattern, k=-1)
    elif direction == 'down':
        pattern = np.rot90(pattern, k=-2)
    elif direction == 'right':
        pattern = np.rot90(pattern, k=-3)
    newpattern = pattern.copy()
    newpattern[newpattern == 'O'] = '.'
    maxrow, maxcol = pattern.shape

    for col in range(maxcol):
        depth = 0
        start = 0
        for i, row in enumerate(range(maxrow)):
            d = pattern[row, col]
            if d == 'O':
                depth += 1
            elif d == '#':
                newpattern[start:start+depth, col] = 'O'
                start = i+1
                depth = 0
        if depth > 0:
            newpattern[start:start+depth, col] = 'O'

    if direction == 'left':
        newpattern = np.rot90(newpattern, k=1)
    elif direction == 'down':
        newpattern = np.rot90(newpattern, k=2)
    elif direction == 'right':
        newpattern = np.rot90(newpattern, k=3)
    return newpattern
    
def cycle_one(pattern):
    pattern = roll(pattern, direction='up')
    pattern = roll(pattern, direction='left')
    pattern = roll(pattern, direction='down')
    pattern = roll(pattern, direction='right')
    return pattern

def cycle(pattern, n):
    seen = {}
    for i in range(n):
        h = pattern.tobytes()
        if h in seen:
            break
        seen[h] = i
        pattern = cycle_one(pattern)
    period = i - seen[h]
    L = (n - seen[h]) % period
    nequiv = seen[h] + L
    return reconstruct([key for key, value in seen.items() if value == nequiv][0])
    
n = 1000000000
data = cycle(data, n)
print(score_pattern(data))
