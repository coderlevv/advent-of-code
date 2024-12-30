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
  const isNotNull = (value: Guard | null): value is Guard => value !== null;
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

  if (isNotNull(guard)) {
    grid[guard.y][guard.x] = ".";
    while (guard.step(grid)) {
      steps.add(`${guard.x},${guard.y}`);
      debugger;
    }
  }
  return steps.size;
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
