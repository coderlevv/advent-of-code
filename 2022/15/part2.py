import numpy as np
import re

with open('input', 'r') as f:
    content = f.read()

sensors = []
beacons = []
for c in content.split('\n'):
    if c.strip() != '':
        m = re.findall(r'Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)', c)
        sx, sy, bx, by = m[0]
        sensors.append((int(sy), int(sx)))
        beacons.append((int(by), int(bx)))

# In part 1 it was possible to work with distinct coordinates
# Due to the vast parameter space in part 2 I had to switch to
# an algorithm using position ranges and lists of ranges
# thanks to Erik van Oosten for the idea
# https://github.com/erikvanoosten/advent-of-code
class PosRange:
    """Represents a range of position x-coordinates.

    Attributes
    ----------
    left : integer
        Left border of range (inclusive)
    right : integer
        Right border of range (inclusive)
    """

    def __init__(self, left, right):
        self.left = left
        self.right = right
    
    @property
    def not_empty(self):
        return self.left <= self.right
    
    @property
    def is_empty(self):
        return not self.not_empty
    
    def __len__(self):
        return self.right - self.left + 1 if self.not_empty else 0
    
    def __sub__(self, other):
        if self.right < other.left or self.left > other.right:
            return self
        else:
            res = [PosRange(self.left, other.left-1), PosRange(other.right+1, self.right)]
            return [r for r in res if r.not_empty]
    
    def __repr__(self):
        return f"[{self.left},{self.right}]"


class PosRangeList:
    def __init__(self, pos_list):
        self.pos_list = pos_list
        
    def __len__(self):
        return sum([len(p) for p in self.pos_list])
    
    def not_empty(self):
        return len(self.pos_list) > 0
    
    def __sub__(self, other):
        # ugly implementation
        # TODO: do a more pythonic implementation
        acc = self
        for i in range(len(other.pos_list)):
            o = other.pos_list[i]
            tmp = []
            for j in range(len(acc.pos_list)):
                r = acc.pos_list[j]
                if r != [] and (r - o) != []:
                    if type(r - o) != list:
                        tmp += [(r - o)]
                    else:
                        tmp += (r - o)
            acc = PosRangeList([r for r in tmp if len(r) > 0])
        return acc
        
    def __repr__(self):
        return f"{self.pos_list}"


def xcoverage(i, y):
    """Sensor i x-coordinate coverage of positions with y-coordinate=y."""

    dist_sb = abs(sensors[i][0] - beacons[i][0]) + abs(sensors[i][1] - beacons[i][1])
    dist_y = abs(sensors[i][0] - y)
    cover = dist_sb - dist_y
    if cover < 1:
        return PosRangeList([])
    else:
        return PosRangeList([PosRange(sensors[i][1]-cover, sensors[i][1]+cover)])


for y in range(0,4000001):
    xrange = PosRangeList([PosRange(0, 4000000)])
    for i in range(len(sensors)):
        xrange = xrange - xcoverage(i, y)
    if len(xrange.pos_list) > 0:
        print(xrange.pos_list[0].left * 4000000 + y)
        break

