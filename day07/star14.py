from collections import Counter

data = []
with open("input.txt") as f:
    for line in f:
        hand, bid = line.split()
        modhand = hand.translate(str.maketrans("J23456789TQKA", "23456789ABCDE"))
        bid = int(bid)
        c = Counter(hand)
        if 'J' in c and len(c) > 1:
            cj = c.pop('J')
            maxc = c.most_common(1)[0][0]
            c[maxc] += cj
        cardinality = len(c)
        biggest = max(c.values())
        data.append((-cardinality, biggest, modhand, bid))

data = sorted(data)
print(sum((i+1)*d[-1] for i, d in enumerate(data)))
