def make_rule(rule_str):
    destination, source, number = map(int, rule_str.split())
    source_end = source + number
    f = lambda i: destination + (i - source)
    return (source, source_end, f)

def dispatch(x, rule):
    for (source, end), f in rule.items():
        if source <= x <= end:
            return f(x)
    return x

with open('input.txt') as f:
    s = f.read()
    
slist = s.split('\n\n')
seeds = map(int, slist[0].split(':')[1].split())
rules_str = slist[1:]

rules = []
for all_rules in rules_str:  #FIXME
    rule = {}
    rules.append(rule)
    for rule_str in all_rules.split('\n')[1:]:
        source, source_end, f = make_rule(rule_str)
        rule[(source, source_end)] = f

locations = []
for seed in seeds:
    for rule in rules:
        seed = dispatch(seed, rule)
    locations.append(seed)
print(f"{min(locations)}")
