import * as fs from "fs";
import { Dir, Guard } from "./guard.js";

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
  const grid: string[][] = [];
  let guard: Guard | null = null;
  lines.forEach((line, y) => {
    const row: string[] = [];
    for (let x = 0; x < line.length; x++) {
      row.push(line[x]);
      switch (line[x]) {
        case "^":
          guard = new Guard(x, y, Dir.N);
          break;
        case ">":
          guard = new Guard(x, y, Dir.E);
          break;
        case "v":
          guard = new Guard(x, y, Dir.S);
          break;
        case "<":
          guard = new Guard(x, y, Dir.W);
          break;
      }
    }
    grid.push(row);
  });

  let steps = new Set<string>();
  let init: { x: number; y: number; dir: Dir } | null = null;
  if (guard !== null) {
    init = {
      x: guard.x,
      y: guard.y,
      dir: guard.dir,
    };
    grid[guard.y][guard.x] = ".";
    while (guard.step(grid)) {
      steps.add(`${guard.x},${guard.y}`);
    }
  }

  let count = 0;
  for (const step of steps) {
    let slow: Guard = new Guard(init.x, init.y, init.dir);
    let fast: Guard = new Guard(init.x, init.y, init.dir);
    let [x, y] = step.split(",").map(Number);
    grid[y][x] = "#";
    let cycle: boolean = false;
    let snp: [number, number, Dir] | undefined = undefined;
    let fnp: [number, number, Dir] | undefined = undefined;
    do {
      debugger;
      snp = slow.step(grid);
      fnp = fast.step(grid);
      if (fnp) fnp = fast.step(grid);
      if (
        fnp &&
        snp &&
        fnp[0] === snp[0] &&
        fnp[1] === snp[1] &&
        fnp[2] === snp[2]
      )
        cycle = true;
    } while (!cycle && fnp);
    if (cycle) count++;
    grid[y][x] = ".";
  }
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
