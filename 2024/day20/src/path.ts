import { PriorityQueue } from "./queue.js";

export type Position = [number, number];
export type Cheat = [Position];

const DISPL: Position[] = [
  [0, -1], [1, 0], [0, 1], [-1, 0] 
];

function adjacent(curr: Position, grid: string[][]): Position[] {
    let adj: Position[] = [];
    let [x, y] = curr;
    for (const [dx, dy] of DISPL) {
      let ax = x + dx;
      let ay = y + dy;
      if (ax < 0 && ax >= grid[0].length && ay < 0 && ay >= grid.length)
        continue;
      if (grid[ay][ax] !== "#") adj.push([ax, ay]); 
    }
    return adj;
  }
  
  export function findPath(start: Position, goal: Position, grid: string[][]): Position[] {
    let path: Position[] = [];
    let front = new PriorityQueue<{ pos: Position, priority: number }>();
    let cameFrom = new Map<string, string|null>();
    let cost = new Map<string, number>();
    front.enqueue({ pos: start, priority: 0 });
    cameFrom.set(`${start[0]},${start[1]}`, null);
    cost.set(`${start[0]},${start[1]}`, 0);
    while (!front.isEmpty) {
      let curr = front.dequeue();
      if (curr) {
        if (curr.pos[0] === goal[0] && curr.pos[1] === goal[1]) {
          path.push(curr.pos);
          let prev = cameFrom.get(curr.pos.toString());
          while (prev !== undefined && prev != null) {
            let [x, y] = prev.split(",").map(Number);
            path.push([x, y]);
            prev = cameFrom.get(prev);
          }
          return path.reverse();
        }
        let nextPos = adjacent(curr.pos, grid);
        for (const next of nextPos) {
          let newCost = curr.priority + 1;
          let costSoFar = cost.get(next.toString());
          if (costSoFar === undefined || newCost < costSoFar) {
            cost.set(`${next[0]},${next[1]}`, newCost);
            front.enqueue({ pos: next, priority: newCost });
            cameFrom.set(`${next[0]},${next[1]}`, `${curr.pos[0]},${curr.pos[1]}`);
          }
        }
      }
    }
    return path;
  }

export function findCheats(path: Position[], grid: string[][]): Set<string> {
  let cheats = new Set<string>();
  for (let [px, py] of path) {
    if (py-1 >= 0 && py-2 >= 0 && grid[py-1][px] === "#" && grid[py-2][px] === ".")
      cheats.add(`${px},${py-1}`);
    if (px+1 < grid[0].length && px+2 < grid[0].length && grid[py][px+1] === "#" && grid[py][px+2] === ".")
      cheats.add(`${px+1},${py}`);
    if (py+1 < grid.length && py+2 < grid.length && grid[py+1][px] === "#" && grid[py+2][px] === ".")
      cheats.add(`${px},${py+1}`);
    if (px-1 >= 0 && px-2 >= 0 && grid[py][px-1] === "#" && grid[py][px-2] === ".")
      cheats.add(`${px-1},${py}`);
  }
  return cheats;
}