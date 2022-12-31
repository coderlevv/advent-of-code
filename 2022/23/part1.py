import numpy as np
from collections import Counter


class Elf:
    """Elf with y,x position in the crater."""
    def __init__(self, y, x):
        self.y = y
        self.x = x
        # proposed coordinates of next move
        self.prop = None
    
    def __repr__(self):
        return f"<{self.y},{self.x},{self.prop}>"


class Crater:
    """Represents the crater with elves, implements elf movement."""

    def __init__(self, elves):
        self.elves = list(elves)
        # dir_idx cycles through dirs and determines
        # order of dir proposition
        self.dirs = ['N', 'S', 'W', 'E']
        self.dir_idx = 0
        self.prop_counter = None
        self.valid_props = None
        self.shift_zero()
        self.grid = self.make_grid()
        self.round_count = 0
    

    def coords_range(self):
        """Determine range of y- x-coordinates of elves."""
        min_y = None
        max_y = None
        min_x = None
        max_x = None
        for e in self.elves:
            if min_y is None:
                min_y = e.y
            elif e.y < min_y:
                min_y = e.y
            
            if max_y is None:
                max_y = e.y
            elif e.y > max_y:
                max_y = e.y
                
            if min_x is None:
                min_x = e.x
            elif e.x < min_x:
                min_x = e.x
                
            if max_x is None:
                max_x = e.x
            elif e.x > max_x:
                max_x = e.x
        
        return min_y, max_y, min_x, max_x

    
    def shift_zero(self):
        """Shift elf coordinates.

        The coordinate ranges are used to shift y- and x-
        coordinates of the elves to zero.
        """

        min_y, max_y, min_x, max_x = self.coords_range()
        for e in self.elves:
            e.y -= min_y
            e.x -= min_x
        
        # update coords
        min_y, max_y, min_x, max_x = self.coords_range()
        self.min_y = min_y
        self.max_y = max_y
        self.min_x = min_x
        self.max_x = max_x
    
    
    def make_grid(self):
        """Generate a 2-dim array with elf positions."""
        grid = []
        for _ in range(self.min_y, self.max_y+1):
            row = []
            for _ in range(self.min_x, self.max_x+1):
                row.append('.')
            grid.append(row)
        for e in self.elves:
            grid[e.y][e.x] = '#'
        return grid

    
    def check_grid(self, e, off, prop_idx=1):
        """Check for possible next positions.
        
        Parameter
        ---------
        e : Elf
            Check position for this specific elf.

        off : array of integer
            Offset for the four different directions an elf can move.

        prop_idx : integer
           indexes into the offset array.  
        """

        prop = []
        # Counter of clear positions
        clear = 0
        # check each offset
        for o in off:
            if e.y + o[0] < 0 or e.y + o[0] > self.max_y:
                clear += 1
            elif e.x + o[1] < 0 or e.x + o[1] > self.max_x:
                clear += 1
            elif self.grid[e.y+o[0]][e.x+o[1]] == '.':
                clear += 1
        
        if clear == len(off):
            # all offset positions clear
            # propose offset position prop_idx
            prop.append((e.y + off[prop_idx][0], e.x + off[prop_idx][1]))
            
        return prop
    

    def propose(self):
        """Make a proposal for next elf movements."""

        # Proposals are made in the order the directions
        # appear in this array.
        dirs = []
        idx = self.dir_idx
        for i in range(idx, idx + 4):
            dirs.append(self.dirs[i % 4])
        
        for e in self.elves:
            prop = []
            for d in dirs:
                if d == 'N':
                    prop += self.check_grid(e, [(-1, -1), (-1, 0), (-1,1)])
                if d == 'S':
                    prop += self.check_grid(e, [(1, -1), (1, 0), (1,1)])
                if d == 'W':
                    prop += self.check_grid(e, [(-1, -1), (0, -1), (1,-1)])
                if d == 'E':
                    prop += self.check_grid(e, [(-1, 1), (0, 1), (1, 1)])
            if len(prop) == 4:
                # all adjacent pos free, don't move
                e.prop = None
            else:
                # not all adj pos free, take first prop
                if len(prop) > 0:
                    e.prop = prop[0]
                else:
                    e.prop = None
        
        # collect all move proposals
        all_props = []
        for e in self.elves:
            if not e.prop is None:
                all_props.append(e.prop)
        # count proposals by position coordinate
        self.prop_counter = Counter(all_props)
        # proposed move only valid of single elf wants to move there
        self.valid_props = [p for p, c in self.prop_counter.items() if c == 1]

    
    def move(self):
        """Move elves according to proposed movement."""
        for e in self.elves:
            if e.prop in self.valid_props:
                e.y, e.x = e.prop
                e.prop = None
            else:
                e.prop = None

    
    def step(self):
        """Propose, move, zero-shift and prepare for next step."""
        self.propose()
        self.move()
        self.shift_zero()
        self.grid = self.make_grid()
        self.dir_idx += 1
        if self.dir_idx == len(self.dirs):
            self.dir_idx = 0  
    

    def count_free(self):
        """Count free positions in crater."""
        count = 0
        for r in self.grid:
            for c in r:
                if c == ".":
                    count += 1
        return count
    

    def __repr__(self):
        #min_y, max_y, min_x, max_x = self.coords_range()
        res = ""
        for r in self.grid:
            res += ''.join(r) + "\n"
        return res


if __name__ == '__main__':
    with open("input", "r") as f:
        content = f.read()

    # initialize elves
    elves = []
    for y, r in enumerate(content.split("\n")):
        for x, c in enumerate(list(r)):
            if c == "#":
                elves.append(Elf(y,x))

    crater = Crater(elves)
    for _ in range(10):
        crater.step()

    print(crater.count_free())
