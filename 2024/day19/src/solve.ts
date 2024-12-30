import * as fs from "fs";
import { fit, countFits } from "./fit.js";

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

  let patterns: string[] = [];
  let designs: string[] = [];
  for (const line of lines) {
    if (line.includes(",")) patterns = line.split(",").map((p) => p.trim());
    else if (line.trim() !== "") {
      designs.push(line.trim());
    }
  }

  let possible = 0;
  let counts: number[] = [];
  for (const design of designs) {
    if (fit(design, patterns)) possible++;
    counts.push(countFits(design, patterns));
  }
  return [possible, counts.reduce((a, b) => a + b)];
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
