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

yref = 2000000
pos = set()
for i in range(len(sensors)):
    # distance from sensor to it's nearest beacon
    dist_sb = abs(sensors[i][0] - beacons[i][0]) + abs(sensors[i][1] - beacons[i][1])
    # distance from sensor to the y reference coordinate
    dist_sy = abs(sensors[i][0] - yref)
    # check if sensor covers y reference coordinate and
    # remember the positions covered
    if dist_sb >= dist_sy:
        for k in range(dist_sb - dist_sy + 1):
            pos.add((yref, sensors[i][1] + k))
            pos.add((yref, sensors[i][1] - k))
# if there are any beacons with y eq. reference y, remove
# the position
for b in set(beacons):
    if b in pos:
         pos.remove(b)

print(len(pos))
