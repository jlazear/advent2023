digits = '0123456789'

def get_neighbors(row, col, m):
    left = max(0, col - 1)
    right = min(len(m[0])-1, col+1)
    top = max(0, row - 1)
    bottom = min(len(m)-1, row+1)

    return [(y, x) for x in range(left, right+1) for y in range(top, bottom+1) if x != col or y != row]

def turkey(row, col, m):
    if m[row][col] not in digits:
        return 0
    left = col
    right = col
    while(left > 0 and m[row][left-1] in digits):
        left -= 1
    while (right < len(m[0])-1 and m[row][right+1] in digits):
        right += 1
    return int(m[row][left:right+1]), hash((left, right, row))

def get_power(star, m):
    numbers = set()
    for y, x in get_neighbors(*star, m):
        if m[y][x] in digits:
            numbers.add(turkey(y, x, m))
    if len(numbers) == 2:
        return numbers.pop()[0]*numbers.pop()[0]
    return 0

with open("input.txt") as f:
    mstr = f.read()
m = mstr.split('\n')

stars = []
for row, line in enumerate(m):
    for col, c in enumerate(line):
        if c == '*':
            stars.append((row, col))

print(sum(get_power(star, m) for star in stars))
