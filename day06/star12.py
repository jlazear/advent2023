import numpy as np

with open("input.txt") as f:
    times, distances = f.readlines()
T = int(''.join(times.split(':')[1].split()))
D = int(''.join(distances.split(':')[1].split()))

def solutions(T, D):
    D = D + 1.e-6  # need to make sure we exceed the distance, not match it
    plus = (T + np.sqrt(T**2 - 4*D))/2
    minus = (T - np.sqrt(T**2 - 4*D))/2
    return plus, minus

def n_solutions(plus, minus):
    left = int(np.ceil(minus))
    right = int(np.floor(plus))
    return right - left + 1

print(n_solutions(*solutions(T, D)))
    