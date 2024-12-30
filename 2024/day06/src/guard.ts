export enum Dir {
  N,
  E,
  S,
  W,
}

export class Guard {
  x: number;
  y: number;
  dir: Dir;

  constructor(x: number, y: number, dir: Dir) {
    this.x = x;
    this.y = y;
    this.dir = dir;
  }

  nextPos(grid: string[][]): { x: number; y: number; g: string } | undefined {
    let nx: number, ny: number;
    switch (this.dir) {
      case Dir.N:
        nx = this.x;
        ny = this.y - 1;
        break;
      case Dir.E:
        nx = this.x + 1;
        ny = this.y;
        break;
      case Dir.S:
        nx = this.x;
        ny = this.y + 1;
        break;
      case Dir.W:
        nx = this.x - 1;
        ny = this.y;
    }
    if (nx < 0 || nx >= grid[0].length || ny < 0 || ny >= grid.length)
      return undefined;
    else return { x: nx, y: ny, g: grid[ny][nx] };
  }

  rotate(grid: string[][]): void {
    switch (this.dir) {
      case Dir.N:
        this.dir = Dir.E;
        break;
      case Dir.E:
        this.dir = Dir.S;
        break;
      case Dir.S:
        this.dir = Dir.W;
        break;
      case Dir.W:
        this.dir = Dir.N;
    }
  }

  step(grid: string[][]): [number, number, Dir] | undefined {
    let np = this.nextPos(grid);
    if (np) {
      if (np.g !== ".") {
        do {
          this.rotate(grid);
          np = this.nextPos(grid);
        } while (np && np.g !== ".");
      }
      if (np) {
        this.x = np.x;
        this.y = np.y;
        return [this.x, this.y, this.dir];
      } else {
        return undefined;
      }
    }
    return undefined;
  }

  toString() {
    return `(${this.x},${this.y},${this.dir})`;
  }
}
