const DISPL = Array.of([0, -1], [-1, 0], [1, 0], [0, 1]);

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

export function perimeter(coords: Position[]): number {
  let counts: number[] = [];
  for (const point of coords) {
    let count = 0;
    for (let d of DISPL) {
      let [dx, dy] = d;
      let adjX = point[0] + dx;
      let adjY = point[1] + dy;
      for (const other of coords) {
        if (other[0] === adjX && other[1] === adjY) count++;
      }
    }
    counts.push(4 - count);
  }
  return counts.reduce((a, b) => a + b);
}

export function adjacent(
  pos: Position,
  id: string,
  grid: string[][],
): Position[] {
  let adj: Position[] = [];
  let [x, y] = pos;
  for (let d of DISPL) {
    let [dx, dy] = d;
    let adjX = x + dx;
    let adjY = y + dy;
    if (adjX >= 0 && adjX < grid[0].length && adjY >= 0 && adjY < grid.length) {
      if (grid[adjY][adjX] === id) {
        adj.push([adjX, adjY]);
      }
    }
  }
  return adj;
}

export function area(pos: Position, id: string, grid: string[][]): Position[] {
  let front = new Queue<Position>();
  let area: Position[] = [];
  let visited = new Set<string>();
  visited.add(pos.toString());
  front.enqueue(pos);
  while (!front.isEmpty) {
    let curr = front.dequeue();
    if (curr) {
      area.push(curr);
      let adj = adjacent(curr, id, grid);
      for (let next of adj) {
        if (!visited.has(next.toString())) {
          visited.add(next.toString());
          front.enqueue(next);
        }
      }
    }
  }
  return area;
}

function leftTurn(p: Position, q: Position, r: Position): boolean {
  return (q[1] - p[1]) * (r[0] - q[0]) > (q[0] - p[0]) * (r[1] - q[1]);
}

export function convexHull(points: Position[]): Position[] {
  let hull: Position[] = [];
  if (points.length < 3) return hull;

  let leftmost = points[0];
  for (const point of points) {
    if (point[0] < leftmost[0]) {
      leftmost = point;
    }
  }

  let current = leftmost;
  do {
    hull.push(current);
    let next = points[0];
    for (const point of points) {
      if (
        (next[0] === current[0] && next[1] === current[1]) ||
        leftTurn(current, next, point)
      ) {
        next = point;
      }
    }
    current = next;
  } while (current[0] !== leftmost[0] || current[1] !== leftmost[1]);
  return hull;
}
