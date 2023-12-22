from functools import cache

@cache
def HASH(s):
    current = 0
    for c in s:
        current += ord(c)
        current = (current*17) % 256
    return current

def minus(boxes, label):
    i = HASH(label)
    box = boxes[i]
    todel = None
    for j, pair in enumerate(box):
        if pair[0] == label:
            todel = j
    if todel is not None:
        del box[todel]

def equals(boxes, label, n):
    i = HASH(label)
    box = boxes[i]
    found = False
    for j, pair in enumerate(box):
        if pair[0] == label:
            pair[1] = n
            found = True
    if not found:
        box.append([label, n])

def str_pair(pair):
    return f"[{pair[0]} {pair[1]}]"

def print_boxes(boxes):
    for i, box in enumerate(boxes):
        if box:
            boxstr = ' '.join(str_pair(pair) for pair in box)
            
def score(boxes):
    s = 0
    for n_box, box in enumerate(boxes):
        for n_lens, (_, f_len) in enumerate(box):
            s += (n_box+1) * (n_lens+1) * int(f_len)
    return s

def execute(boxes, step):
    if '-' in step:
        label = step.rstrip('-')
        minus(boxes, label)
    elif '=' in step:
        label, n = step.split('=')
        equals(boxes, label, n)
    

with open('input.txt') as f:
    steps = f.read().split(',')

boxes = [[] for _ in range(256)]
    
for step in steps:
    execute(boxes, step)

print(score(boxes))
