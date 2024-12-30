import * as fs from "fs";
import { Antenna, antinodes, harmonics } from "./antenna.js";

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

  const grid: string[][] = [];
  const antennas: Antenna[] = [];
  const freqs: Set<string> = new Set();
  lines.forEach((line, y) => {
    for (let x = 0; x < lines[0].length; x++) {
      if (lines[y][x] !== ".") {
        antennas.push(new Antenna(x, y, line[x]));
        freqs.add(line[x]);
      }
    }
    grid.push([...line]);
  });

  const xdim = grid[0].length;
  const ydim = grid.length;
  let loc1: Set<string> = new Set();
  let loc2: Set<string> = new Set();
  for (const freq of freqs) {
    const inFreq = antennas.filter((a) => a.freq === freq);
    if (inFreq.length > 1) {
      for (let i = 0; i < inFreq.length; i++) {
        for (let j = 0; j < inFreq.length; j++) {
          if (i === j) continue;
          antinodes(inFreq[i], inFreq[j]).forEach((n) => {
            if (
              n[0] >= 0 &&
              n[0] < grid[0].length &&
              n[1] >= 0 &&
              n[1] < grid.length
            )
              loc1.add(`${n[0]},${n[1]}`);
          });
          harmonics(inFreq[i], inFreq[j], xdim, ydim).forEach((n) => {
            if (
              n[0] >= 0 &&
              n[0] < grid[0].length &&
              n[1] >= 0 &&
              n[1] < grid.length
            )
              loc2.add(`${n[0]},${n[1]}`);
          });
        }
      }
    }
  }
  return [loc1.size, loc2.size];
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
