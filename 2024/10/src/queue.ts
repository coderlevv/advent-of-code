export type Position = [number, number];

export class Queue<T> {
  private items: T[];

  constructor() {
    this.items = new Array();
  }

  get size(): number {
    return this.items.length;
  }

  get isEmpty(): boolean {
    return this.size === 0;
  }

  enqueue(item: T): void {
    this.items.push(item);
  }

  dequeue(): T | undefined {
    return !this.isEmpty ? this.items.shift() : undefined;
  }
}

const DISPL = Array.of([0, -1], [-1, 0], [1, 0], [0, 1]);

export function adjacent(pos: Position, grid: number[][]): Position[] {
  let adj: Position[] = [];
  let [x, y] = pos;
  for (let d of DISPL) {
    let [dx, dy] = d;
    let adjX = x + dx;
    let adjY = y + dy;
    if (adjX >= 0 && adjX < grid[0].length && adjY >= 0 && adjY < grid.length) {
      let h = grid[adjY][adjX];
      if (h - grid[y][x] === 1) {
        adj.push([adjX, adjY]);
      }
    }
  }
  // return an array of Positions
  return adj;
  pos;
}

export function isTrailHead(
  pos: Position,
  goal: Position,
  grid: number[][],
): boolean {
  let front = new Queue<Position>();
  front.enqueue(pos);
  let cameFrom = new Map<string, string | null>();
  cameFrom.set(pos.toString(), null);
  while (!front.isEmpty) {
    let curr = front.dequeue();
    if (curr) {
      if (curr[0] === goal[0] && curr[1] === goal[1]) return true;
      let [currX, currY] = curr;
      let adj = adjacent(curr, grid);
      for (let next of adj) {
        if (!cameFrom.has(next.toString())) {
          front.enqueue(next);
          cameFrom.set(next.toString(), pos.toString());
        }
      }
    }
  }
  return false;
}

export function allTrails(
  pos: Position,
  goal: Position,
  grid: number[][],
): number {
  let front = new Queue<Position>();
  front.enqueue(pos);
  //let cameFrom = new Map<string, string | null>();
  //cameFrom.set(pos.toString(), null);
  let trials = 0;
  while (!front.isEmpty) {
    let curr = front.dequeue();
    if (curr) {
      if (curr[0] === goal[0] && curr[1] === goal[1]) trials++;
      let [currX, currY] = curr;
      let adj = adjacent(curr, grid);
      for (let next of adj) {
        front.enqueue(next);
        //cameFrom.set(next.toString(), pos.toString());
      }
    }
  }
  return trials;
}
