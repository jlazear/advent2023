def make_rule(rule_str):
    destination, source, number = map(int, rule_str.split())
    source_end = source + number - 1
    delta = destination - source
    return (source, source_end, delta)

def split(ranges, rules):
    newranges = []
    while ranges:
        left, right = ranges.pop()
        found = False
        for rule in rules:
            rule_left, rule_right, delta = rule
            if rule_right < left:   # 0 
                continue
            elif right < rule_left:  # 1
                found = True
                newranges.append((left, right))
                break
            elif left < rule_left and right <= rule_right:  # 2
                found = True
                ranges.append((left, rule_left-1))
                newranges.append((rule_left + delta, right + delta))
                break
            elif rule_left <= left and rule_right < right:  # 3
                found = True
                ranges.append((rule_right+1, right))
                newranges.append((left + delta, rule_right + delta))
                break
            elif rule_left <= left and right <= rule_right:  # 4
                found = True
                newranges.append((left+delta, right+delta))
                break
            elif left < rule_left and rule_right < right:  # 5
                found = True
                ranges.append((rule_right+1, right))
                ranges.append((left, rule_left-1))
                newranges.append((rule_left + delta, rule_right + delta))
                break
            else:
                raise Exception("Shouldn't get here!")
        if not found:
            newranges.append((left, right))
    return sorted(newranges)

with open('input.txt') as f:
    s = f.read()
    
slist = s.split('\n\n')
seeds = list(map(int, slist[0].split(':')[1].split()))
rules_str = slist[1:]

rules = []
for all_rules in rules_str:
    rule = []
    rules.append(rule)
    for rule_str in all_rules.split('\n')[1:]:
        source, source_end, delta = make_rule(rule_str)
        rule.append((source, source_end, delta))
    rule.sort()

ranges = []
for i in range(len(seeds)//2):
    ranges.append((seeds[2*i], sum(seeds[2*i:2*i+2])))
ranges = sorted(ranges)
ranges.reverse()

for rule in rules:
    ranges = split(ranges, rule)

print(f"{min(ranges)[0]}")
