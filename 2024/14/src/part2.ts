import * as fs from "fs";
import { Robot } from "./robot.js";

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

const counter = (x: number, y: number, robots: Robot[]) =>
  robots.filter((r) => r.px === x && r.py === y).length;

async function solve() {
  let lines: string[] = [];
  try {
    lines = (await readInput("input")) as string[];
  } catch (err) {
    throw err;
  }

  let robots: Robot[] = [];
  for (const line of lines) {
    let m = /p=(\d+),(\d+) v=(-?\d+),(-?\d+)/.exec(line);
    if (m) {
      let [, px, py, vx, vy] = [...m].map(Number);
      robots.push(new Robot(px, py, vx, vy));
    }
  }

  const output = (x0: number, x1: number, y0: number, y1: number) => {
    for (let y = y0; y < y1; y++) {
      let row: string[] = [];
      for (let x = x0; x < x1; x++) {
        let c = counter(x, y, robots);
        row.push((c === 0 ? "." : c).toString());
      }
      console.log(row.join(""));
    }
  };

  const xDim = 101;
  const yDim = 103;
  const xQuart = Math.floor(xDim / 4);
  const yQuart = Math.floor(yDim / 4);
  let time = 0;
  while (true) {
    for (const robot of robots) {
      robot.move(xDim, yDim);
    }
    time++;
    let count = 0;
    for (let y = yQuart; y < 3 * yQuart; y++)
      for (let x = xQuart; x < 3 * xQuart; x++) count += counter(x, y, robots);

    if (count > 250) {
      output(xQuart, 3 * xQuart + 1, yQuart, 3 * yQuart + 1);
      break;
    }
    if (time % 100000 == 0) console.log(time);
  }
  return time;
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
