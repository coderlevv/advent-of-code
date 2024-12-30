import * as fs from "fs";
import * as PF from "pathfinding";

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

  type Location = { x: number; y: number };
  const bytes: Location[] = [];
  for (const line of lines) {
    let m = /(\d+),(\d+)/.exec(line);
    if (m) {
      let [, x, y] = m.map(Number);
      bytes.push({ x: x, y: y });
    }
  }

  let grid = new PF.Grid(71, 71);
  for (let i = 0; i < bytes.length; i++) {
    if (i >= 1024) break;
    grid.setWalkableAt(bytes[i].x, bytes[i].y, false);
  }
  const finder = new PF.AStarFinder();
  let path = finder.findPath(0, 0, 70, 70, grid.clone());
  let pathLen = path.length;

  let i = 1024;
  let res: string | undefined = undefined;
  while (true) {
    grid.setWalkableAt(bytes[i].x, bytes[i].y, false);
    path = finder.findPath(0, 0, 70, 70, grid.clone());
    if (path.length === 0) {
      res = `${bytes[i].x},${bytes[i].y}`;
      break;
    }
    i++;
  }
  return [pathLen, res];
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
