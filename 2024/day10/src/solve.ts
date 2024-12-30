import * as fs from "fs";
import { Position, isTrailHead, allTrails } from "./queue.js";

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

  const grid: number[][] = [];
  const height9: Position[] = [];
  lines.forEach((line, y) => {
    grid.push([...line].map(Number));
    for (let x = 0; x < line.length; x++)
      if (grid[y][x] === 9) height9.push([x, y]);
  });
  let counts1: number[] = [];
  for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[0].length; x++) {
      if (grid[y][x] === 0) {
        let count = 0;
        height9.forEach((h) => {
          if (isTrailHead([x, y], h, grid)) count++;
        });
        counts1.push(count);
      }
    }
  }
  let counts2: number[] = [];
  for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[0].length; x++) {
      if (grid[y][x] === 0) {
        let count = 0;
        height9.forEach((h) => {
          counts2.push(allTrails([x, y], h, grid));
        });
      }
    }
  }
  return [counts1.reduce((a, b) => a + b), counts2.reduce((a, b) => a + b)];
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
