import * as fs from "fs";
import { Disk } from "./disk.js";

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
  let disk = Disk.from(lines[0]);
  disk.defrag1();
  let checksum1 = disk.checksum();
  disk = Disk.from(lines[0]);
  disk.defrag2();
  let checksum2 = disk.checksum();
  return [checksum1, checksum2];
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
