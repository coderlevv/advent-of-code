import * as fs from "fs";
import { Position, findPath, findCheats } from "./path.js";

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
  lines.forEach((line, y) => {
    for (let x = 0; x < line.length; x++) {
      if (line[x] === "S") start = [x, y];
      if (line[x] === "E") goal = [x, y];
    }
    grid.push([...line]);
  });
  if (start === undefined || goal === undefined) throw "missing start or goal!";
  let path = findPath(start, goal, grid);
  let len = path.length - 1;
  let cheats = findCheats(path, grid);
  let saves: number[] = [];
  for (const cheat of cheats) {
    let [x, y] = cheat.split(",").map(Number);
    grid[y][x] = ".";
    path = findPath(start, goal, grid);
    saves.push(len - path.length + 1);
    grid[y][x] = "#";
  }
  return saves.filter((s) => s >= 100).length;
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
