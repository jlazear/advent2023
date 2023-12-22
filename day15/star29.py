with open('input.txt') as f:
    steps = f.read().split(',')

def HASH(s):
    current = 0
    for c in s:
        current += ord(c)
        current = (current*17) % 256
    return current
    
print(sum(HASH(step) for step in steps))
# for step in steps:
#     print(f"{step} {HASH(step)}")