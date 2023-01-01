import numpy as np

class PriorityQueue:
    def __init__(self):
        self._items = []
        
    def queue(self, item, prio):
        self._items.append((prio, item))
        self._items = sorted(self._items, reverse=True)
        
    def dequeue(self):
        prio, item = self._items.pop()
        return item
    
    def is_empty(self):
        return len(self._items) == 0

def neighbors(y, x, val):
    res = []
    if x > 0 and grid[y, x-1] <= val+1:
        res.append((y, x-1))
    if x < grid.shape[1]-1 and grid[y, x+1] <= val+1:
        res.append((y, x+1))
    if y > 0 and grid[y-1, x] <= val+1:
        res.append((y-1, x))
    if y < grid.shape[0]-1 and grid[y+1, x] <= val+1:
        res.append((y+1, x))
    return res

def transf(c):
    if c == 'S':
        return 0
    if c == 'E':
        return 25
    return ord(c) - 97

if __name__ == '__main__':
    with open("input", "r") as f:
        content = f.read()
    grid = []
    for i, r in enumerate(content.split('\n')):
        if 'S' in r:
            start = (i, r.index('S'))
        if 'E' in r:
            end = (i, r.index('E'))
        if r.strip() != '':
            grid.append([transf(c) for c in list(r)])

    grid = np.array(grid)
    
    frontier = PriorityQueue()
    frontier.queue(start, 0)
    path_chain = dict()
    steps = dict()
    path_chain[start] = None
    steps[start] = 0

    while not frontier.is_empty():
        curr_pos = frontier.dequeue()

        if curr_pos[0] == end[0] and curr_pos[1] == end[1]:
            break

        curr_val = grid[curr_pos[0], curr_pos[1]]
        for nxt in neighbors(curr_pos[0], curr_pos[1], curr_val):
            new_steps = steps[curr_pos] + 1
            if nxt not in steps or new_steps < steps[nxt]:
                steps[nxt] = new_steps
                prio = new_steps + abs(curr_pos[0] - end[0]) + abs(curr_pos[1] - end[1])
                frontier.queue(nxt, prio)
                path_chain[nxt] = curr_pos

    curr_pos = end
    path = []
    while curr_pos != start:
        path.append(curr_pos)
        curr_pos = path_chain[curr_pos]

    print(len(path))
