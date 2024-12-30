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

  let seqMap = new Map<string, number>();
  for (const line of lines) {
    let gen = pseudo(Number(line));
    let nums: number[] = [];
    for (let i = 0; i < 2000; i++) {
        let n = gen.next();
        if (!n.done)
            nums.push(n.value % 10);
    }
    
    let seq: [number, number, number, number] = [0, 0, 0, 0];
    let seen = new Set<string>();
    for (let i = 0; i < nums.length - 4; i++) {
        seq[0] = nums[i+1] - nums[i];
        seq[1] = nums[i+2] - nums[i+1];
        seq[2] = nums[i+3] - nums[i+2];
        seq[3] = nums[i+4] - nums[i+3];
        debugger;
        if (seen.has(seq.toString()))
            continue;
        seen.add(seq.toString());
        let count = seqMap.get(seq.toString());
        if (count !== undefined)
            seqMap.set(seq.toString(), count + nums[i+4]);
        else seqMap.set(seq.toString(), nums[i+4]);
    }
  }
  return Math.max(...[...seqMap.values()]);
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