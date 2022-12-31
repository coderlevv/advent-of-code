import numpy as np
from copy import deepcopy
import math


class Pos:
    """Represents a y, x coordinate on the grid.

    Rows are indexed by y, columns by x.
    """
    def __init__(self, y, x):
        self.y = y
        self.x = x
        
    def __eq__(self, other):
        return self.y == other.y and self.x == other.x
        
    def __repr__(self):
        return f"Pos({self.y},{self.x})"


class Blizz:
    """Represents a blizzard.
    
    A Blizzard has a y, x position and a direction (d).
    """
    def __init__(self, y, x, d):
        self.pos = Pos(y, x)
        self.d = d
        
    def __repr__(self):
        return f"{self.d}:{self.pos}"


def make_grid(blizz, start, goal, y_dim, x_dim):
    """Constructs a 2-dim grid of the valley and blizzards.
    
    If a grid position equals 0, it is empty. A number > 0
    represents the count of blizzards occupying this position.
    A wall is indicated by -1.
    """
    grid = []
    for y in range(y_dim):
        row = []
        for x in range(x_dim):
            # the valley is surrounded by walls
            if x == 0 or x == x_dim-1 or y == 0 or y == y_dim-1:
                row.append(-1)
            else:
                count = 0
                for b in blizz:
                    if b.pos.y == y and b.pos.x == x:
                        count += 1
                row.append(count)
        grid.append(row)
    # open the wall for start and goal position
    grid[goal.y][goal.x] = 0
    grid[start.y][start.x] = 0
    return grid


def display(grid, blizz, goal):
    """Display a valley grid."""

    for y, r in enumerate(grid):
        row = []
        for x, c in enumerate(r):
            if y == goal.y and x == goal.x:
                row.append('.')
            elif c < 0:
                row.append('#')
            else:
                blizz_pos = []
                for b in blizz:
                    if b.pos.y == y and b.pos.x == x:
                        blizz_pos.append(b)
                if len(blizz_pos) > 0:
                    if len(blizz_pos) == 1:
                        row.append(blizz_pos[0].d)
                    else:
                        row.append(str(len(blizz_pos)))
                else:
                    row.append('.')
        print(''.join(row))


def next_pos(grid, pos):
    """Find open positions to walk to next."""
    res = []
    y = pos.y
    x = pos.x
    if y-1 >= 0 and grid[y-1][x] == 0: res.append(Pos(y-1, x))
    if x+1 < len(grid[0]) and grid[y][x+1] == 0: res.append(Pos(y, x+1))
    if y+1 < len(grid) and grid[y+1][x] == 0: res.append(Pos(y+1, x))
    if x-1 > 0 and grid[y][x-1] == 0: res.append(Pos(y, x-1))
    return res

class PriorityQueue:
    """Priority queue used to find shortes path."""

    def __init__(self):
        self._items = []
        
    def queue(self, item, prio):
        self._items.append((item, prio))
        self._items = sorted(self._items, reverse=True, key=lambda x: x[1])
        
    def dequeue(self):
        return self._items.pop()
    
    @property
    def is_empty(self):
        return len(self._items) == 0


def blizz_pos(blizz, t, y_dim, x_dim):
    """Compute blizzard positions.
    
    Given an initial position and a time t, the function computes the
    position of blizzards at time t."""

    # need a deepcopy of the blizz array
    new_blizz = deepcopy(blizz)
    for b in new_blizz:
        if b.d == '>':
            b.pos.x = (b.pos.x - 1 + t) % (x_dim - 2) + 1
        if b.d == '<':
            b.pos.x = (b.pos.x - 1 - t) % (x_dim - 2) + 1
        if b.d in 'v':
            b.pos.y = (b.pos.y - 1 + t) % (y_dim - 2) + 1
        if b.d in '^':
            b.pos.y = (b.pos.y - 1 - t) % (y_dim - 2) + 1
    return new_blizz



def walk(start, goal, lcm, states, minute):
    """Finds the shortest path from start to goal.
    
    Parameter
    ---------
    start : Position
        Where to start.
    goal : Position
        Where to walk to.
    lcm : integer
        Lowest common multiple of the y- and x-dim of
        the (inner) grid, i.e. without the walls. Blizzard
        positions repeat after lcm time steps.
    states : array of grids
        Precomputed blizzard positions.
    minute : integer
        Start time.

    Result
    ------
    integer
        Minute when the goal was reached, None otherwise.
    """

    frontier = PriorityQueue()
    time_elapsed = dict()
    state_idx = minute % lcm
    time_elapsed[(start.y, start.x, state_idx)] = minute
    frontier.queue(start, minute)
    while not frontier.is_empty:
        curr, minute = frontier.dequeue()
        if curr == goal:
            return minute
        
        # look ahead
        next_minute = minute + 1
        state_idx = (next_minute) % lcm
        next_grid = states[state_idx]
        collision = next_grid[curr.y][curr.x] > 0

        # possible next moves
        nxt = next_pos(next_grid, curr)
        if not collision:
            nxt.append(Pos(curr.y, curr.x))
        for n in nxt:
            if next_minute < time_elapsed.setdefault((n.y, n.x, state_idx), math.inf):
                frontier.queue(n, next_minute)
                time_elapsed[(n.y, n.x, state_idx)] = next_minute


if __name__ == '__main__':
    with open("input", "r") as f:
        content = f.read()

    # initialize blizzard positions
    # and read grid to initialize other variables
    blizz = []
    grid_init = []
    for y, r in enumerate(content.split("\n")):
        if r.strip() == "":
            continue
        grid_init.append(list(r))
        for x, c in enumerate(list(r)):
            if c in ['^', '>', '<', 'v']:
                blizz.append(Blizz(y, x, c))
    goal = Pos(len(grid_init)-1, grid_init[len(grid_init)-1].index('.'))
    start = Pos(0, grid_init[0].index('.'))
    y_dim = len(grid_init)
    x_dim = len(grid_init[0])
    lcm = np.lcm(y_dim-2, x_dim-2) # lowest common multiple
    # blizzard positions repeat after lcm time steps. Precomputing
    # all possible grid states considerably speeds up path finding! 
    states = []
    for t in range(lcm):
        new_blizz = blizz_pos(blizz, t, y_dim, x_dim)
        states.append(make_grid(new_blizz, start, goal, y_dim, x_dim))
        grid = make_grid(blizz, start, goal, y_dim, x_dim)

    # part 1
    minute = walk(start, goal, lcm, states, 0)
    print(minute)

    # part 2
    minute = walk(start, goal, lcm, states, 0)
    minute = walk(goal, start, lcm, states, minute)
    minute = walk(start, goal, lcm, states, minute)
    print(minute)