import * as fs from "fs";
import { pseudo } from "./number.js";

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

  let res: number[] = [];
  for (const line of lines) {
    let gen = pseudo(Number(line));
    for (let i = 0; i < 2000; i++) gen.next();
    let n = gen.next();
    if (!n.done)
      res.push(n.value);
  }
  return res.reduce((a, b) => a + b);
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
