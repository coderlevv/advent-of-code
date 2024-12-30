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

function countQuadrants(
  robots: Robot[],
  xDim: number,
  yDim: number,
  print: boolean = false,
): [number, number, number, number] {
  const xMid = Math.floor(xDim / 2);
  const yMid = Math.floor(yDim / 2);
  debugger;
  const counter = (x: number, y: number) =>
    robots.filter((r) => r.px === x && r.py === y).length;
  let q1 = 0,
    q2 = 0,
    q3 = 0,
    q4 = 0;
  for (let y = 0; y < yMid; y++)
    for (let x = 0; x < xMid; x++) q1 += counter(x, y);
  for (let y = 0; y < yMid; y++)
    for (let x = xMid + 1; x < xDim; x++) q2 += counter(x, y);
  for (let y = yMid + 1; y < yDim; y++)
    for (let x = 0; x < xMid; x++) q3 += counter(x, y);
  for (let y = yMid + 1; y < yDim; y++)
    for (let x = xMid + 1; x < xDim; x++) q4 += counter(x, y);

  if (print) {
    for (let y = 0; y < yDim; y++) {
      let row: string[] = [];
      for (let x = 0; x < xDim; x++) {
        let c = counter(x, y);
        console.log(c === 0 ? "." : c);
      }
      console.log();
    }
    console.log();
  }

  return [q1, q2, q3, q4];
}

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

  let xDim = 101;
  let yDim = 103;
  let time = 100;
  while (time > 0) {
    for (const robot of robots) {
      robot.move(xDim, yDim);
    }
    time--;
  }

  return countQuadrants(robots, xDim, yDim).reduce((a, b) => a * b, 1);
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
