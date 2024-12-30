import * as fs from "fs";
import { hasSolution, partition, aggregate } from "./util.js";

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

  let valid: number[] = [];
  for (const line of lines) {
    let colon = line.indexOf(":");
    let sol = Number(line.slice(0, colon));
    let nums = line
      .slice(colon + 1)
      .trim()
      .split(" ")
      .map(Number);
    if (hasSolution(nums[0], sol, nums)) valid.push(sol);
  }
  return valid.reduce((a, b) => a + b);
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