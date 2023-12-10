from dataclasses import dataclass
from collections import defaultdict

@dataclass
class Node:
    name: str = None
    left: str = None
    right: str = None

nodes = defaultdict(Node)

with open("input.txt") as f:
    instructions = f.readline().strip()
    f.readline()
    for line in f.readlines():
        name, rhs = line.split(' = ')
        name = name.strip()
        rhs = rhs.strip().strip('()')
        left, right = rhs.split(',')
        left = left.strip()
        right = right.strip()

        node = nodes[name]
        node.name = name
        node.left = nodes[left]
        node.right = nodes[right]

node = nodes['AAA']
n = 0
while node.name != 'ZZZ':
    instruction = instructions[n % len(instructions)]
    node = node.left if (instruction == 'L') else node.right
    n += 1
        
print(n)
