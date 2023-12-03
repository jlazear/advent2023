def parse_line(line):
    print('-'*10 + '\n' + line)  #DELME
    found = True
    while found:
        found = False
        for i in range(len(line)):
            for j, number in enumerate(['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']):
                if line[i:i+len(number)] == number:
                    line = line[:i] + str(j+1) + line[i+len(number):]
                    # line = line[:i].replace(number, str(j+1), 1)
                    found = True

    print(line)  #DELME
    digits = [c for c in line if c in '0123456789']
    print(digits)
    number = int(digits[0])*10 + int(digits[-1])
    print(number)
    return number

sum = 0
with open('input.txt') as f:
    i = 0  #DELME
    for line in f:
        sum += parse_line(line)
        i += 1
        # if i>10:
        #     break

print(sum)