type Position = { x: number; y: number };

export class Robot {
  pos: Position;
  moves: string;
  grid: string[][];

  static from(lines: string[]): Robot {
    let pos: Position | undefined = undefined;
    let grid: string[][] = [];
    let readGrid = true;
    let moves = "";
    lines.forEach((line, y) => {
      if (line.trim() === "") readGrid = false;
      if (readGrid) {
        grid.push([...line]);
        for (let x = 0; x < line.length; x++) {
          if (line[x] === "@") pos = { x: x, y: y };
        }
      } else {
        moves += line.trim();
      }
    });
    if (pos === undefined) throw "No robot position found!";
    return new Robot(pos, moves, grid);
  }

  constructor(pos: Position, moves: string, grid: string[][]) {
    this.pos = pos;
    this.moves = moves;
    this.grid = grid;
  }

  step(curr: Position, dir: string): void {
    let nx = curr.x;
    let ny = curr.y;
    switch (dir) {
      case "^":
        ny -= 1;
        break;
      case ">":
        nx += 1;
        break;
      case "v":
        ny += 1;
        break;
      case "<":
        nx -= 1;
        break;
      default:
        throw "Invalid move!";
    }
    if (this.grid[ny][nx] === "O") this.step({ x: nx, y: ny }, dir);
    if (this.grid[ny][nx] === "#") return;
    if (this.grid[ny][nx] === ".") {
      if (this.grid[curr.y][curr.x] === "@") this.pos = { x: nx, y: ny };
      this.grid[ny][nx] = this.grid[curr.y][curr.x];
      this.grid[curr.y][curr.x] = ".";
    }
  }

  move() {
    for (const m of this.moves) this.step(this.pos, m);
  }

  sumBoxes(): number {
    let sum = 0;
    this.grid.forEach((row, y) => {
      for (let x = 0; x < row.length; x++)
        if (this.grid[y][x] === "O") sum += 100 * y + x;
    });
    return sum;
  }
}
