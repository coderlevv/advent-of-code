// part2 inspired by https://github.com/GNUSheep/AdventOfCode/tree/main/2024/day20
import * as fs from "fs";
import { Position } from "./path.js";

const readInput = (inputFile: string) => {
  return new Promise((resolve, reject) => {
    fs.readFile(inputFile, (err, data) => {
      if (err) {
        reject(err);
      } else {
        const lines = data.toString().trim().split("\n");
        resolve(lines);
      }
    });
  });
};

async function solve() {
  let lines: string[] = [];
  try {
    lines = (await readInput("input")) as string[];
  } catch (err) {
    throw err;
  }
  let grid: string[][] = [];
  let start: Position | undefined = undefined;
  let goal: Position | undefined = undefined;
  let pathTime = 0;
  lines.forEach((line, y) => {
    for (let x = 0; x < line.length; x++) {
      if (line[x] === "S") start = [x, y];
      if (line[x] === "E") goal = [x, y];
      if (line[x] === ".") pathTime += 1;
    }
    grid.push([...line]);
  });
  if (start === undefined || goal === undefined) throw "Missing start or goal!";
  const seen = new Set<string>();
  const dist: [number, number, number][] = [[start[0], start[1], pathTime]];
  const dmap = new Map<Position, number>();
  dmap.set([start[0], start[1]], pathTime);
  debugger;
  while (true) {
    const [x, y, t] = dist[dist.length-1];
    if (grid[y][x] === "E")
      break;
    seen.add(`${x},${y}`);
    for (const [nx, ny] of [[x+1, y], [x-1, y], [x,y+1],[x,y-1]]) {
      if (grid[ny][nx] === "#" || seen.has(`${nx},${ny}`)) continue;
      dist.push([nx, ny, t-1]);
      dmap.set([nx, ny], t-1);
    }
  }
  
  seen.clear();
  const cheatMap = new Map<number, number>();
  const queue = dist.reverse();
  while(queue.length > 0) {
    let curr = queue.pop();
    if (curr) {
      let [x, y, t] = curr;
      seen.add(`${x},${y}`);
      for (const [xc, yc, tc] of dist) {
        if (seen.has(`${xc},${yc}`)) continue;
        let md = Math.abs(x - xc) + Math.abs(y - yc);
        if (md >= 2 && md <= 20) {
          let c = cheatMap.get(t - md - tc);
          cheatMap.set(t - md - tc, c === undefined ? 1 : c + 1);
        }
      }
    }
  }
  let count = 0;
  for (const [k, v] of cheatMap.entries())
    if (k >= 100)
      count += v;

  return count;
}

solve()
  .then(console.log)
  .catch((err) => {
    if ((err as NodeJS.ErrnoException).code === "ENOENT") {
      console.error("Please provide input file!");
    } else {
      console.error("An unexpected error occurred:", err);
    }
  });
