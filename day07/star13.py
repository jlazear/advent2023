from collections import Counter

data = []
with open("input.txt") as f:
    for line in f:
        hand, bid = line.split()
        hand = hand.translate(str.maketrans("23456789TJQKA", "23456789ABCDE"))
        bid = int(bid)
        c = Counter(hand)
        cardinality = len(c)
        biggest = max(c.values())
        data.append((-cardinality, biggest, hand, bid))

data = sorted(data)
print(sum((i+1)*d[-1] for i, d in enumerate(data)))
