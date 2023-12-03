def parse_line(line):
    digits = [c for c in line if c in '0123456789']
    return int(digits[0])*10 + int(digits[-1])

sum = 0
with open('input.txt') as f:
    for line in f:
        sum += parse_line(line)

print(sum)