import numpy as np

with open("input", "r") as f:
    content = f.read()

grid = []
empty_found = False
move_str = ""
for c in content.split('\n'):
    if c.strip() == "":
        empty_found = True
        continue
    if not empty_found:
        row = []
        for r in list(c):
            if r == " ":
                row.append(0)
            if r == ".":
                row.append(1)
            if r == "#":
                row.append(9)
        grid.append(row)
    else:
        move_str = c.strip()


class Cursor:
    def __init__(self, y, x):
        self.y = y
        self.x = x
        #Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^).
        self.face = 0
        
    def __repr__(self):
        if self.face == 0:
            face = "R"
        elif self.face == 1:
            face = "D"
        elif self.face == 2:
            face = "L"
        elif self.face == 3:
            face = "U"
        return f"({self.y},{self.x},{face})"


class Board:
    def __init__(self, grid, move_str):
        self.grid = grid
        # find start pos
        x = 0
        while (grid[0][x] != 1):
            x += 1
        self.cur = Cursor(0, x)
        #self.move_str = move_str
        self.move = self.move_gen(move_str)
    
    def move_gen(self, move_str):
        k1 = 0
        while not move_str[k1] in ['R', 'L']:
            k1 += 1
        move = move_str[0:k1]
        yield ('', int(move))
        move = ""
        k2 = k1 + 1
        done = False
        while not done:
            while k2 < len(move_str) and not move_str[k2] in ['R', 'L']:
                k2 += 1
            yield (move_str[k1:k1+1], move_str[k1+1:k2])
            k1 = k2
            k2 = k1 + 1
            if k2 >= len(move_str):
                done = True
    
    def move_right(self, num_steps):
        y = self.cur.y
        x = self.cur.x
        for _ in range(num_steps):
            void = False
            if x+1 < len(self.grid[y]) and self.grid[y][x+1] == 0:
                void = True
            # off of the grid?
            if x+1 == len(self.grid[y]) or void:
                k = 0
                while self.grid[y][k] == 0:
                    k += 1
                # is wrap pos a wall?
                if self.grid[y][k] == 9:
                    break
                # if not, new pos
                x = k
                continue
            # Wall to the right?
            elif self.grid[y][x+1] == 9:
                break
            x += 1
        # set new pos
        self.cur.x = x
        
    def move_left(self, num_steps):
        y = self.cur.y
        x = self.cur.x
        for _ in range(num_steps):
            void = False
            if x-1 >= 0 and self.grid[y][x-1] == 0:
                void = True
            # off of the grid?
            if x-1 < 0 or void:
                k = len(grid[y])-1
                while self.grid[y][k] == 0:
                    k -= 1
                # is wrap pos a wall?
                if self.grid[y][k] == 9:
                    break
                # if not, new pos
                x = k
                continue
            # Wall to the right?
            elif self.grid[y][x-1] == 9:
                break
            x -= 1
        # set new pos
        self.cur.x = x
        
    def move_down(self, num_steps):
        y = self.cur.y
        x = self.cur.x
        for _ in range(num_steps):
            void = False
            if (y+1 < len(self.grid) and x < len(self.grid[y+1]) and self.grid[y+1][x] == 0) \
                or (y+1 < len(self.grid) and x >= len(self.grid[y+1])):
                void = True
            # off of the grid?
            if y+1 == len(self.grid) or void:
                k = 0
                while x >= len(self.grid[k]) or self.grid[k][x] == 0:
                    k += 1
                # is wrap pos a wall?
                if self.grid[k][x] == 9:
                    break
                # if not, new pos
                y = k
                continue
            # Wall to the right?
            elif self.grid[y+1][x] == 9:
                break
            y += 1
        # set new pos
        self.cur.y = y
        
    def move_up(self, num_steps):
        y = self.cur.y
        x = self.cur.x
        for _ in range(num_steps):
            void = False
            if (y-1 >= 0 and x < len(grid[y-1]) and self.grid[y-1][x] == 0) \
                or (y-1 >= 0 and x >= len(grid[y-1])):
                void = True
            # off of the grid?
            if y-1 < 0 or void:
                k = len(self.grid)-1
                while x >= len(self.grid[k]) or self.grid[k][x] == 0:
                    k -= 1
                # is wrap pos a wall?
                if self.grid[k][x] == 9:
                    break
                # if not, new pos
                y = k
                continue
            # Wall to the right?
            elif self.grid[y-1][x] == 9:
                break
            y -= 1
        # set new pos
        self.cur.y = y
        
    def step(self, move):
        if move[0] == 'R':
            self.cur.face += 1
        if move[0] == 'L':
            self.cur.face -= 1
        if self.cur.face == 4:
            self.cur.face = 0
        if self.cur.face == -1:
            self.cur.face = 3
        
        num_steps = int(move[1])
        if self.cur.face == 0:
            self.move_right(num_steps)
        if self.cur.face == 1:
            self.move_down(num_steps)
        if self.cur.face == 2:
            self.move_left(num_steps)
        if self.cur.face == 3:
            self.move_up(num_steps)
            
    def walk(self):
        for m in self.move:
            self.step(m)
        return self.cur

b = Board(grid, move_str)
last_pos = b.walk()
res = (last_pos.y+1)*1000 + (last_pos.x+1)*4 + last_pos.face
print(res)
