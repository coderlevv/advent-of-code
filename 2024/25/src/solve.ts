import * as fs from "fs";

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

  let keys: number[][] = [];
  let locks: number[][] = [];
  let isLock: boolean | undefined = undefined;
  let pins: string[] = [];
  let counts: number[] = [0, 0, 0, 0, 0];
  for (const line of lines) {
    pins = [...line];
    if (pins.length === 0) {
      if (!isLock) keys.push([...counts]);
      else {
        locks.push([...counts].map((c) => (c -= 1)));
      }
      isLock = undefined;
      pins = [];
      counts = [0, 0, 0, 0, 0];
      continue;
    }
    if (isLock === undefined) {
      isLock = pins.every((p) => p === ".");
      continue;
    }
    pins.forEach((pin, idx) => (counts[idx] += pin === "#" ? 1 : 0));
  }
  if (isLock !== undefined) {
    if (!isLock) keys.push([...counts]);
    else {
      locks.push([...counts].map((c) => (c -= 1)));
    }
  }

  let fit = 0;
  for (const key of keys) {
    for (const lock of locks) {
      counts = [0, 0, 0, 0, 0];
      for (let idx = 0; idx < key.length; idx++)
        counts[idx] = key[idx] + lock[idx];
      fit += counts.every((c) => c <= 5) ? 1 : 0;
    }
  }
  return fit;
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
