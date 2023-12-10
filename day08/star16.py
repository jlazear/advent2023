from dataclasses import dataclass
from collections import defaultdict
from math import lcm

class Node:
    def __init__(self, name=None, left=None, right=None):
        self.name = name
        self.left = left
        self.right = right
        
    def __str__(self):
        left = self.left.name if self.left else None
        right = self.right.name if self.right else None
        return f"Node({self.name}, L={left}, R={right})"
    
def find_next(node, instructions, n0=0):
    n = n0
    first = True
    while first or not node.name.endswith('Z'):
        first = False
        instruction = instructions[n % len(instructions)]
        node = node.left if (instruction == 'L') else node.right
        n += 1
    return node, n

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

Ns = []
for nodename in nodes:
    if nodename.endswith('A'):
        node = nodes[nodename]
        nextnode, n = find_next(node, instructions)
        Ns.append(n//len(instructions))
        nextnextnode, n2 = find_next(nextnode, instructions)
        print(f"{nodename} --({n} = {n/len(instructions)}N)--> {nextnode.name} --({n2} = {n2/len(instructions)}N)--> {nextnextnode.name}")

# xyA -> abZ -> abZ in all cases, with same distances for each
# so just need lcm of distances for the A->Z's
print(lcm(*Ns) * len(instructions))