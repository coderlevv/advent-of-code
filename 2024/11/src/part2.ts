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

  let seq = lines[0].split(" ").map(Number);
  let pebb = new Map<number, number>();
  seq.forEach((s) => {
    let c = pebb.get(s);
    pebb.set(s, (c === undefined ? 0 : c) + 1);
  });
  let blinks = 75;
  while (blinks > 0) {
    let newPebb = new Map<number, number>();
    for (const [k, v] of pebb.entries()) {
      if (k === 0) {
        let c = newPebb.get(1);
        newPebb.set(1, (c === undefined ? 0 : c) + 1 * v);
      } else if (k.toString().length % 2 === 0) {
        let mid = k.toString().length / 2;
        let h1 = Number(k.toString().slice(0, mid));
        let c1 = newPebb.get(h1);
        newPebb.set(h1, (c1 === undefined ? 0 : c1) + 1 * v);
        let h2 = Number(k.toString().slice(mid));
        let c2 = newPebb.get(h2);
        newPebb.set(h2, (c2 === undefined ? 0 : c2) + 1 * v);
      } else {
        let c = newPebb.get(k * 2024);
        newPebb.set(k * 2024, (c === undefined ? 0 : c) + 1 * v);
      }
    }
    blinks--;
    pebb = newPebb;
  }
  let len = 0;
  for (let c of pebb.values()) len += c;
  return len;
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
