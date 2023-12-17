from functools import cache

@cache
def delta(N):
    if N == 0:
        return [1]
    prevdelta = delta(N-1) + [0]
    return [prevdelta[i] - prevdelta[i-1]  for i in range(N+1)]

def next_value(xs, N):
    d = delta(N)
    if len(xs) < len(d) - 1:
        raise Exception(f"xs ({len(xs)}) must be at least N ({N}) elements")
    xs_N = xs[-len(d) + 1:]
    ds_N = d[1:][::-1]
    next_value = sum(-ds_N[i] * xs_N[i] for i in range(len(d)-1))
    return next_value
    
def check_order(xs, N):
    xs0 = xs[:N]
    while len(xs0) < len(xs):
        xs0.append(next_value(xs0, N))
        if xs0[-1] != xs[len(xs0)-1]:
            return False
    return True

def solve(xs):
    N = 1
    while not check_order(xs, N):
        N += 1
    return next_value(xs, N)

s = 0
with open("input.txt") as f:
    for line in f:
        xs = list(map(int, line.split()))
        s += solve(xs)

print(s)