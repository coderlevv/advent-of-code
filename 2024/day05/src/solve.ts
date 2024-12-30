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

  let rules = new Map<number, number[]>();
  let updates: number[][] = [];
  for (const line of lines) {
    if (line.includes("|")) {
      let [k, v] = line.split("|").map(Number);
      if (!rules.has(k)) rules.set(k, new Array<number>());
      rules.get(k)!.push(v);
      continue;
    }
    if (line.includes(",")) {
      updates.push(line.split(",").map(Number));
      continue;
    }
  }

  let midValid: number[] = [];
  let midInvalid: number[] = [];
  for (const update of updates) {
    let valid: boolean = true;
    for (let i = 0; i < update.length; i++) {
      const after = update.slice(i + 1);
      if (after.some((a) => rules.get(a)?.includes(update[i]))) {
        valid = false;
        break;
      }
    }
    const idx = Math.floor(update.length / 2);
    if (valid) {
      midValid.push(update[idx]);
    } else {
      const sorted = update.sort((a, b) => {
        if (rules.get(a)?.includes(b)) return -1;
        return 1;
      });
      midInvalid.push(sorted[idx]);
    }
  }
  return [midValid.reduce((a, b) => a + b), midInvalid.reduce((a, b) => a + b)];
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
