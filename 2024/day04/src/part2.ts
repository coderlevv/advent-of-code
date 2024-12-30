import * as fs from "fs";

const CROSS: [number, number][] = [
  [-1, -1],
  [1, -1],
  [-1, 1],
  [1, 1],
];

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

function hasX(x: number, y: number, grid: string[][]): boolean {
  for (const [dx, dy] of CROSS) {
    const xa = x + dx;
    const ya = y + dy;
    if (xa < 0 || xa >= grid[0].length || ya < 0 || ya >= grid.length) {
      return false;
    }
  }
  if (
    ((grid[y - 1][x - 1] === "M" && grid[y + 1][x + 1] === "S") ||
      (grid[y - 1][x - 1] === "S" && grid[y + 1][x + 1] === "M")) &&
    ((grid[y - 1][x + 1] === "M" && grid[y + 1][x - 1] === "S") ||
      (grid[y - 1][x + 1] === "S" && grid[y + 1][x - 1] === "M"))
  ) {
    return true;
  }
  return false;
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
      if (grid[y][x] === "A") count += hasX(x, y, grid) ? 1 : 0;
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
