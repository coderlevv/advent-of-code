import * as fs from "fs";

const DIR: [number, number][] = [
  [-1, -1],
  [0, -1],
  [1, -1],
  [-1, 0],
  [1, 0],
  [-1, 1],
  [0, 1],
  [1, 1],
];
const WORD: string[] = ["X", "M", "A", "S"];

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

function search(x: number, y: number, grid: string[][]): number {
  let count = 0;
  for (const [dx, dy] of DIR) {
    let found: boolean = true;
    for (let idx = 1; idx < WORD.length; idx++) {
      const xa = x + idx * dx;
      const ya = y + idx * dy;
      if (
        xa < 0 ||
        xa >= grid[0].length ||
        ya < 0 ||
        ya >= grid.length ||
        grid[ya][xa] !== WORD[idx]
      ) {
        found = false;
        break;
      }
    }
    if (found) count++;
  }
  return count;
}

async function solve() {
  let lines: string[] = [];
  try {
    lines = (await readInput("input")) as string[];
  } catch (err) {
    throw err;
  }

  let grid: string[][] = [];
  lines.forEach((line) => grid.push([...line]));
  let count = 0;
  for (let y = 0; y < grid.length; y++)
    for (let x = 0; x < grid[0].length; x++)
      if (grid[y][x] === WORD[0]) count += search(x, y, grid);
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
