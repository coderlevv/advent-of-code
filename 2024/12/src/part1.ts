import * as fs from "fs";
import { Position, area, perimeter } from "./plot.js";

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
  lines.forEach((line, y) => {
    grid.push([...line]);
  });

  let plots = new Set<string>();
  let prices: number[] = [];
  for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[0].length; x++) {
      if (!plots.has(`${x},${y}`)) {
        let coords = area([x, y], grid[y][x], grid);
        prices.push(coords.length * perimeter(coords));
        coords.forEach((c) => plots.add(`${c[0]},${c[1]}`));
        debugger;
      }
    }
  }

  return prices.reduce((a, b) => a + b);
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
