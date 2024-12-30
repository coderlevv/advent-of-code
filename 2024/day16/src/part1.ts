import * as fs from "fs";
import { Position, Dir, findLowestCost } from "./path.js";

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
    lines = (await readInput("test_input")) as string[];
  } catch (err) {
    throw err;
  }

  let grid: string[][] = [];
  let start: Position | undefined = undefined;
  let goal: Position | undefined = undefined;
  lines.forEach((line, y) => {
    for (let x = 0; x < line.length; x++) {
      if (line[x] === "S") start = [x, y, Dir.E];
      if (line[x] === "E") goal = [x, y, Dir.E];
    }
    grid.push([...line]);
  });
  if (start === undefined || goal === undefined) throw "missing start or goal!";
  let minCost = findLowestCost(start, goal, grid);
  return minCost;
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
