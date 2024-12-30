import { PriorityQueue } from "./queue.js";

export type Position = [number, number, Dir];

export enum Dir {
  N,
  E,
  S,
  W,
}

function adjacent(curr: Position, grid: string[][]): Position[] {
  let adj: Position[] = [];
  let [x, y, dir] = curr;
  switch (dir) {
    case Dir.N:
      adj.push([x, y, Dir.W]);
      adj.push([x, y, Dir.E]);
      y -= 1;
      break;
    case Dir.E:
      adj.push([x, y, Dir.N]);
      adj.push([x, y, Dir.S]);
      x += 1;
      break;
    case Dir.S:
      adj.push([x, y, Dir.E]);
      adj.push([x, y, Dir.W]);
      y += 1;
      break;
    case Dir.W:
      adj.push([x, y, Dir.S]);
      adj.push([x, y, Dir.N]);
      x -= 1;
      break;
    default:
      throw "Unknown direction!";
  }
  if (grid[y][x] !== "#") adj.push([x, y, dir]);
  return adj;
}

export function findLowestCost(
  start: Position,
  goal: Position,
  grid: string[][],
): number | undefined {
  let front = new PriorityQueue<{ pos: Position; priority: number }>();
  let cameFrom = new Map<string, string | null>();
  let cost = new Map<string, number>();
  front.enqueue({ pos: start, priority: 0 });
  cameFrom.set(start.toString(), null);
  cost.set(start.toString(), 0);
  debugger;
  while (!front.isEmpty) {
    let curr = front.dequeue();
    if (curr) {
      if (curr.pos[0] === goal[0] && curr.pos[1] === goal[1])
        return curr.priority;
      let nextPos = adjacent(curr.pos, grid);
      for (const next of nextPos) {
        let newCost = curr.priority;
        newCost += next[2] === curr.pos[2] ? 1 : 1000;
        let costSoFar = cost.get(next.toString());
        if (costSoFar === undefined || newCost < costSoFar) {
          cost.set(next.toString(), newCost);
          front.enqueue({ pos: next, priority: newCost });
          cameFrom.set(next.toString(), curr.pos.toString());
        }
      }
    }
  }
  return undefined;
}
