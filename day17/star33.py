from collections import defaultdict

with open('input.txt') as f:
    m = [[int(c) for c in row.strip()] for row in f.readlines()]
valids = [(row, col) for row in range(len(m)) for col in range(len(m[0]))]

def next(m, state, score):
    (row, col), dir, streak, subscore = state
    nextstates = []
    if streak < 2:
        if dir == 'U':
            nextpos = (row-1, col)
        elif dir == 'D':
            nextpos = (row+1, col)
        elif dir == 'L':
            nextpos = (row, col-1)
        elif dir == 'R':
            nextpos = (row, col+1)
        if nextpos in valids:
            delta = m[nextpos[0]][nextpos[1]]
            nextstates.append((nextpos, dir, streak+1, subscore + delta))
    if dir == 'U':
        nextposL = (row, col-1)
        nextposR = (row, col+1)
        dirL = 'L'
        dirR = 'R'
    elif dir == 'D':
        nextposL = (row, col+1)
        nextposR = (row, col-1)
        dirL = 'R'
        dirR = 'L'
    elif dir == 'R':
        nextposL = (row-1, col)
        nextposR = (row+1, col)
        dirL = 'U'
        dirR = 'D'
    elif dir == 'L':
        nextposL = (row+1, col)
        nextposR = (row-1, col)
        dirL = 'D'
        dirR = 'U'
    if nextposL in valids:
        deltaL = m[nextposL[0]][nextposL[1]]
        nextstates.append((nextposL, dirL, 0, subscore + deltaL))
    if nextposR in valids:
        deltaR = m[nextposR[0]][nextposR[1]]
        nextstates.append((nextposR, dirR, 0, subscore + deltaR))
    nextstates = [nextstate for nextstate in nextstates if nextstate[-1] < score]
    nextstates = sorted(nextstates, key=lambda x: (sum(x[0]), x[-1]))
    # nextstates = sorted(nextstates, key=lambda x: x[-1])
    return nextstates

def substate(state):
    return (state[0], state[2])

def dfs(m):
    maxrow, maxcol = len(m), len(m[0])
    states = [((1, 0), 'D', 1, m[1][0]), ((0, 1), 'R', 1, m[0][1])]
    # paths = [[(0, 0)],[(0, 0)]]  #DELME
    score = 1560 # sum([sum(row) for row in m]) * 5
    # bestpath = None
    seen = defaultdict(lambda: score)
    i = 0  #DELME
    while states:
        state = states.pop()
        if i % 1000 == 0:   #DELME
            print(f"{i = } {len(states) = } {len(seen) = } {sum(seen.values()) = } {score = } {state[0] = } {state[-1] = }")  #DELME
        i += 1  #DELME
        if seen[substate(state)] < state[-1]:
            continue
        # path = paths.pop()  #DELME
        # print(f"{len(states) = } {score = } {len(path) = } {len(paths) = }")  #DELME
        # print(f"{path = }")  #DELME
        # if len(path) > 4:
        #     break
        # substate = state[:3]
        # seen[state[:-1]] = min(state[-1], seen[state[:-1]])
        seen[substate(state)] = min(state[-1], seen[substate(state)])
        # path.append(state[0])  #DELME
        # print(f"{state = }")  #DELME
        if state[0] == (maxrow-1, maxcol-1):
            # if state[-1] < score:
            #     bestpath = path
            #     score = state[-1]
            score = min(score, state[-1])
            continue
        nextstates = next(m, state, score)
        for nextstate in nextstates:
            # if nextstate[:-1] not in seen or nextstate[-1] < seen[nextstate[:-1]]:
            if substate(nextstate) not in seen or nextstate[-1] < seen[substate(nextstate)]:
                states.append(nextstate)
        # nextstates = [nextstate for nextstate in nextstates if nextstate not in seen]
        # print(f"{nextstates = }\n")  #DELME
        # states.extend(nextstates)
        # paths.extend([path.copy() for _ in nextstates])  #DELME
        # states = sorted(states, key=lambda x: (sum(x[0]), x[-1]))
    return score# , bestpath

score = dfs(m)
# score, bestpath = dfs(m)
print(f"{score = }")
# print(f"{len(bestpath) = }")
# print(f"{bestpath = }")